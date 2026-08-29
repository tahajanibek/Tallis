use candle_core::{Module, Result, Tensor};
use candle_nn::{Conv2d, LayerNorm, Linear, VarBuilder, conv2d, layer_norm, linear};

pub struct Dreadeon {
    projeksiyon: Conv2d,
}

impl Dreadeon {
    pub fn yeni(vb: VarBuilder) -> Result<Self> {
        let proj_cfg = candle_nn::Conv2dConfig {
            stride: 16,
            ..Default::default()
        };
        let projeksiyon = conv2d(3, 768, 16, proj_cfg, vb.pp("proj"))?;

        Ok(Self { projeksiyon })
    }
}

impl Module for Dreadeon {
    fn forward(&self, x: &Tensor) -> Result<Tensor> {
        self.projeksiyon.forward(x)
    }
}

pub struct Saray {
    mezarci: Dreadeon,
    anorm: LayerNorm,
}

impl Saray {
    pub fn yeni(vb: VarBuilder) -> Result<Self> {
        let mezarci = Dreadeon::yeni(vb.pp("patch_embed"))?;
        let anorm = layer_norm(768, 1e-6, vb.pp("norm"))?;

        Ok(Self { mezarci, anorm })
    }
}

impl Module for Saray {
    fn forward(&self, x: &Tensor) -> Result<Tensor> {
        let dirilme = self.mezarci.forward(x)?;

        let (b, c, h, w) = dirilme.dims4()?;
        let sekillendirilmis = dirilme.reshape((b, c, h * w))?.transpose(1, 2)?;

        self.anorm.forward(&sekillendirilmis)
    }
}

pub struct Goz {
    qkv: Linear,
    proj: Linear,
    kafa_sayisi: usize,
    kafa_boyutu: usize,
}

impl Goz {
    pub fn yeni(vb: VarBuilder, boyut: usize, kafa_sayisi: usize) -> Result<Self> {
        let kafa_boyutu = boyut / kafa_sayisi;
        let qkv = linear(boyut, boyut * 3, vb.pp("qkv"))?;
        let proj = linear(boyut, boyut, vb.pp("proj"))?;

        Ok(Self {
            qkv,
            proj,
            kafa_sayisi,
            kafa_boyutu,
        })
    }
}

impl Module for Goz {
    fn forward(&self, x: &Tensor) -> Result<Tensor> {
        let (b, n, c) = x.dims3()?;

        let qkv = self.qkv.forward(x)?;
        let qkv = qkv.reshape((b, n, 3, self.kafa_sayisi, self.kafa_boyutu))?;

        let q = qkv
            .narrow(2, 0, 1)?
            .squeeze(2)?
            .transpose(1, 2)?
            .contiguous()?;
        let k = qkv
            .narrow(2, 1, 1)?
            .squeeze(2)?
            .transpose(1, 2)?
            .contiguous()?;
        let v = qkv
            .narrow(2, 2, 1)?
            .squeeze(2)?
            .transpose(1, 2)?
            .contiguous()?;

        let scale = (self.kafa_boyutu as f64).sqrt();
        let dikkat_skoru = (q.matmul(&k.transpose(2, 3)?)? / scale)?;
        let dikkat_agirligi = candle_nn::ops::softmax(&dikkat_skoru, candle_core::D::Minus1)?;

        let baglam = dikkat_agirligi.matmul(&v)?;
        let baglam = baglam.transpose(1, 2)?.reshape((b, n, c))?;

        self.proj.forward(&baglam)
    }
}

pub struct Zihin {
    fc1: Linear,
    fc2: Linear,
}

impl Zihin {
    pub fn yeni(vb: VarBuilder, boyut: usize, genisleme_carpani: usize) -> Result<Self> {
        let gizli_boyut = boyut * genisleme_carpani;

        let fc1 = linear(boyut, gizli_boyut, vb.pp("fc1"))?;
        let fc2 = linear(gizli_boyut, boyut, vb.pp("fc2"))?;

        Ok(Self { fc1, fc2 })
    }
}

impl Module for Zihin {
    fn forward(&self, x: &Tensor) -> Result<Tensor> {
        let x = self.fc1.forward(x)?;
        let x = x.gelu()?;
        self.fc2.forward(&x)
    }
}

pub struct Dugum {
    norm1: LayerNorm,
    goz: Goz,
    norm2: LayerNorm,
    zihin: Zihin,
}

impl Dugum {
    pub fn yeni(vb: VarBuilder, boyut: usize, kafa_sayisi: usize) -> Result<Self> {
        let norm1 = layer_norm(boyut, 1e-6, vb.pp("norm1"))?;
        let goz = Goz::yeni(vb.pp("attn"), boyut, kafa_sayisi)?;

        let norm2 = layer_norm(boyut, 1e-6, vb.pp("norm2"))?;
        let zihin = Zihin::yeni(vb.pp("mlp"), boyut, 4)?;

        Ok(Self {
            norm1,
            goz,
            norm2,
            zihin,
        })
    }
}

impl Module for Dugum {
    fn forward(&self, x: &Tensor) -> Result<Tensor> {
        let x_norm1 = self.norm1.forward(x)?;
        let goz_ciktisi = self.goz.forward(&x_norm1)?;
        let x = x.broadcast_add(&goz_ciktisi)?;

        let x_norm2 = self.norm2.forward(&x)?;
        let zihin_ciktisi = self.zihin.forward(&x_norm2)?;
        let x = x.broadcast_add(&zihin_ciktisi)?;

        Ok(x)
    }
}

pub struct Kahin {
    saray: Saray,
    dugumler: Vec<Dugum>,
    son_norm: LayerNorm,
}

impl Kahin {
    pub fn yeni(
        vb: VarBuilder,
        boyut: usize,
        kafa_sayisi: usize,
        dugum_sayisi: usize,
    ) -> Result<Self> {
        let saray = Saray::yeni(vb.clone())?;

        let mut dugumler = Vec::with_capacity(dugum_sayisi);
        let blocks_vb = vb.pp("blocks");

        for i in 0..dugum_sayisi {
            let dugum = Dugum::yeni(blocks_vb.pp(i.to_string()), boyut, kafa_sayisi)?;
            dugumler.push(dugum);
        }

        let son_norm = layer_norm(boyut, 1e-6, vb.pp("norm_post"))?;

        Ok(Self {
            saray,
            dugumler,
            son_norm,
        })
    }
}

impl Module for Kahin {
    fn forward(&self, x: &Tensor) -> Result<Tensor> {
        let mut x = self.saray.forward(x)?;

        for dugum in &self.dugumler {
            x = dugum.forward(&x)?;
        }

        self.son_norm.forward(&x)
    }
}

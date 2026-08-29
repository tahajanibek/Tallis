use candle_core::{Module, Result, Tensor};
use candle_nn::{Linear, VarBuilder, linear};
use std::sync::Mutex;

pub struct IbnHazmAgabey {
    qkv: Linear,
    proj: Linear,
    kafa_sayisi: usize,
    kafa_boyutu: usize,
    kv_cache: Mutex<Option<(Tensor, Tensor)>>,
}

impl IbnHazmAgabey {
    pub fn yeni(vb: VarBuilder, boyut: usize, kafa_sayisi: usize) -> Result<Self> {
        let kafa_boyutu = boyut / kafa_sayisi;
        let qkv = linear(boyut, boyut * 3, vb.pp("qkv"))?;
        let proj = linear(boyut, boyut, vb.pp("proj"))?;

        Ok(Self {
            qkv,
            proj,
            kafa_sayisi,
            kafa_boyutu,
            kv_cache: Mutex::new(None),
        })
    }

    pub fn onbellegi_temizle(&self) {
        if let Ok(mut cache) = self.kv_cache.lock() {
            *cache = None;
        }
    }
}

impl Module for IbnHazmAgabey {
    fn forward(&self, x: &Tensor) -> Result<Tensor> {
        let (b, n, c) = x.dims3()?;

        let qkv = self.qkv.forward(x)?;
        let qkv = qkv.reshape((b, n, 3, self.kafa_sayisi, self.kafa_boyutu))?;

        let q = qkv
            .narrow(2, 0, 1)?
            .squeeze(2)?
            .transpose(1, 2)?
            .contiguous()?;
        let mut k = qkv
            .narrow(2, 1, 1)?
            .squeeze(2)?
            .transpose(1, 2)?
            .contiguous()?;
        let mut v = qkv
            .narrow(2, 2, 1)?
            .squeeze(2)?
            .transpose(1, 2)?
            .contiguous()?;

        {
            let mut cache = self.kv_cache.lock().unwrap();
            if let Some((gecmis_k, gecmis_v)) = cache.as_ref() {
                k = Tensor::cat(&[gecmis_k, &k], 2)?;
                v = Tensor::cat(&[gecmis_v, &v], 2)?;
            }
            *cache = Some((k.clone(), v.clone()));
        }
        let scale = (self.kafa_boyutu as f64).sqrt();

        let dikkat_skoru = (q.matmul(&k.transpose(2, 3)?)? * (1.0 / scale))?;

        let dikkat_agirligi = candle_nn::ops::softmax(&dikkat_skoru, candle_core::D::Minus1)?;
        let baglam = dikkat_agirligi.matmul(&v)?;
        let baglam = baglam.transpose(1, 2)?.reshape((b, n, c))?;

        self.proj.forward(&baglam)
    }
}

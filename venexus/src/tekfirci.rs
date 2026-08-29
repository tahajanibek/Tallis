use crate::ibnhazm::IbnHazmAgabey;
use candle_core::{Module, Result, Tensor};
use candle_nn::{LayerNorm, Linear, VarBuilder, linear};

pub struct Tekfirsel {
    self_attn: IbnHazmAgabey,
    mlp_fc1: Linear,
    mlp_fc2: Linear,
    norm1: LayerNorm,
    norm2: LayerNorm,
}

impl Tekfirsel {
    pub fn yeni(vb: VarBuilder, boyut: usize, kafa_sayisi: usize) -> Result<Self> {
        let self_attn = IbnHazmAgabey::yeni(vb.pp("self_attn"), boyut, kafa_sayisi)?;
        let mlp_fc1 = linear(boyut, boyut * 4, vb.pp("mlp.gate_proj"))?;
        let mlp_fc2 = linear(boyut * 4, boyut, vb.pp("mlp.down_proj"))?;

        let norm1 = candle_nn::layer_norm(boyut, 1e-6, vb.pp("input_layernorm"))?;
        let norm2 = candle_nn::layer_norm(boyut, 1e-6, vb.pp("post_attention_layernorm"))?;

        Ok(Self {
            self_attn,
            mlp_fc1,
            mlp_fc2,
            norm1,
            norm2,
        })
    }

    pub fn onbellegi_temizle(&self) {
        self.self_attn.onbellegi_temizle();
    }
}

impl Module for Tekfirsel {
    fn forward(&self, x: &Tensor) -> Result<Tensor> {
        let norm_x = self.norm1.forward(x)?;
        let attn_out = self.self_attn.forward(&norm_x)?;
        let x = x.broadcast_add(&attn_out)?;

        let norm_x2 = self.norm2.forward(&x)?;
        let mlp_int = candle_nn::ops::silu(&self.mlp_fc1.forward(&norm_x2)?)?;
        let mlp_out = self.mlp_fc2.forward(&mlp_int)?;
        let x = x.broadcast_add(&mlp_out)?;

        Ok(x)
    }
}

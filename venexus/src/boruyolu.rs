use crate::kodex::Kahin;
use crate::tekfirci::Tekfirsel;
use anyhow::{Context, Result};
use candle_core::IndexOp;
use candle_core::{Device, Module, Tensor};
use candle_nn::{Embedding, LayerNorm, Linear};
use image::DynamicImage;
use std::path::Path;
use tokenizers::Tokenizer;

pub struct VenexusBoruHatti {
    kahin: Kahin,
    tekfirsel_katmanlari: Vec<Tekfirsel>,
    word_embeddings: Embedding,
    son_norm: LayerNorm,
    lm_head: Linear,
    tokenizer: Tokenizer,
    cihaz: Device,
}

impl VenexusBoruHatti {
    pub fn yeni(
        kahin: Kahin,
        tekfirsel_katmanlari: Vec<Tekfirsel>,
        word_embeddings: Embedding,
        son_norm: LayerNorm,
        lm_head: Linear,
        tokenizer_dosyasi: &Path,
        cihaz: Device,
    ) -> Result<Self> {
        let tokenizer = Tokenizer::from_file(tokenizer_dosyasi)
            .map_err(|e| anyhow::anyhow!("error_tokenizer_load_failed: {}", e))?;

        Ok(Self {
            kahin,
            tekfirsel_katmanlari,
            word_embeddings,
            son_norm,
            lm_head,
            tokenizer,
            cihaz,
        })
    }

    pub fn gorseli_isle(&self, gorsel: &DynamicImage) -> Result<Tensor> {
        let yeniden_boyutlandirilmis =
            gorsel.resize_exact(1024, 1024, image::imageops::FilterType::Lanczos3);

        let rgb_gorsel = yeniden_boyutlandirilmis.to_rgb8();
        let (genislik, yukseklik) = rgb_gorsel.dimensions();
        let piksel_verileri = rgb_gorsel.into_raw();

        let tensor = Tensor::from_vec(
            piksel_verileri,
            (1, yukseklik as usize, genislik as usize, 3),
            &self.cihaz,
        )?
        .permute((0, 3, 1, 2))?
        .to_dtype(candle_core::DType::F32)?
        .affine(1.0 / 255.0, 0.0)?;

        self.kahin
            .forward(&tensor)
            .context("error_vision_encoder_forward_failed")
    }

    pub fn metin_uret(&self, gorsel: &DynamicImage) -> Result<String> {
        for katman in &self.tekfirsel_katmanlari {
            katman.onbellegi_temizle();
        }

        let _gorsel_vektorleri = self.gorseli_isle(gorsel)?;

        let mut uretilen_tokenlar: Vec<u32> = vec![1];
        let max_uzunluk = 64;

        for _ in 0..max_uzunluk {
            let son_token = *uretilen_tokenlar.last().unwrap();
            let girdi_tensoru = Tensor::new(&[son_token], &self.cihaz)?.unsqueeze(0)?;

            let mut x = self.word_embeddings.forward(&girdi_tensoru)?;

            for katman in &self.tekfirsel_katmanlari {
                x = katman.forward(&x)?;
            }

            x = self.son_norm.forward(&x)?;

            let logits = self.lm_head.forward(&x)?;

            let sonraki_token = logits.i((0, 0))?.argmax(0)?.to_scalar::<u32>()?;

            if sonraki_token == 2 || sonraki_token == 151643 {
                break;
            }
            uretilen_tokenlar.push(sonraki_token);
        }

        let sonuc_metni = self
            .tokenizer
            .decode(&uretilen_tokenlar, true)
            .map_err(|e| anyhow::anyhow!("error_token_decode_failed: {}", e))?;

        Ok(sonuc_metni)
    }
}

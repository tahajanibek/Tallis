use crate::boruyolu::VenexusBoruHatti;
use crate::kodex::Kahin;
use crate::tekfirci::Tekfirsel;
use anyhow::{Context, Result};
use candle_core::Device;
use candle_nn::VarBuilder;
use std::fs;
use std::path::{Path, PathBuf};

pub mod boruyolu;
pub mod ibnhazm;
pub mod kodex;
pub mod tekfirci;

pub struct Venexus {
    pub cihaz: Device,
    pub boru_hatti: VenexusBoruHatti,
}

impl Venexus {
    pub fn baslat(model_dizini: &Path) -> Result<Self> {
        let cihaz = if candle_core::utils::metal_is_available() {
            Device::new_metal(0)?
        } else if candle_core::utils::cuda_is_available() {
            Device::new_cuda(0)?
        } else {
            Device::Cpu
        };

        let mut safetensors_dosyalari: Vec<PathBuf> = Vec::new();

        if model_dizini.is_dir() {
            for girdi in fs::read_dir(model_dizini)? {
                let yol = girdi?.path();
                if yol.extension().and_then(|e| e.to_str()) == Some("safetensors") {
                    safetensors_dosyalari.push(yol);
                }
            }
        } else if model_dizini.extension().and_then(|e| e.to_str()) == Some("safetensors") {
            safetensors_dosyalari.push(model_dizini.to_path_buf());
        }

        if safetensors_dosyalari.is_empty() {
            return Err(anyhow::anyhow!("error_no_safetensors_found"));
        }

        let vb = unsafe {
            VarBuilder::from_mmaped_safetensors(
                &safetensors_dosyalari,
                candle_core::DType::F32,
                &cihaz,
            )?
        };

        let boyut = 3584;
        let kafa_sayisi = 28;
        let dugum_sayisi = 28;
        let vokab_boyutu = 151936;

        let kahin = Kahin::yeni(vb.pp("vision_model"), boyut, kafa_sayisi, dugum_sayisi)
            .context("error_kahin_init_failed")?;

        let mut tekfirsel_katmanlari = Vec::with_capacity(dugum_sayisi);
        let layers_vb = vb.pp("model.layers");
        for i in 0..dugum_sayisi {
            let katman = Tekfirsel::yeni(layers_vb.pp(i.to_string()), boyut, kafa_sayisi)
                .context("error_tekfirsel_init_failed")?;
            tekfirsel_katmanlari.push(katman);
        }

        let word_embeddings =
            candle_nn::embedding(vokab_boyutu, boyut, vb.pp("model.embed_tokens"))
                .context("error_embedding_init_failed")?;

        let son_norm = candle_nn::layer_norm(boyut, 1e-6, vb.pp("model.norm"))
            .context("error_final_norm_init_failed")?;

        let lm_head = candle_nn::linear_no_bias(boyut, vokab_boyutu, vb.pp("lm_head"))
            .context("error_lm_head_init_failed")?;

        let tokenizer_yolu = model_dizini.join("tokenizer.json");
        let tokenizer_dosyasi = if tokenizer_yolu.exists() {
            tokenizer_yolu
        } else {
            PathBuf::from("core/models/tokenizer.json")
        };

        let boru_hatti = VenexusBoruHatti::yeni(
            kahin,
            tekfirsel_katmanlari,
            word_embeddings,
            son_norm,
            lm_head,
            &tokenizer_dosyasi,
            cihaz.clone(),
        )?;

        Ok(Self { cihaz, boru_hatti })
    }
}

use crate::clie::Lang;
use hf_hub::api::sync::Api;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

pub struct ModelSecenegi {
    pub isim: &'static str,
    pub repo_id: &'static str,
    pub dosya_adi: &'static str,
    pub hedef_dosya_adi: &'static str,
}

pub fn model_kontrol_ve_yonet(lang: Lang) -> io::Result<PathBuf> {
    let model_dizini = Path::new("core/models");
    if !model_dizini.exists() {
        fs::create_dir_all(model_dizini)?;
    }

    if let Ok(entries) = fs::read_dir(model_dizini) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("safetensors") {
                return Ok(path);
            }
        }
    }

    let secenekler = [
        ModelSecenegi {
            isim: "Qwen2-VL (Multilingual / Recommended)",
            repo_id: "Qwen/Qwen2-VL-7B-Instruct",
            dosya_adi: "model-00001-of-00004.safetensors",
            hedef_dosya_adi: "qwen2_vl.safetensors",
        },
        ModelSecenegi {
            isim: "Baidu Unlimited-OCR (Chinese/English)",
            repo_id: "baidu/Unlimited-OCR",
            dosya_adi: "model-00001-of-000001.safetensors",
            hedef_dosya_adi: "model-00001-of-000001.safetensors",
        },
    ];

    println!("{}", lang.t("model_not_found"));

    for (i, secenek) in secenekler.iter().enumerate() {
        println!("  {}) {}", i + 1, secenek.isim);
    }

    print!("{} [1-{}]: ", lang.t("prompt_select"), secenekler.len());
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let index: usize = match input.trim().parse::<usize>() {
        Ok(n) if n > 0 && n <= secenekler.len() => n - 1,
        _ => {
            return Err(io::Error::other(lang.t("aborted")));
        }
    };

    let secilen = &secenekler[index];

    println!("{} {}", secilen.isim, lang.t("downloading"));

    let api = Api::new().map_err(|e| io::Error::other(e.to_string()))?;
    let repo = api.model(secilen.repo_id.to_string());

    let indirilen = repo
        .get(secilen.dosya_adi)
        .map_err(|e| io::Error::other(format!("{} {}", lang.t("download_failed"), e)))?;

    let hedef_yol = model_dizini.join(secilen.hedef_dosya_adi);
    fs::copy(&indirilen, &hedef_yol)?;

    println!("{} {:?}", lang.t("success_download"), hedef_yol);

    Ok(hedef_yol)
}

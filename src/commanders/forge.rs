use crate::clie::{ForgeArgs, Lang};
use crate::storme::Storme;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Serialize, Deserialize)]
struct ForgeRapor {
    basarili: Vec<String>,
    basarisiz: Vec<String>,
}

pub fn run(args: ForgeArgs, dil: Lang) -> io::Result<()> {
    let model_yolu = if let Some(ref ozel_yol) = args.model {
        let yol = PathBuf::from(ozel_yol);
        if yol.exists() { Some(yol) } else { None }
    } else {
        fs::read_dir("core/models").ok().and_then(|mut entries| {
            entries.find_map(|entry| {
                let path = entry.ok()?.path();
                if path.extension().and_then(|e| e.to_str()) == Some("safetensors") {
                    Some(path)
                } else {
                    None
                }
            })
        })
    };

    if let Some(ref yol) = model_yolu {
        println!(
            "{} ({:?})",
            dil.t("forge_model_found"),
            yol.file_name().unwrap_or_default()
        );
    } else {
        println!("{}", dil.t("forge_model_missing"));
    }

    let toplam_cpu = num_cpus::get();
    let is_parcasi = if args.omega {
        ((toplam_cpu as f32 * 0.80).ceil() as usize).max(1)
    } else {
        ((toplam_cpu as f32 * 0.40).ceil() as usize).max(1)
    };

    let havuz = rayon::ThreadPoolBuilder::new()
        .num_threads(is_parcasi)
        .build()
        .map_err(io::Error::other)?;

    let mut dosyalar: Vec<PathBuf> = WalkDir::new(&args.directory)
        .max_depth(2)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
        .map(|e| e.path().to_path_buf())
        .filter(|p| {
            p.extension()
                .map(|e| {
                    let ext = e.to_string_lossy().to_lowercase();
                    ext == "jpg" || ext == "jpeg" || ext == "pdf" || ext == "txt" || ext == "epub"
                })
                .unwrap_or(false)
        })
        .collect();

    if dosyalar.is_empty() {
        println!("{}", dil.t("no_files"));
        return Ok(());
    }
    dosyalar.sort();

    let pb = ProgressBar::new(dosyalar.len() as u64);
    pb.set_style(
        ProgressStyle::with_template("[{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}")
            .unwrap()
            .progress_chars("##-"),
    );
    pb.set_message(dil.t("forge_scanning"));

    let gorev_sonuclari: Vec<(String, PathBuf)> = havuz.install(|| {
        dosyalar
            .par_iter()
            .map(|dosya| {
                let uzanti = dosya
                    .extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or("")
                    .to_lowercase();

                let ham_metin = match uzanti.as_str() {
                    "txt" => fs::read_to_string(dosya).unwrap_or_default(),
                    "pdf" => match fs::read(dosya).and_then(|bytes| {
                        pdf_extract::extract_text_from_mem(&bytes)
                            .map_err(|_| io::Error::other("PDF parse error"))
                    }) {
                        Ok(text) => text,
                        Err(_) => format!("[{} {:?}]", dil.t("pdf_extract_error"), dosya),
                    },
                    _ => format!("[{} {:?}]", dil.t("image_file_pointer"), dosya),
                };

                let temiz_metin =
                    Storme::akademik_temizle(&ham_metin, args.top, args.strip_footers);

                pb.inc(1);
                (temiz_metin, dosya.clone())
            })
            .collect()
    });
    pb.finish_with_message(dil.t("forge_saving_corpus"));

    let mut rapor = ForgeRapor {
        basarili: vec![],
        basarisiz: vec![],
    };

    for (idx, (icerik_metni, orijinal_yol)) in gorev_sonuclari.into_iter().enumerate() {
        let sayac = idx + 1;
        let dosya_adi_md = format!("{}_{:04}.md", args.prefix, sayac);
        let dosya_adi_txt = format!("{}_{:04}.txt", args.prefix, sayac);

        let hedef_yol_md = Path::new(&args.directory).join(dosya_adi_md);
        let hedef_yol_txt = Path::new(&args.directory).join(dosya_adi_txt);

        let markdown_cikti = format!(
            "# {}\n- {}: {:?}\n\n{}",
            dil.t("corpus_doc"),
            dil.t("md_source"),
            orijinal_yol,
            icerik_metni
        );
        let md_basarili = fs::write(&hedef_yol_md, &markdown_cikti).is_ok();
        let txt_basarili = fs::write(&hedef_yol_txt, &icerik_metni).is_ok();

        if md_basarili && txt_basarili {
            rapor
                .basarili
                .push(hedef_yol_md.to_string_lossy().to_string());
            rapor
                .basarili
                .push(hedef_yol_txt.to_string_lossy().to_string());
        } else {
            rapor.basarisiz.push(format!("{:?}", orijinal_yol));
        }
    }

    let rapor_adi = format!("{}_forge_report.json", args.prefix);
    let rapor_yolu = Path::new(&args.directory).join(rapor_adi);
    let json = serde_json::to_string_pretty(&rapor).unwrap();
    let _ = fs::write(&rapor_yolu, json);

    println!("\n {}", dil.t("forge_done"));
    println!(
        "{}: {}, {}: {}",
        dil.t("forge_success_count"),
        rapor.basarili.len(),
        dil.t("fail_label"),
        rapor.basarisiz.len()
    );
    println!("{}: {}", dil.t("report_file"), rapor_yolu.display());

    Ok(())
}

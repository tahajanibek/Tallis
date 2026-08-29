use crate::clie::{Lang, Quozart7Args};
use crate::indi::model_kontrol_ve_yonet;
use crate::storme::Storme;
use venexus::Venexus;

use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Serialize, Deserialize)]
struct Quozart7Rapor {
    basarili: Vec<String>,
    basarisiz: Vec<String>,
}

pub fn run(args: Quozart7Args, dil: Lang) -> io::Result<()> {
    let model_yolu = model_kontrol_ve_yonet(dil)?;
    let model_dizini = model_yolu.parent().unwrap_or(Path::new("core/models"));
    let motor = Venexus::baslat(model_dizini)
        .map_err(|e| io::Error::other(format!("{} {}", dil.t("venexus_init_error"), e)))?;

    match motor.cihaz {
        candle_core::Device::Metal(_) => println!("{}", dil.t("metal_active")),
        candle_core::Device::Cuda(_) => println!("{}", dil.t("cuda_active")),
        candle_core::Device::Cpu => println!("{}", dil.t("cpu_active")),
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

    let cikis_dizini = if let Some(ref dir) = args.output_dir {
        let path = Path::new(dir);
        if !path.exists() {
            fs::create_dir_all(path)?;
        }
        path.to_path_buf()
    } else {
        Path::new(&args.directory).to_path_buf()
    };

    let mut dosyalar: Vec<PathBuf> = WalkDir::new(&args.directory)
        .max_depth(1)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .map(|e| e.path().to_path_buf())
        .filter(|p| {
            p.extension()
                .map(|e| {
                    let ext = e.to_string_lossy().to_lowercase();
                    ext == "jpg" || ext == "jpeg" || ext == "pdf" || ext == "txt"
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
            .progress_chars("++-"),
    );
    pb.set_message(dil.t("quozart_scanning"));

    let gorev_sonuclari: Vec<(Vec<(String, String)>, PathBuf)> = havuz.install(|| {
        dosyalar
            .par_iter()
            .map(|dosya| {
                let mut veri_listesi = Vec::new();
                let uzanti = dosya
                    .extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or("")
                    .to_lowercase();

                if uzanti == "txt" {
                    if let Ok(icerik) = fs::read_to_string(dosya) {
                        veri_listesi.push((icerik, "duz_metin".to_string()));
                    }
                } else if uzanti == "pdf" {
                    println!("{} {:?}", dil.t("pdf_detected"), dosya);
                } else if (uzanti == "jpg" || uzanti == "jpeg")
                    && let Ok(img) = image::open(dosya)
                {
                    let (sol_sayfa, sag_sayfa) = Storme::sayfayi_bol(&img);

                    let sol_metin = motor
                        .boru_hatti
                        .metin_uret(&sol_sayfa)
                        .unwrap_or_else(|e| format!("[{}: {}]", dil.t("left_page_error"), e));

                    let sag_metin = motor
                        .boru_hatti
                        .metin_uret(&sag_sayfa)
                        .unwrap_or_else(|e| format!("[{}: {}]", dil.t("right_page_error"), e));

                    veri_listesi.push((sol_metin, "sol".to_string()));
                    veri_listesi.push((sag_metin, "sag".to_string()));
                }

                pb.inc(1);
                (veri_listesi, dosya.clone())
            })
            .collect()
    });

    pb.finish_with_message(dil.t("saving_markdown"));

    let mut rapor = Quozart7Rapor {
        basarili: vec![],
        basarisiz: vec![],
    };

    let mut sayac = 1;

    for (sayfalar, orijinal_yol) in gorev_sonuclari {
        for (icerik_metni, taraf) in sayfalar {
            let dosya_adi_md = format!("{}_{:04}.md", args.prefix, sayac);
            let dosya_adi_txt = format!("{}_{:04}.txt", args.prefix, sayac);

            let hedef_yol_md = cikis_dizini.join(dosya_adi_md);
            let hedef_yol_txt = cikis_dizini.join(dosya_adi_txt);

            let markdown_cikti = format!(
                "# {}\n- {}: {:?}\n- {}: {}\n\n{}",
                dil.t("md_data"),
                dil.t("md_source"),
                orijinal_yol,
                dil.t("md_mode"),
                taraf,
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
                sayac += 1;
            } else {
                rapor
                    .basarisiz
                    .push(format!("{:?} ({})", orijinal_yol, taraf));
            }
        }
    }

    let rapor_adi = format!("{}_rapor.json", args.prefix);
    let rapor_yolu = cikis_dizini.join(rapor_adi);
    let json = serde_json::to_string_pretty(&rapor).unwrap();
    let _ = fs::write(&rapor_yolu, json);

    println!("\n {}", dil.t("quozart_done"));
    println!(
        "{}: {}, {}: {}",
        dil.t("success_label"),
        rapor.basarili.len(),
        dil.t("fail_label"),
        rapor.basarisiz.len()
    );
    println!("{}: {}", dil.t("report_file"), rapor_yolu.display());

    Ok(())
}

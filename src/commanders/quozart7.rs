use crate::clie::Quozart7Args;
use crate::storme::PageNumberExtractor;

use image::GenericImageView;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Serialize, Deserialize)]
struct Quozart7Report {
    success: Vec<String>,
    failed: Vec<String>,
}

pub fn run(args: Quozart7Args) -> io::Result<()> {
    let extractor =
        PageNumberExtractor::new("core/text-detector.rten", "core/text-recognitor.rten")
            .ok_or_else(|| io::Error::other("Models could not be loaded!"))?;

    let total_cpus = num_cpus::get();
    let num_threads = if args.omega {
        ((total_cpus as f32 * 0.80).ceil() as usize).max(1)
    } else {
        ((total_cpus as f32 * 0.40).ceil() as usize).max(1)
    };

    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build()
        .map_err(io::Error::other)?;

    if !args.jpg {
        println!("ERROR: Currently, only JPG/JPEG is supported for process.");
        return Ok(());
    }

    if args.omega {
        println!(
            "Omega Activate!: CPU will run at 90% capacity! ({}/{} threads)",
            num_threads, total_cpus
        );
    }

    let output_dir = if let Some(ref dir) = args.output_dir {
        let path = Path::new(dir);
        if !path.exists() {
            fs::create_dir_all(path)?;
        }
        path.to_path_buf()
    } else {
        Path::new(&args.directory).to_path_buf()
    };

    let mut files: Vec<PathBuf> = WalkDir::new(&args.directory)
        .max_depth(1)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .map(|e| e.path().to_path_buf())
        .filter(|p| {
            p.extension()
                .map(|e| {
                    let ext = e.to_string_lossy().to_lowercase();
                    ext == "jpg" || ext == "jpeg"
                })
                .unwrap_or(false)
        })
        .collect();

    if files.is_empty() {
        println!("ERROR: No JPG/JPEG files found in the directory.");
        return Ok(());
    }
    files.sort();

    let pb = ProgressBar::new(files.len() as u64);
    pb.set_style(
        ProgressStyle::with_template("[{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}")
            .unwrap()
            .progress_chars("++-"),
    );
    pb.set_message("Processing Pages...");

    let task_results: Vec<(Vec<(image::DynamicImage, String)>, PathBuf)> = pool.install(|| {
        files
            .par_iter()
            .map(|file| {
                let mut page_data = Vec::new();

                if let Ok(img) = image::open(file) {
                    let (w, h) = img.dimensions();

                    //universal side-by-side
                    let left_page = img.crop_imm(0, 0, w / 2, h);
                    let right_page = img.crop_imm(w / 2, 0, w / 2, h);

                    page_data.push((left_page, "left".to_string()));
                    page_data.push((right_page, "right".to_string()));
                }

                pb.inc(1);
                (page_data, file.clone())
            })
            .collect()
    });
    pb.finish_with_message("OCR & Processing completed!");

    let mut used = HashSet::new();
    let mut report = Quozart7Report {
        success: vec![],
        failed: vec![],
    };

    println!("Saving files and generating unique names...");

    for (pages, original_path) in task_results {
        for (half, side) in pages {
            let page_num = extractor.extract(&half, Some(side.as_str()), args.top);

            match page_num {
                Some(num) => {
                    let final_path =
                        generate_unique_name(&output_dir, &args.prefix, &num, &mut used);
                    if half.save(&final_path).is_ok() {
                        report
                            .success
                            .push(final_path.to_string_lossy().to_string());
                    }
                }
                None => {
                    report
                        .failed
                        .push(format!("{} ({})", original_path.display(), side));
                }
            }
        }
    }

    let report_filename = format!("{}_report.json", args.prefix);

    let report_path = output_dir.join(report_filename);

    let json = serde_json::to_string_pretty(&report).unwrap();

    match fs::write(&report_path, json) {
        Ok(_) => println!("Report generated: {}", report_path.display()),
        Err(e) => println!("Failed to generate report: {}", e),
    }

    println!("\n Transaction Completed!");
    println!(
        "Success: {}, Failed: {}",
        report.success.len(),
        report.failed.len()
    );
    println!("Report: {}", report_path.display());

    Ok(())
}

fn generate_unique_name(
    dir: &Path,
    prefix: &str,
    number: &str,
    used: &mut HashSet<String>,
) -> PathBuf {
    let mut counter = 0;
    loop {
        let name = if counter == 0 {
            format!("{}_{}.jpg", prefix, number)
        } else {
            format!("{}_{}_{}.jpg", prefix, number, counter)
        };

        if !used.contains(&name) && !dir.join(&name).exists() {
            used.insert(name.clone());
            return dir.join(name);
        }
        counter += 1;
    }
}

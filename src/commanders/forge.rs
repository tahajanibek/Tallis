use crate::clie::ForgeArgs;
use crate::storme::PageNumberExtractor;

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::collections::HashSet;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::time::{Instant, SystemTime};
use uuid::Uuid;
use walkdir::WalkDir;

pub fn run(args: ForgeArgs) -> io::Result<()> {
    let extractor = std::sync::Arc::new(
        PageNumberExtractor::new("core/text-detector.rten", "core/text-recognitor.rten")
            .ok_or_else(|| io::Error::other("Modeller yüklenemedi"))?,
    );

    let total_cpus = num_cpus::get();
    let num_threads = if args.omega {
        ((total_cpus as f32 * 0.80).ceil() as usize).max(1)
    } else {
        ((total_cpus as f32 * 0.40).ceil() as usize).max(1)
    };

    if args.omega {
        println!(
            "Omega Activate!: CPU will run at 90% capacity!! ({}/{} threads)",
            num_threads, total_cpus
        );
    }

    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build()
        .map_err(io::Error::other)?;

    let files = collect_and_sort(&args.directory)?;
    let total = files.len();

    if total == 0 {
        println!(
            "ERROR: '{}' No files were found in the directory.",
            args.directory
        );
        return Ok(());
    }

    let m = MultiProgress::new();
    let pb_ocr = m.add(ProgressBar::new(total as u64));
    let pb_rename = m.add(ProgressBar::new(total as u64));

    let style_shared = ProgressStyle::with_template(
        "[{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta}) {msg}",
    )
    .unwrap()
    .progress_chars("##-");

    pb_ocr.set_style(style_shared.clone());
    pb_ocr.set_message("Analyzing (OCR)...");
    pb_rename.set_style(style_shared);
    pb_rename.set_message("Renaming...");

    let start_ocr = Instant::now();
    let results: Vec<(PathBuf, Option<String>)> = pool.install(|| {
        files
            .par_iter()
            .map(|path| {
                let number = image::open(path)
                    .ok()
                    .and_then(|img| extractor.extract(&img, None, args.top));
                pb_ocr.inc(1);
                (path.clone(), number)
            })
            .collect()
    });
    pb_ocr.finish_with_message(format!("OCR completed! ({:.2?})", start_ocr.elapsed()));

    let start_rename = Instant::now();
    let mut temp_mappings = Vec::new();

    for (original, number_opt) in results {
        if let Some(number) = number_opt {
            let ext = original
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("jpg");
            let temp_name = format!("__tmp_{}.{}", Uuid::new_v4(), ext);
            let temp_path = original.parent().unwrap().join(temp_name);

            if fs::rename(&original, &temp_path).is_ok() {
                temp_mappings.push((temp_path, number));
            }
        } else {
            pb_rename.inc(1);
        }
    }

    let mut used_names = HashSet::new();
    for (temp_path, num_str) in temp_mappings {
        let final_path = generate_unique_name(
            temp_path.parent().unwrap(),
            &args.prefix,
            &num_str,
            &mut used_names,
        );
        let _ = fs::rename(temp_path, final_path);
        pb_rename.inc(1);
    }

    pb_rename.finish_with_message(format!(
        "Renaming completed! ({:.2?})",
        start_rename.elapsed()
    ));

    println!(
        "\n Transaction completed successfully!\n Total duration: {:.2?}",
        start_ocr.elapsed() + start_rename.elapsed()
    );

    Ok(())
}

fn collect_and_sort(directory: &str) -> io::Result<Vec<PathBuf>> {
    let mut files: Vec<PathBuf> = WalkDir::new(directory)
        .max_depth(1)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
        .map(|e| e.path().to_path_buf())
        .filter(|p| {
            let s = p.to_string_lossy().to_lowercase();
            s.ends_with(".jpg") || s.ends_with(".jpeg")
        })
        .collect();

    if files.is_empty() {
        return Err(io::Error::other("Currently, only JPG/JPEG is supported."));
    }

    files.sort_by(|a, b| {
        let meta_a = fs::metadata(a)
            .and_then(|m| m.created().or_else(|_| m.modified()))
            .unwrap_or(SystemTime::UNIX_EPOCH);
        let meta_b = fs::metadata(b)
            .and_then(|m| m.created().or_else(|_| m.modified()))
            .unwrap_or(SystemTime::UNIX_EPOCH);
        meta_a
            .cmp(&meta_b)
            .then_with(|| a.file_name().cmp(&b.file_name()))
    });

    Ok(files)
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

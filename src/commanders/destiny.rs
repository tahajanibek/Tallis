use std::collections::HashSet;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use crate::clie::{DestinyArgs, Lang};
use uuid::Uuid;
use walkdir::WalkDir;

pub fn run(args: DestinyArgs, dil: Lang) -> io::Result<()> {
    let target_ext = args.extension.to_lowercase().replace(".", "");

    let mut entries: Vec<PathBuf> = WalkDir::new(&args.directory)
        .max_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            let p = e.path();
            p.is_file()
                && p.extension()
                    .and_then(|ext| ext.to_str())
                    .map(|ext| ext.to_lowercase() == target_ext)
                    .unwrap_or(false)
        })
        .map(|e| e.path().to_path_buf())
        .collect();

    if entries.is_empty() {
        println!("{}", dil.t("no_files"));
        return Ok(());
    }

    entries.sort_by(|a, b| {
        let meta_a = fs::metadata(a);
        let meta_b = fs::metadata(b);

        let time_a = meta_a
            .and_then(|m| m.created().or_else(|_| m.modified()))
            .unwrap_or(SystemTime::UNIX_EPOCH);
        let time_b = meta_b
            .and_then(|m| m.created().or_else(|_| m.modified()))
            .unwrap_or(SystemTime::UNIX_EPOCH);

        time_a
            .cmp(&time_b)
            .then_with(|| a.file_name().cmp(&b.file_name()))
    });

    println!(
        "{} .{} {}",
        entries.len(),
        target_ext,
        dil.t("destiny_start")
    );

    let mut temp_mappings = Vec::new();
    for original in &entries {
        let temp_name = format!("__tmp_{}.{}", Uuid::new_v4(), target_ext);
        let temp_path = original.parent().unwrap().join(&temp_name);

        if let Err(e) = fs::rename(original, &temp_path) {
            eprintln!(
                "{} {:?}: {}",
                dil.t("destiny_warn"),
                original.file_name(),
                e
            );
            continue;
        }
        temp_mappings.push(temp_path);
    }

    let mut used = HashSet::new();
    let mut count = 0;
    let total_to_process = temp_mappings.len();

    for (idx, temp_path) in temp_mappings.into_iter().enumerate() {
        let final_path = generate_unique_name(
            temp_path.parent().unwrap(),
            &args.prefix,
            idx + 1,
            &mut used,
            &target_ext,
        );

        if fs::rename(&temp_path, &final_path).is_ok() {
            count += 1;
        }
    }

    println!("{}: {}/{}", dil.t("destiny_done"), count, total_to_process);
    Ok(())
}

fn generate_unique_name(
    dir: &Path,
    prefix: &str,
    number: usize,
    used: &mut HashSet<String>,
    ext: &str,
) -> PathBuf {
    let mut counter = 0;
    loop {
        let name = if counter == 0 {
            format!("{}_{}.{}", prefix, number, ext)
        } else {
            format!("{}_{}_{}.{}", prefix, number, counter, ext)
        };

        if !used.contains(&name) && !dir.join(&name).exists() {
            used.insert(name.clone());
            return dir.join(name);
        }
        counter += 1;
    }
}

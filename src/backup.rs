use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use std::time::{Duration, Instant};
use sysinfo::{DiskExt, System, SystemExt};
use walkdir::WalkDir;

fn list_external_drives() -> Vec<String> {
    let mut system = System::new_all();
    system.refresh_disks_list();

    let mut drives = Vec::new();
    for disk in system.disks() {
        if disk.is_removable() {
            if let Some(name) = disk.mount_point().to_str() {
                drives.push(name.to_string());
            }
        }
    }
    drives
}

fn copy_directory(src: &Path, dst: &Path) -> io::Result<(u64, Duration)> {
    let start_time = Instant::now();
    let mut total_bytes = 0;

    for entry in WalkDir::new(src) {
        let entry = entry?;
        let path = entry.path();
        let relative_path = path.strip_prefix(src).unwrap();
        let dest_path = dst.join(relative_path);

        if path.is_dir() {
            fs::create_dir_all(&dest_path)?;
        } else {
            fs::copy(&path, &dest_path)?;
            total_bytes += entry.metadata()?.len();
        }
    }

    let duration = start_time.elapsed();
    Ok((total_bytes, duration))
}

pub fn backup_execute() -> io::Result<()> {
    let current_dir = std::env::current_dir()?;
    let drives = list_external_drives();

    if drives.is_empty() {
        eprintln!("Non ci sono drive esterni collegati.");
        return Ok(());
    }

    println!("Drives disponibili:");
    for (i, drive) in drives.iter().enumerate() {
        println!("{}: {}", i, drive);
    }

    println!("Seleziona il drive su cui eseguire il backup (per indice):");
    let mut index = String::new();
    std::io::stdin().read_line(&mut index)?;
    let index: usize = index.trim().parse().expect("indice non valido");

    if index >= drives.len() {
        eprintln!("indice non valido.");
        return Ok(());
    }

    let selected_drive = &drives[index];
    let backup_dir = Path::new(selected_drive).join("backup");

    fs::create_dir_all(&backup_dir)?;

    let (total_bytes, duration) = copy_directory(&current_dir, &backup_dir)?;

    let mut report_file = File::create(backup_dir.join("backup_report.txt"))?;
    writeln!(report_file, "Backup completed in: {:?}", duration)?;
    writeln!(report_file, "Total bytes copied: {}", total_bytes)?;

    println!("Backup completato correttamente.");
    Ok(())
}

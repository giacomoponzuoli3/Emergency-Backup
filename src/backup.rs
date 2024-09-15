use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
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
    println!("{:?}", drives);
    drives
}

fn copy_directory(src: &Path, dst: &Path, file_types: &[String]) -> io::Result<(u64, Duration)> {
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
            if should_copy_file(path, file_types) {
                fs::copy(&path, &dest_path)?;
                total_bytes += entry.metadata()?.len();
            }
        }
    }

    let duration = start_time.elapsed();
    Ok((total_bytes, duration))
}

fn should_copy_file(path: &Path, file_types: &[String]) -> bool {
    if file_types.contains(&"all".to_string()) {
        return true;
    }

    if let Some(extension) = path.extension().and_then(|e| e.to_str()) {
        return file_types.iter().any(|file_type| file_type == extension);
    }

    false
}

pub fn backup_execute(selected_drive: &String, src_dir: &Path, file_types: &[String]) -> io::Result<()> {

    // --- TUTTA QUESTA LOGICA VA SPOSTATA NELLA FINESTRA DI CONFIGURAZIONE
    /*
        let src_dir = std::env::current_dir()?;
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
    */
    //--FINE-----

    let backup_dir = Path::new(selected_drive).join("backup");

    fs::create_dir_all(&backup_dir)?;

    let (total_bytes, duration) = copy_directory(&src_dir, &backup_dir, file_types)?;

    let mut report_file = File::create(backup_dir.join("backup_report.txt"))?;
    writeln!(report_file, "Backup completed in: {:?}", duration)?;
    writeln!(report_file, "Total bytes copied: {}", total_bytes)?;

    println!("Backup completato correttamente.");
    Ok(())
}

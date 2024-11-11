use std::path::PathBuf;
use std::env;

pub fn get_base_path(base_path: &PathBuf) -> Option<PathBuf> {
    let os = env::consts::OS;

    let final_path = match os {
        "windows" => base_path.join("windows"),
        "macos" => {
            if cfg!(target_arch = "x86_64") { // Architettura Intel
                base_path.join("macos-intel")
            } else { // Architettura ARM
                base_path.join("macos-arm")
            }
        },
        "linux" => base_path.join("linux"),
        _ => {
            eprintln!("Sistema operativo non supportato.");
            return None;
        }
    };

    Some(final_path)
}

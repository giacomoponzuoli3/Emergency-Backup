use std::process::Command;

fn main() {
    #[cfg(target_os = "windows")]
    {
        let _ = Command::new("taskkill")
            .args(&["/IM", "Group-35.exe", "/F"])
            .output()
            .expect("Failed to execute command");
    }

    #[cfg(target_os = "macos")]
    {
        let _ = Command::new("pkill")
            .args(&["-f", "Group-35"])
            .output()
            .expect("Failed to execute command");
    }

    #[cfg(target_os = "linux")]
    {
        let _ = Command::new("pkill")
            .args(&["-f", "Group-35"])
            .output()
            .expect("Failed to execute command");
    }
}

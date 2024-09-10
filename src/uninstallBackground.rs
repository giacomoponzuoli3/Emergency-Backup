use std::env;
use std::process::Command;
use auto_launch::{ AutoLaunchBuilder};

fn main() {
    let exe = env::current_exe().unwrap(); // exe path
    let wd = exe.parent().unwrap();
    let app_path = wd.join("Group-35");

    let _ = AutoLaunchBuilder::new()
        .set_app_name("Group-35")
        .set_app_path(&app_path.to_str().unwrap())
        .set_use_launch_agent(false)
        .build()
        .unwrap().disable();

    #[cfg(target_os = "macos")]
    {
        let _ = AutoLaunchBuilder::new()
            .set_app_name("Group-35")
            .set_app_path(&app_path.to_str().unwrap())
            .set_use_launch_agent(true)
            .build()
            .unwrap().disable();
    }

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
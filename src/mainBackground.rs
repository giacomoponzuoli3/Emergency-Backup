

use auto_launch::{ AutoLaunchBuilder};
use std::env;
use std::process::{Command};
use crate::log::log_with_tick;

fn main() {
    let exe = env::current_exe().unwrap(); // exe path
    let wd = exe.parent().unwrap();
    println!("{:?}", wd); //  /Users/giacomoponzuoli/Desktop/Programmazione di sistema/Programmazione RUST/Group-35/target/debug

    /* Autostart configuration */
    let app_path = wd.join("Group-35");
    println!("{:?}", app_path); // /Users/giacomoponzuoli/Desktop/Programmazione di sistema/Programmazione RUST/Group-35/target/debug/Group-35


    //configurazione autostart per Windows e linux
    #[cfg(not(target_os = "macos"))]
    {
        let auto = AutoLaunchBuilder::new()
            .set_app_name("Group-35")
            .set_app_path(&app_path.to_str().unwrap())
            .set_use_launch_agent(false)
            .build()
            .unwrap();

        auto.enable().unwrap();
        println!("Autostart enabled: {}", auto.is_enabled().unwrap());
    }


    #[cfg(target_os = "macos")]
    {
        let _ = AutoLaunchBuilder::new()
            .set_app_name("Group-35") //nome che verrà utilizzato per identificare l'applicazione che verrà avviata automaticamente all'avvio del sistema
            .set_app_path(&app_path.to_str().unwrap()) //Imposta il percorso dell'eseguibile che deve essere lanciato automaticamente
            .set_use_launch_agent(false) //Questo imposta se utilizzare i Launch Agents su macOS (in questo caso no)
            .build() //Crea la configurazione completa per l'auto-launch, con tutte le opzioni specificate
            .unwrap().enable(); //abilita l'avvio automatico della tua applicazione. Dopo aver chiamato questo metodo, l'applicazione "Group-35" verrà configurata per avviarsi automaticamente all'accensione del sistema

        Command::new("osascript") //comando di macOS che permette di eseguire script AppleScript direttamente dalla linea di comando
            .arg("-e") // indica che il successivo comando sarà uno script passato direttamente sulla linea di comando
            .arg("tell application \"Terminal\" to set visible of front window to false") //script AppleScript vero e proprio. L'istruzione tell application "Terminal" invia un comando all'applicazione Terminale
            .output() //Esegue il comando e raccoglie l'output del processo
            .expect("Failed to hide terminal"); //Se il comando non viene eseguito correttamente
    }

    /*
    eventuali altre esecuzioni in background
    ...
    */

    loop {
        log_with_tick().unwrap();
    }
}
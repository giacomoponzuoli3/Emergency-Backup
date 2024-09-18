

use auto_launch::{ AutoLaunchBuilder};
use std::env;
use std::process::{Command};
use std::path::Path;
use std::thread;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};

use std::fs::File;
use std::io::{Error, Write};
use std::time::{Duration, Instant};
use crossbeam_channel::tick;


// ESEMPIO BASE SCRITTURA FILE LOG OGNI 2 MINUTI
// bisogna prendere l'utilizzo di CPU del processo


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
    Controllo che non ci sia un file output.csv all'interno della directory corrente,
    se non c'è lancio la gui altrimenti no
    */

    // Verifica se il file "output.csv" esiste nella directory corrente
    if Path::new("output.csv").exists() {
        println!("Il file 'output.csv' esiste. Non lancio la GUI");

    } else {
        println!("Il file 'output.csv' non esiste. Lancio la GUI");
        // Definisci il comando da eseguire

        // Rileva il sistema operativo
        let os = env::consts::OS;

        // Definisci il comando da eseguire in base al sistema operativo
        let command = match os {
            "windows" => "release\\windows\\gui.exe", // Comando per Windows
            "macos" => "release/macos/gui", // Comando per macOS
            "linux" => "release/linux/gui", // Comando per Linux
            _ => {
                eprintln!("Sistema operativo non supportato.");
                return;
            }
        };

        // Esegui il comando
        let status = Command::new(command)
            .status()
            .expect("Errore durante l'esecuzione del comando");

        // Controlla se il comando è stato eseguito con successo
        if status.success() {
            println!("Il comando è stato eseguito con successo.");
        } else {
            eprintln!("Il comando ha restituito un errore.");
        }
    }


    /* Processo di che avvia l'applicazione  */
    let exe = env::current_exe().unwrap(); // exe path
    let wd = exe.parent().unwrap();
    let backup_path = wd.join("main");


    //avvio background dell'app
    #[cfg(target_os = "windows")]
    {
        let output = Command::new("tasklist") //controllo la lista dei processi attivi in windows
            .args(&["/FI", "IMAGENAME eq main.exe", "/FO", "CSV", "/NH"]) //filtro per quelli che hanno il nome "main.exe" e li metto in un formato CSV
            .output()
            .expect("Failed to execute command");

        let exists = String::from_utf8_lossy(&output.stdout).split(",").count() > 1; //output viene decodificato

        if exists { //se è già nella lista dei processi in esecuzione
            println!("Backup App already running!");
        } else { //altrimenti lo avvio
            let mut backup_app = Command::new(backup_path)
                .spawn()
                .expect("Failed to create backup");

            backup_app.wait().expect("Failed to wait on backup");
        }

    }

    #[cfg(not(target_os = "windows"))]
    {
        let pid = Command::new("pgrep")  //cerca il processo con il nome specificato da dal path
            .args(&["-f", &backup_path.to_str().unwrap()])
            .output();
        //se restituisce Ok(_) significa che non è avvenuto nessun errore e bisogna controllare che il processo sia in esecuzione tramite il pid

        match &pid {
            Ok(_) => {
                //se il processo è in esecuzione il comando restituisce su stdout il pid del processo altrimenti stdout è vuoto
                if !pid.unwrap().stdout.is_empty() {
                    println!("Backup App already running!");
                } else {
                    let mut backup_app = Command::new(backup_path)
                        .spawn()
                        .expect("Failed to execute process");

                    backup_app.wait().expect("Failed to wait on backup");
                }
            },
            Err(e) => println!("Error: {}", e),
        }
    }

}
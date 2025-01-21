mod model;

use auto_launch::{ AutoLaunchBuilder};
use std::env;
use std::process::{Command};
use std::path::{Path, PathBuf};
use std::thread;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::fs::create_dir_all;

use std::fs::File;
use std::io::{Error, Write};
use std::time::{Duration, Instant};
use crossbeam_channel::tick;
use crate::model::path_base::get_base_path;

fn create_autostart_linux(app_name: &str, app_path: &str) {
    // Trova la cartella di autostart dell'utente
    let autostart_dir = dirs::home_dir()
        .unwrap()
        .join(".config")
        .join("autostart");

    // Se la cartella di autostart non esiste, la crea
    if !autostart_dir.exists() {
        create_dir_all(&autostart_dir).expect("Impossibile creare la cartella autostart");
    }

    // Crea un file .desktop per l'applicazione
    let file_path = autostart_dir.join(format!("{}.desktop", app_name));
    let mut file = File::create(file_path).expect("Impossibile creare il file .desktop");

    // Scrivi il contenuto del file .desktop
    let content = format!(
        "[Desktop Entry]\n\
        Name={}\n\
        Exec={}\n\
        Type=Application\n\
        X-GNOME-Autostart-enabled=true\n\
        ",
        app_name, app_path
    );

    file.write_all(content.as_bytes()).expect("Errore durante la scrittura del file .desktop");
}

fn main() {

    // Rileva il sistema operativo corrente
    let os = env::consts::OS;

    //desktop path ../Desktop
    let desktop_path = dirs::desktop_dir()
        .expect("Impossibile ottenere la cartella Desktop");

    //base path per tutti gli eseguibili è ../Desktop/Group-35/release
    let mut base_path: PathBuf = desktop_path
        .join("Group-35")// Aggiungi la cartella "Group-35"
        .join("release");   // Aggiungi il file specificato

    //definizione base path ../Desktop/Group-35/release/...
    base_path = match get_base_path(&base_path) {
        Some(path) => path,
        None => return, // Esci se il sistema operativo non è supportato
    };

    //determino il path dell'auto start
    let mut auto_start_path = base_path.join("Group-35");


    // Se il sistema è Windows, aggiungi l'estensione ".exe"
    #[cfg(windows)]
    {
        auto_start_path.set_extension("exe");
    }

    println!("Auto start path: {:?}", auto_start_path);

    //configurazione autostart per Windows e linux
    #[cfg(target_os = "windows")]
    {
        let auto = AutoLaunchBuilder::new()
            .set_app_name("Group-35")
            .set_app_path(&auto_start_path.to_str().unwrap())
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
            .set_app_path(&auto_start_path.to_str().unwrap()) //Imposta il percorso dell'eseguibile che deve essere lanciato automaticamente
            .set_use_launch_agent(false) //Questo imposta se utilizzare i Launch Agents su macOS (in questo caso no)
            .build() //Crea la configurazione completa per l'auto-launch, con tutte le opzioni specificate
            .unwrap().enable(); //abilita l'avvio automatico della tua applicazione. Dopo aver chiamato questo metodo, l'applicazione "Group-35" verrà configurata per avviarsi automaticamente all'accensione del sistema
    }

    #[cfg(target_os = "linux")]
    {
        // Genera il file per l'autostart su Linux
        create_autostart_linux("Group-35", &auto_start_path.to_str().unwrap());
    }

    /*
    Controllo che non ci sia un file output.csv all'interno della directory corrente,
    se non c'è lancio la gui altrimenti no
    */

    // Verifica se il file "output.csv" esiste nella directory corrente
    let path_csv = desktop_path
        .join("Group-35")
        .join("output.csv");

    if path_csv.exists() {
        println!("Il file 'output.csv' esiste. Non lancio la GUI");

    } else {
        println!("Il file 'output.csv' non esiste. Lancio la GUI");

        //determino il path della gui
        let mut path_gui = base_path.clone().join("gui");

        // Se il sistema è Windows, aggiungi l'estensione ".exe"
        #[cfg(windows)]
        {
            path_gui.set_extension("exe");
        }

        println!("GUI path: {:?}", path_gui);

        // Esegui il comando
        let mut status = Command::new(path_gui.clone())
            .status()
            .expect("Errore durante l'esecuzione del comando");

        // Controlla se il comando è stato eseguito con successo, altrimenti rimostra la gui
        while !(status.success()) {
            println!("Errore nella creazione della GUI.");
            status = Command::new(path_gui.clone())
                .status()
                .expect("Errore durante l'esecuzione del comando");
        }

        println!("Il comando è stato eseguito con successo.");
    }


    /* Processo di che avvia l'applicazione del backup */
    let mut path_main = base_path.clone().join("main");
    // Se il sistema è Windows, aggiungi l'estensione ".exe"
    #[cfg(windows)]
    {
        path_main.set_extension("exe");
    }

    println!("Backup path: {:?}", path_main);

    //avvio background dell'app
    #[cfg(target_os = "windows")]
    {
        let output = Command::new("tasklist") //controllo la lista dei processi attivi in windows
            .args(&["/FI", "IMAGENAME eq main.exe", "/FO", "CSV", "/NH"]) //filtro per quelli che hanno il nome "main.exe" e li metto in un formato CSV
            .output()
            .expect("Il comando tasklist non è stato eseguito con successo.");

        let exists = String::from_utf8_lossy(&output.stdout).split(",").count() > 1; //output viene decodificato

        if exists { //se è già nella lista dei processi in esecuzione
            println!("L'app di backup è già in esecuzione!");
        } else { //altrimenti lo avvio
            let mut backup_app = Command::new(path_main)
                .spawn()
                .expect("Il comando per la crezione dell'app non è stata eseguita con successo.");

            backup_app.wait().expect("Impossibile attendere il backup");
        }

    }

    #[cfg(not(target_os = "windows"))]
    {
        let pid = Command::new("pgrep")  //cerca il processo con il nome specificato da dal path
            .args(&["-f", &path_main.to_str().unwrap()])
            .output();
        //se restituisce Ok(_) significa che non è avvenuto nessun errore e bisogna controllare che il processo sia in esecuzione tramite il pid
        match &pid {
            Ok(_) => {
                //se il processo è in esecuzione il comando restituisce su stdout il pid del processo altrimenti stdout è vuoto
                if !pid.unwrap().stdout.is_empty() {
                    println!("L'app di backup è già in esecuzione!");
                } else {
                    let mut backup_app = Command::new(path_main)
                        .spawn()
                        .expect("Il comando per la crezione dell'app non è stata eseguita con successo.");

                    backup_app.wait().expect("Impossibile attendere il backup");
                }
            },
            Err(e) => println!("Errore: {}", e),
        }
    }
}
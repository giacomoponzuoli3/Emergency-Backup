use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::path::Path;
use std::sync::mpsc::{self, Receiver};
use std::thread;
use std::time::{Duration, Instant};
use sysinfo::{Pid, PidExt, ProcessExt, System, SystemExt};

pub fn log_with_tick(log_dir: &Path, pid: i32) -> io::Result<()> {
    // Crea o apri il file di log nella directory specificata
    let log_file_path = log_dir.join("cpu_usage_log.txt");
    let mut file = OpenOptions::new().create(true).append(true).open(&log_file_path)?;

    // Crea un ticker che scatta ogni 120 secondi
    let ticker = start_ticker(Duration::from_secs(5));

    // Inizia a monitorare la CPU del processo
    let mut system = System::new_all();
    loop {
        // Aspetta che il ticker riceva un segnale
        ticker.recv().unwrap();

        // Aggiorna le informazioni del sistema
        system.refresh_processes();

        // Ottieni il consumo di CPU del processo specificato
        if let Some(process) = system.process(Pid::from(pid)) {
            let cpu_usage = process.cpu_usage();
            let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

            // Scrivi le informazioni nel file di log
            writeln!(file, "{} - CPU Usage: {:.2}%", timestamp, cpu_usage)?;
            println!("Logged: {} - CPU Usage: {:.2}%", timestamp, cpu_usage);
        } else {
            eprintln!("Processo con PID {} non trovato!", pid);
        }
    }
}

// Funzione di utilità per creare un ticker che scatta ogni intervallo di tempo specificato
fn start_ticker(interval: Duration) -> Receiver<()> {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || loop {
        thread::sleep(interval);
        if tx.send(()).is_err() {
            break; // Interrompe il thread se il canale viene chiuso
        }
    });
    rx
}

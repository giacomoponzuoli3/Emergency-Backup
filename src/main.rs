use crate::backup::backup_execute;
use rdev::{listen, EventType, Event};
use std::sync::{Arc, Mutex, mpsc};
use std::{env, thread};
use std::any::Any;
use std::collections::VecDeque;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};
use csv::Writer;
use iced::{Application, Sandbox, Settings};
use winit::event_loop::EventLoop;
use crate::beep::play_beep;
use crate::gui::{Bootstrap, MyApp, Segno};
use crate::log::log_with_tick;
use crate::shapeRecognize::shape_recognizer;

mod shapeRecognize;
mod backup;
mod log;
mod mainBackground;
mod uninstallBackground;
mod beep;
mod gui;
mod countdownGui;

struct MouseState {
   sides: [bool; 4], // [top, bottom, left, right]
   points: VecDeque<(f64, f64)>, // Punti del mouse per cerchio, rettangolo e segno -
   recognized_shape: Option<String>, // Forma riconosciuta
}


fn main() {
   /*
   //PRELEVARE I VALORI DAL FILE
   let value = MyApp::get_value();
   //FARE UN CONTROLLO PER VERIFICARE SE ESISTE IL FILE, SE NON ESISTE CREARLO CON IL CODICE SEGUENTE
   let file = File::create("output.csv").expect("Non posso creare il file CSV");
   let mut wtr = Writer::from_writer(file);
   wtr.serialize(value).expect("Non posso scrivere i dati nel file CSV");
   wtr.flush().expect("Non posso salvare i dati nel file");
   */


   // Get the current directory as a PathBuf
   let path = env::current_dir().unwrap();

   // You can directly use &path without converting to &str
   let log_dir = path.clone();

   //let log_dir = Path::new("/Users/marco/Desktop/dati_per_backup");

   let pid = std::process::id(); // Usa l'ID del processo corrente per testare

   // Avvia il processo di monitoraggio della CPU in maniera parallela rispetto alla funzionalità di backup
   /*thread::spawn(move||{
      log_with_tick(log_dir.as_path(), pid as i32).unwrap();
   });*/


   let state = Arc::new(Mutex::new(MouseState {
      sides: [false; 4],
      points: VecDeque::new(),
      recognized_shape: None,
   }));

   // Crea un event loop per ottenere la dimensione dello schermo
   let event_loop = EventLoop::new();
   let primary_monitor = event_loop.primary_monitor().unwrap();

   // Recupera le dimensioni fisiche del monitor
   let size = primary_monitor.size(); // Dimensioni fisiche
   let scale_factor = primary_monitor.scale_factor(); // Fattore di scaling

   // Calcola le dimensioni logiche
   let logical_width = (size.width as f64 / scale_factor) as f64;
   let logical_height = (size.height as f64 / scale_factor) as f64;

   let mut first_recognition_done = false;
   let mut enabled= true;

   loop {

      // carico i parametri dal file.
      // TEORICAMENTE SE CARICO QUI LA CONFIGURAZIONE, A RUNTIME POSSO AGGIONRARE I PARAMETRI.
      // SE LA METTO PRIMA DEL LOOP, I PARAMETRI VENGONO CARICATI SOLO LA PRIMA VOLTA,
      // QUINDI PER RENDERE EFFETTIVE LE MODIFICHE PRESUMO CI VOGLIA UN RIAVVIO
/*
      let config_parameters = MyApp::load_from_csv("output.csv");
      match config_parameters{
         Ok(app) => {},
         Err(err) => {println!("errore caricamento configurazione")}
      }
 */


      if enabled {
         if shape_recognizer("rettangolo", Arc::clone(&state), logical_width, logical_height, true) {
            if !first_recognition_done {
               play_beep(Duration::from_millis(100)); // Bip corto
               first_recognition_done = true;
            }

            println!("Recognition done!");
            // ---- QUI DEVE USCIRE UNA FINESTRA PER LA CONFERMA CON UN TIMER: SE ANNULLO NON SUCCEDE NIENTE
            // ALTRIMENTI CONFERMO CON IL SECONDO SEGNO.
            // LEGGENDO LA TRACCIA NON MI è CHIARO DOVE DOVREBBE USCIRE, SE QUA OPPURE DOPO AVER RICONOSCIUTO ANCHE IL SECONDO SEGNO

            if shape_recognizer("rettangolo", Arc::clone(&state), logical_width, logical_height, false) {

               play_beep(Duration::from_millis(500)); // Bip lungo
               println!("backup");
               enabled = false;

               backup_execute( &"/Volumes/ESD-USB".to_string() , "/Users/marco/Desktop/dati_per_backup", &vec!["pdf".to_string()] ).expect("errore");
               enabled = true;
               first_recognition_done = false; // Resetta il flag per riconoscere di nuovo
            } else {
               println!("scaduto");
               first_recognition_done = false;
            }
         }
      }else {
         println!("no")
      }
   }
}
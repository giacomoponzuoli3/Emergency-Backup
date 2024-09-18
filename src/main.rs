use crate::backup::backup_execute;
use rdev::{listen, EventType, Event};
use std::sync::{Arc, Mutex, mpsc};
use std::{env, thread};
use std::collections::VecDeque;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};
use iced::{Sandbox, Settings};
use winit::event_loop::EventLoop;
use crate::beep::play_beep;
use crate::gui::MyApp;
use crate::log::log_with_tick;
use crate::shapeRecognize::shape_recognizer;

mod shapeRecognize;
mod backup;
mod log;
mod mainBackground;
mod uninstallBackground;
mod beep;
mod gui;


struct MouseState {
   sides: [bool; 4], // [top, bottom, left, right]
   points: VecDeque<(f64, f64)>, // Punti del mouse per cerchio, rettangolo e segno -
   recognized_shape: Option<String>, // Forma riconosciuta
}


fn main() {
   // Get the current directory as a PathBuf
   let path = env::current_dir().unwrap();

   // You can directly use &path without converting to &str
   let log_dir = path.clone();

   //let log_dir = Path::new("/Users/marco/Desktop/dati_per_backup");

   let pid = std::process::id(); // Usa l'ID del processo corrente per testare

   // Avvia il processo di monitoraggio della CPU in maniera parallela rispetto alla funzionalità di backup
   thread::spawn(move||{
      log_with_tick(log_dir.as_path(), pid as i32).unwrap();
   });


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
      if enabled {
         if shape_recognizer("cerchio", Arc::clone(&state), logical_width, logical_height, true) {
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

               backup_execute( &"/Volumes/ESD-USB".to_string() , &PathBuf::from("/Users/marco/Desktop/dati_per_backup"), &vec!["pdf".to_string()] ).expect("errore");
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

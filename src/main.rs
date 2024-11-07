use crate::backup::backup_execute;
use std::sync::{Arc, Mutex, mpsc};
use std::{thread};
use std::any::Any;
use std::collections::VecDeque;
use std::path::Path;
use std::process::Command;
use std::time::{Duration};
use iced::{Application, Sandbox};
use winit::event_loop::EventLoop;
use crate::beep::play_beep;
use crate::model::menuGui::{ MyApp };
use crate::shapeRecognize::shape_recognizer;
mod model;
use model::MouseState::MouseState;
use crate::log::log_with_tick;

mod shapeRecognize;
mod backup;
mod log;
mod mainBackground;
mod uninstallBackground;
mod beep;
mod gui;
mod countdownGui;

fn main() {

   let value = MyApp::get_value();

   let pid = std::process::id(); // Usa l'ID del processo corrente per testare

   // Avvia il processo di monitoraggio della CPU in maniera parallela rispetto alla funzionalità di backup
   thread::spawn(move||{
      log_with_tick(Path::new(&value.text_directory_log), pid as i32).unwrap();
   });


   let state = Arc::new(Mutex::new(MouseState {
      sides: [false; 4],
      points: VecDeque::new()
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
      let value = MyApp::get_value();


      if enabled {
         if shape_recognizer(Arc::new(value.radio_segno_avvio.unwrap().clone()), Arc::clone(&state), logical_width, logical_height, true) {
            if !first_recognition_done {
               play_beep(Duration::from_millis(100), 440.0); // Bip corto
               first_recognition_done = true;
            }

            println!("primo segno riconosciuto ");

            let popup = Command::new("target/debug/popup_gui").spawn();
            thread::sleep(Duration::from_millis(250));

            if shape_recognizer(Arc::new(value.radio_segno_conferma.unwrap()), Arc::clone(&state), logical_width, logical_height, false) {
               popup.unwrap().kill().expect("problema kill failed");
               play_beep(Duration::from_millis(500), 440.0); // Bip lungo
               println!("secondo segno riconosciuto");
               enabled = false;

               let mut vec_filter = Vec::new();

               if (value.check_music==false && value.check_doc==false && value.check_img==false && value.check_video==false){
                  vec_filter.push("all".to_string());
               }else {
                  if value.check_video{
                     vec_filter.push("mp4".to_string());
                     vec_filter.push("avi".to_string());
                     vec_filter.push("mov".to_string());
                  }
                  if value.check_doc{
                     vec_filter.push("pdf".to_string());
                     vec_filter.push("docx".to_string());
                     vec_filter.push("xlsx".to_string());
                     vec_filter.push("pptx".to_string());

                  }
                  if value.check_img{
                     vec_filter.push("png".to_string());
                     vec_filter.push("jpg".to_string());
                     vec_filter.push("gif".to_string());
                  }
                  if value.check_music{
                     vec_filter.push("mp3".to_string());
                     vec_filter.push("wav".to_string());
                  }

               }
              // println!("{:?}", vec_filter);
               backup_execute( &value.text_drive_destinazione , &value.text_cartella_sorgente, &vec_filter ).expect("errore nel backup");
               enabled = true;
               first_recognition_done = false; // Resetta il flag per riconoscere di nuovo
            } else {
               println!("timer scaduto, ripartire dal primo segno");
               play_beep(Duration::from_millis(500), 220.0); // Bip errore
               first_recognition_done = false;
            }
         }
      }else {
         println!("riconoscimento non attivo, azione in corso")
      }
   }
}
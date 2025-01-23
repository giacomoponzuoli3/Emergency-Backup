use crate::backup::backup_execute;
use std::sync::{Arc, Mutex};
use std::{thread};
use std::collections::VecDeque;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration};
use winit::dpi::PhysicalSize;
use winit::event_loop::EventLoop;
use crate::beep::play_beep;
use crate::model::menu_gui::{ MyApp };
use crate::shape_recognize::shape_recognizer;
mod model;
use model::mouse_state::MouseState;
use crate::log::log_with_tick;
use crate::model::path_base::get_base_path;

mod shape_recognize;
mod backup;
mod log;

mod uninstall_background;
mod beep;
mod gui;
mod countdown_gui;

fn main() {

   let value = MyApp::get_value();

   let pid = std::process::id(); // prendi l'ID del processo corrente per fare monitoring

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
   let mut size = PhysicalSize::new(  1920, 1080);
   let mut scale_factor :f64 =1.0;

   let primary_monitor = event_loop.primary_monitor();
   match primary_monitor {
      Some(monitor) => {
         // Recupera le dimensioni fisiche del monitor
         size = monitor.size();
         scale_factor = monitor.scale_factor();
      },
      None => {
         // RSe non c'è un monitor, usa le dimensioni inserite staticamente
         println!("No Monitor found");
      }
   }



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

            println!("Primo segno riconosciuto: {:?}", value.radio_segno_avvio);

            //desktop path ../Desktop
            let desktop_path = dirs::desktop_dir()
                .expect("Impossibile ottenere la cartella Desktop");

            //base path per tutti gli eseguibili è ../Desktop/Group-35/release
            let base_path: PathBuf = desktop_path
                .join("Group-35")// Aggiungi la cartella "Group-35"
                .join("release");   // Aggiungi il file specificato

            let mut path_popup_gui= match get_base_path(&base_path) {
               Some(path) => path,
               None => return, // Esci se il sistema operativo non è supportato
            };

            path_popup_gui = path_popup_gui.join("popup_gui");

            // Se il sistema è Windows, aggiungi l'estensione ".exe"
            #[cfg(windows)]
            {
               path_popup_gui.set_extension("exe");
            }

            println!("Path popup_gui: {:?}", path_popup_gui);

            let popup = Command::new(path_popup_gui).spawn();
            thread::sleep(Duration::from_millis(250));

            if shape_recognizer(Arc::new(value.radio_segno_conferma.unwrap()), Arc::clone(&state), logical_width, logical_height, false) {
               popup.unwrap().kill().expect("finestra timer non chiusa correttamente");
               play_beep(Duration::from_millis(500), 440.0); // Bip lungo
               println!("Secondo segno riconosciuto: {:?}", value.radio_segno_conferma);
               enabled = false;

               let mut vec_filter = Vec::new();
               // definisci le estensioni per filtrare i dati da includere nel backup
               if value.check_music==false && value.check_doc==false && value.check_img==false && value.check_video==false{
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

               backup_execute( &value.text_drive_destinazione , &value.text_cartella_sorgente, &vec_filter ).expect("errore nel backup");
               enabled = true;
               first_recognition_done = false; // Resetta il flag per riconoscere di nuovo
            } else {
               println!("Il timer è scaduto, si riparte dal riconoscimento del primo segno.");
               play_beep(Duration::from_millis(500), 220.0); // Bip errore
               first_recognition_done = false;
            }
         }
      }else {
         println!("Riconoscimento non attivo, errore.")
      }
   }
}
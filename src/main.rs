use winit::event_loop::EventLoop;
use crate::backup::backup_execute;
use rdev::{listen, EventType, Event};
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::collections::VecDeque;
use std::time::{Duration, Instant};
use crate::beep::play_beep;


use crate::shapeRecognize::shape_recognizer;

mod shapeRecognize;
mod backup;
mod log;
mod mainBackground;
mod uninstallBackground;
mod beep;

struct MouseState {
   sides: [bool; 4], // [top, bottom, left, right]
   points: VecDeque<(f64, f64)>, // Punti del mouse per cerchio, rettangolo e segno -
   recognized_shape: Option<String>, // Forma riconosciuta
}

fn main() {
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
   let mut last_recognition_time = Instant::now();
   let mut enabled= true;

   loop {
      if enabled {
         if shape_recognizer("cerchio", Arc::clone(&state), logical_width, logical_height, true) {
            if !first_recognition_done {
               play_beep(Duration::from_millis(100)); // Bip corto
               first_recognition_done = true;
               println!("1");
            }

            if shape_recognizer("rettangolo", Arc::clone(&state), logical_width, logical_height, false) {
               play_beep(Duration::from_millis(500)); // Bip lungo
               println!("backup");
               enabled = false;
               backup_execute().expect("errore");
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

use iced::{Application, Settings};
use crate::backup::backup_execute;

use crate::shapeRecognize::shape_recognizer;

mod shapeRecognize;
mod backup;
mod log;
mod mainBackground;
mod uninstallBackground;
mod gui;

fn main() {
    //shape_recognizer()
    //backup_execute();
    //log_with_tick();

    gui::MyApp::run(Settings {
        window: iced::window::Settings {
            size: iced::Size::new(800 as f32, 700 as f32),  // Imposta la dimensione della finestra
            resizable: true,   // Permette di ridimensionare la finestra
            ..Default::default()
        },
        ..Settings::default()
    }).unwrap();
}

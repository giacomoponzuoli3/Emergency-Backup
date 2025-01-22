use iced::{Application, Settings};

//mod model;
use crate::model::countdown::App;

pub fn main() -> iced::Result {
    App::run(Settings {
        window: iced::window::Settings {
            size: iced::Size::new(500 as f32, 100 as f32),  // Imposta la dimensione della finestra
            resizable: true,   // Permette di ridimensionare la finestra
            ..Default::default()
        },
        ..Settings::default()
    }).expect("Errore generazione finestra con timer");
    Ok(())
}

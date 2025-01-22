use std::thread::sleep;
use iced::{
    Application, Command, Element, Subscription, executor,
    widget::{Text, Column}, time::Duration, Theme,
};
use iced::futures::{stream};

/// Enum per rappresentare i messaggi dell'applicazione
#[derive(Debug, Clone, Copy)]
pub enum Message {
    TimerTick(u32),  // Aggiungi un messaggio con il valore corrente del countdown
}

/// Struttura principale dell'applicazione
pub struct App {
    time_left: u32,
}

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    ///Inizializza l'applicazione e imposta il countdown iniziale a 10 secondi
    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            App {
                time_left: 10,  // Imposta il countdown iniziale a 10 secondi
            },
            Command::none(),
        )
    }

    /// Titolo della finestra dell'applicazione
    fn title(&self) -> String {
        String::from("Countdown Timer")
    }

    /// Aggiorna lo stato dell'applicazione in base al messaggio ricevuto
    /// - `Message::TimerTick`: Aggiorna il tempo rimanente con il nuovo valore fornito
    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::TimerTick(new_time_left) => {
                self.time_left = new_time_left;
                Command::none()
            }
        }
    }

    ///Costruisce e mostra il tempo rimanente nel countdown
    fn view(&self) -> Element<Message> {
        Column::new()
            .push(Text::new(format!("Tempo rimasto: {} secondi", self.time_left)).size(40))
            .padding(23)
            .into()

    }

    /// Genera un flusso che invia un messaggio ogni secondo fino a quando il countdown raggiunge zero
    fn subscription(&self) -> Subscription<Message> {
        iced::subscription::run(move || {
            // Inizia il countdown da 10 secondi
            stream::unfold(10, |countdown| async move {
                if countdown > 0 {
                    // Aspetta un secondo
                    sleep(Duration::from_secs(1));
                    // Manda il valore attuale del countdown a `update`
                    Some((Message::TimerTick(countdown - 1), countdown - 1))
                } else {
                    // Manda il messaggio per chiudere la finestra quando il countdown finisce
                    std::process::exit(0);
                }
            })
        })
    }
}
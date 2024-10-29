use std::thread::sleep;
use iced::{
    Application, Command, Element, Settings, Subscription, executor,
    widget::{Text, Column}, time::Duration, Alignment, Theme,
};
use std::time::Instant;
use iced::futures::{stream, StreamExt};

#[derive(Debug, Clone, Copy)]
pub enum Message {
    TimerTick(u32),  // Aggiungi un messaggio con il valore corrente del countdown
}

pub(crate) struct App {
    time_left: u32,
}

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            App {
                time_left: 10,  // Imposta il countdown iniziale a 10 secondi
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Countdown Timer")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::TimerTick(new_time_left) => {
                self.time_left = new_time_left;
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
            Column::new()
                .push(Text::new(format!("Tempo rimasto: {} secondi", self.time_left)).size(20))
                .align_items(Alignment::Center)
                .into()

    }

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

pub fn main() -> iced::Result {
    App::run(Settings::default())
}

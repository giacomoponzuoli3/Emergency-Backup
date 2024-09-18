use std::error::Error;
use std::fs::File;
use std::path::Path;
use csv::{Reader, Writer};
use iced::widget::{button, column, radio, row, text, text_input, };
use iced::{Alignment, Element, Sandbox, Settings};
use rfd::FileDialog;
use rfd::MessageDialog;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MyApp {
    radio_bootstrap: Option<Bootstrap>,
    text_cartella_sorgente: String,
    text_drive_destinazione: String,
    text_directory_log: String,
    radio_segno_avvio: Option<Segno>,
    radio_segno_conferma: Option<Segno>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Bootstrap{
    Positivo,
    Negativo
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Segno{
    Rettangolo,
    Cerchio
}

#[derive(Debug, Clone)]
pub enum Message {
    InputCartellaSorgente(String),
    InputDriveDestinazione(String),
    InputDirectoryLog(String),
    ButtonCartellaSorgente,
    ButtonDriveDestinazione,
    ButtonDirectoryLog,
    ButtonSalva,
    BootstrapSelected(Bootstrap),
    SegnoSelectedAvvio(Segno),
    SegnoSelectedConferma(Segno)
}

impl MyApp {
    fn load_from_csv(file_path: &str) -> Result<MyApp, Box<dyn Error>> {
        // Apre il file
        let file = File::open(file_path)?;

        // Crea un lettore CSV
        let mut rdr = Reader::from_reader(file);

        // Deserializza il primo record come MyApp
        let app = rdr.deserialize().next().ok_or("Errore nella lettura del CSV")??;

        Ok(app)
    }
}

impl Default for MyApp{
    fn default() -> Self {
        MyApp{
            radio_bootstrap : Some(Bootstrap::Negativo),
            text_cartella_sorgente : dirs::desktop_dir()
                .map(|path| path.to_string_lossy().to_string())
                .unwrap_or_else(|| "".to_string()),
            text_drive_destinazione: dirs::desktop_dir()
                .map(|path| path.to_string_lossy().to_string()) // Se esiste, ottieni il percorso come stringa
                .unwrap_or_else(|| "".to_string()),                                   // Altrimenti, imposta una stringa vuota
            text_directory_log: dirs::desktop_dir()
                .map(|path| path.to_string_lossy().to_string())
                .unwrap_or_else(|| "".to_string()),
            radio_segno_avvio: Some(Segno::Rettangolo),
            radio_segno_conferma: Some(Segno::Cerchio),
        }
    }
}

impl Sandbox for MyApp {
    type Message = Message;

    fn new() -> Self {
        if let Some(app) = MyApp::load_from_csv("output.csv").ok() {
            return app;
        }

        Self::default()
    }

    fn title(&self) -> String {
        String::from("Impostazioni Backup")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::InputCartellaSorgente(value) => {
                self.text_cartella_sorgente = value;
            }
            Message::InputDriveDestinazione(value) => {
                self.text_drive_destinazione = value;
            }
            Message::InputDirectoryLog(value) => {
                self.text_directory_log = value;
            }
            Message::ButtonCartellaSorgente => {
                if let Some(path) = FileDialog::new().pick_folder() {
                    self.text_cartella_sorgente = path.display().to_string();
                }
            }
            Message::ButtonDriveDestinazione => {
                if let Some(path) = FileDialog::new().pick_folder() {
                    self.text_drive_destinazione = path.display().to_string();
                }
            }
            Message::ButtonDirectoryLog => {
                if let Some(path) = FileDialog::new().pick_folder() {
                    self.text_directory_log = path.display().to_string();
                }
            }
            Message::BootstrapSelected(bootstrap) => {
                self.radio_bootstrap = Some(bootstrap);
            }
            Message::SegnoSelectedAvvio(Segno) => {
                self.radio_segno_avvio = Some(Segno);
            }
            Message::SegnoSelectedConferma(Segno) => {
                self.radio_segno_conferma = Some(Segno);
            }
            Message::ButtonSalva =>{
                //validazioni
                let mut flag = 0;
                if self.radio_bootstrap.is_none() || self.text_cartella_sorgente.is_empty() || self.text_drive_destinazione.is_empty()
                   || self.text_directory_log.is_empty()  || self.radio_segno_conferma.is_none() || self.radio_segno_avvio.is_none(){
                    MessageDialog::new()
                        .set_title("Errore")
                        .set_description("Compilare tutti i campi per poter proseguire")
                        .set_buttons(rfd::MessageButtons::Ok)
                        .show();

                    flag = 1;
                }
                let p = Path::new(&self.text_cartella_sorgente);
                if !p.exists(){
                    MessageDialog::new()
                        .set_title("Errore")
                        .set_description("Inserire un path corretto per la cartella sorgente")
                        .set_buttons(rfd::MessageButtons::Ok)
                        .show();

                    flag = 1;
                }

                let p = Path::new(&self.text_drive_destinazione);
                if !p.exists(){
                    MessageDialog::new()
                        .set_title("Errore")
                        .set_description("Inserire un path corretto per la cartella destinazione")
                        .set_buttons(rfd::MessageButtons::Ok)
                        .show();

                    flag = 1;
                }

                let p = Path::new(&self.text_directory_log);
                if !p.exists(){
                    MessageDialog::new()
                        .set_title("Errore")
                        .set_description("Inserire un path corretto per la cartella log")
                        .set_buttons(rfd::MessageButtons::Ok)
                        .show();

                    flag = 1;
                }

                if flag == 0{
                    let file = File::create("output.csv").expect("Non posso creare il file CSV");
                    let mut wtr = Writer::from_writer(file);

                    wtr.serialize(self).expect("Non posso scrivere i dati nel file CSV");
                    wtr.flush().expect("Non posso salvare i dati nel file");

                    MessageDialog::new()
                        .set_title("Successo")
                        .set_description("Dati salvati con successo!")
                        .set_buttons(rfd::MessageButtons::Ok)
                        .show();

                    std::process::exit(0);
                }
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let inputCartellaSorgente = text_input("Enter something...", &self.text_cartella_sorgente)
            .on_input(Message::InputCartellaSorgente);

        let btnCartellaSorgente = button("Seleziona").on_press(Message::ButtonCartellaSorgente);

        let inputDriveDestinazione = text_input("Enter something...", &self.text_drive_destinazione)
            .on_input(Message::InputDriveDestinazione);

        let btnDriveDestinazione = button("Seleziona").on_press(Message::ButtonDriveDestinazione);

        let inputDirectoryLog = text_input("Enter directory...", &self.text_directory_log)
            .on_input(Message::InputDirectoryLog);

        let btnDirectoryLog = button("Seleziona").on_press(Message::ButtonDirectoryLog);

        let btnSalva = button("Salva").on_press(Message::ButtonSalva);

        let radioBootstrap = row![
            radio("Si", Bootstrap::Positivo, self.radio_bootstrap, Message::BootstrapSelected),
            radio("No", Bootstrap::Negativo, self.radio_bootstrap, Message::BootstrapSelected)
        ]
            .spacing(20);

        let radioSegnoAvvio = row![
            radio("Rettangolo", Segno::Rettangolo, self.radio_segno_avvio, Message::SegnoSelectedAvvio),
            radio("Cerchio", Segno::Cerchio, self.radio_segno_avvio, Message::SegnoSelectedAvvio)
        ]
            .spacing(20);

        let radioSegnoConferma = row![
            radio("Rettangolo", Segno::Rettangolo, self.radio_segno_conferma, Message::SegnoSelectedConferma),
            radio("Cerchio", Segno::Cerchio, self.radio_segno_conferma, Message::SegnoSelectedConferma)
        ]
            .spacing(20);

        let riga1 = row![
            inputCartellaSorgente
                .width(500),
            btnCartellaSorgente
        ]
            .spacing(20);

        let riga2 = row![
            inputDriveDestinazione
                .width(500),
            btnDriveDestinazione
        ]
            .spacing(20);

        let riga3 = row![
            inputDirectoryLog
                .width(500),
            btnDirectoryLog
        ]
            .spacing(20);

        let content = column![
            text("IMPOSTAZIONI BACKUP").size(30),
            text("Avvio del tool in fase di bootstrap:"),
            radioBootstrap,
            text("Seleziona una cartella sorgente"),
            riga1,
            text("Seleziona il drive di destinazione"),
            riga2,
            text("Scegliere il segno per avviare il backup"),
            radioSegnoAvvio,
            text("Scegliere il segno per confermare il backup"),
            radioSegnoConferma,
            text("Selezionare la cartella dove salvare il log di sistema"),
            riga3,
            btnSalva
        ]
            .padding(20)
            .spacing(20)
            .align_items(Alignment::Start);

        content.into()
    }
}

pub fn main() -> iced::Result {
    MyApp::run(Settings {
      window: iced::window::Settings {
         size: iced::Size::new(800 as f32, 700 as f32),  // Imposta la dimensione della finestra
         resizable: true,   // Permette di ridimensionare la finestra
         ..Default::default()
      },
      ..Settings::default()
   }).expect("Errore");


    MessageDialog::new()
        .set_title("Errore")
        .set_description("Il campo cartella sorgente è obbligatorio!")
        .set_buttons(rfd::MessageButtons::Ok)
        .show();
    Ok(())
}
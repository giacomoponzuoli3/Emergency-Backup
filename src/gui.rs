use std::error::Error;
use std::fs::File;
use std::path::Path;
use csv::{Reader, Writer};
use iced::widget::{button, checkbox, column, radio, row, text, text_input};
use iced::{Alignment, Element, Sandbox, Settings};
use iced::theme::Checkbox;
use rfd::FileDialog;
use rfd::MessageDialog;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct MyApp {
    text_cartella_sorgente: String,
    text_drive_destinazione: String,
    text_directory_log: String,
    radio_segno_avvio: Option<Segno>,
    radio_segno_conferma: Option<Segno>,
    check_img: bool,
    check_video: bool,
    check_music: bool,
    check_doc: bool
}

#[derive(Serialize, Deserialize)]
pub struct OutputValue{
    pub text_cartella_sorgente: String,
    pub text_drive_destinazione: String,
    pub text_directory_log: String,
    pub radio_segno_avvio: Option<Segno>,
    pub radio_segno_conferma: Option<Segno>,
    pub check_img: bool,
    pub check_video: bool,
    pub check_music: bool,
    pub check_doc: bool
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Bootstrap{
    Positivo,
    Negativo
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Segno{
    Rettangolo,
    Cerchio,
    Meno
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
    SegnoSelectedAvvio(Segno),
    SegnoSelectedConferma(Segno),
    CheckboxImg,
    CheckboxVideo,
    CheckboxMusic,
    CheckboxDoc,
}

impl MyApp {
    pub fn load_from_csv(file_path: &str) -> Result<MyApp, Box<dyn Error>> {
        // Apre il file
        let file = File::open(file_path)?;

        // Crea un lettore CSV
        let mut rdr = Reader::from_reader(file);

        // Deserializza il primo record come MyApp
        let app = rdr.deserialize().next().ok_or("Errore nella lettura del CSV")??;

        Ok(app)
    }

    pub fn get_value() -> OutputValue {
        if let Some(app) = MyApp::load_from_csv("output.csv").ok() {
            return OutputValue{
                text_cartella_sorgente: app.text_cartella_sorgente,
                text_drive_destinazione: app.text_drive_destinazione,
                text_directory_log: app.text_directory_log,
                radio_segno_avvio: app.radio_segno_avvio,
                radio_segno_conferma: app.radio_segno_conferma,
                check_img: app.check_img,
                check_doc:app.check_doc,
                check_music: app.check_music,
                check_video: app.check_video,
            };
        }

        let file = File::create("output.csv").expect("Non posso creare il file CSV");
        let mut wtr = Writer::from_writer(file);

       let def = OutputValue{
            text_cartella_sorgente: Self::default().text_cartella_sorgente,
            text_drive_destinazione: Self::default().text_drive_destinazione,
            text_directory_log: Self::default().text_directory_log,
            radio_segno_avvio: Self::default().radio_segno_avvio,
            radio_segno_conferma: Self::default().radio_segno_conferma,
            check_video: Self::default().check_video,
            check_img: Self::default().check_img,
            check_doc: Self::default().check_doc,
            check_music: Self::default().check_music
        };
        wtr.serialize(&def).expect("Non posso scrivere i dati nel CSV");
        wtr.flush().expect("Non posso salvare i dati nel CSV");
        def
    }
}

impl Default for MyApp{
    fn default() -> Self {
        MyApp{
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
            radio_segno_conferma: Some(Segno::Rettangolo),
            check_music: false,
            check_doc: false,
            check_img:false,
            check_video:false
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
            Message::SegnoSelectedAvvio(Segno) => {
                self.radio_segno_avvio = Some(Segno);
            }
            Message::SegnoSelectedConferma(Segno) => {
                self.radio_segno_conferma = Some(Segno);
            }
            Message::CheckboxVideo =>{
                self.check_video = !self.check_video
            }
            Message::CheckboxDoc =>{
                self.check_doc = !self.check_doc
            }
            Message::CheckboxImg =>{
                self.check_img = !self.check_img
            }
            Message::CheckboxMusic =>{
                self.check_music = !self.check_music
            }

            Message::ButtonSalva =>{
                //validazioni
                let mut flag = 0;
                if self.text_cartella_sorgente.is_empty() || self.text_drive_destinazione.is_empty()
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
        let input_cartella_sorgente = text_input("Enter something...", &self.text_cartella_sorgente)
            .on_input(Message::InputCartellaSorgente);

        let btn_cartella_sorgente = button("Seleziona").on_press(Message::ButtonCartellaSorgente);

        let input_drive_destinazione = text_input("Enter something...", &self.text_drive_destinazione)
            .on_input(Message::InputDriveDestinazione);

        let btn_drive_destinazione = button("Seleziona").on_press(Message::ButtonDriveDestinazione);

        let input_directory_log = text_input("Enter directory...", &self.text_directory_log)
            .on_input(Message::InputDirectoryLog);

        let btn_directory_log = button("Seleziona").on_press(Message::ButtonDirectoryLog);

        let ck_img = checkbox("Immagini", self.check_img).on_toggle(|_| Message::CheckboxImg);

        let ck_video = checkbox("Video", self.check_video).on_toggle(|_| Message::CheckboxVideo);

        let ck_music = checkbox("Music", self.check_music).on_toggle(|_| Message::CheckboxMusic);

        let ck_doc = checkbox("Doc", self.check_doc).on_toggle(|_| Message::CheckboxDoc);

        let btn_salva = button("Salva").on_press(Message::ButtonSalva);


        let radio_segno_avvio = row![
            radio("Rettangolo", Segno::Rettangolo, self.radio_segno_avvio, Message::SegnoSelectedAvvio),
            radio("Cerchio", Segno::Cerchio, self.radio_segno_avvio, Message::SegnoSelectedAvvio)
        ]
            .spacing(20);

        let radio_segno_conferma = row![
            radio("Rettangolo", Segno::Rettangolo, self.radio_segno_conferma, Message::SegnoSelectedConferma),
            radio("Cerchio", Segno::Cerchio, self.radio_segno_conferma, Message::SegnoSelectedConferma)
        ]
            .spacing(20);

        let riga1 = row![
            input_cartella_sorgente
                .width(500),
            btn_cartella_sorgente
        ]
            .spacing(20);

        let riga2 = row![
            input_drive_destinazione
                .width(500),
            btn_drive_destinazione
        ]
            .spacing(20);

        let riga3 = row![
            input_directory_log
                .width(500),
            btn_directory_log
        ]
            .spacing(20);

        let riga4 =  row![
            ck_video,
            ck_img,
            ck_doc,
            ck_music
        ]
            .spacing(20);

        let content = column![
            text("IMPOSTAZIONI BACKUP").size(30),
            text("Seleziona una cartella sorgente"),
            riga1,
            text("Seleziona il drive di destinazione"),
            riga2,
            text("Scegliere il segno per avviare il backup"),
            radio_segno_avvio,
            text("Scegliere il segno per confermare il backup"),
            radio_segno_conferma,
            text("Selezionare la cartella dove salvare il log di sistema"),
            riga3,
            riga4,
            btn_salva
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
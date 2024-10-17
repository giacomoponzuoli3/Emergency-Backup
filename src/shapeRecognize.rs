use crate::MouseState;
use rdev::{listen, EventType, Event};
use std::sync::{Arc, Mutex};
use std::thread;
use std::collections::VecDeque;
use std::time::{Duration, Instant};
use iced::futures::SinkExt;
use crate::gui::Segno;

pub(crate) fn shape_recognizer(shape: Arc<Segno>, state: Arc<Mutex<MouseState>>, logical_width: f64, logical_height: f64, is_first_recognize: bool) -> bool {
    let shape_ref = Arc::clone(&shape);
    let result = Arc::new(Mutex::new(false));
    let start_time = Instant::now(); // Tempo di avvio

    // Thread per ascoltare gli eventi del mouse
    let result_clone = Arc::clone(&result);
    let state_clone = Arc::clone(&state);

    thread::spawn(move || {
        let _ = std::panic::catch_unwind(|| {
            listen(move |event: Event| {
                let mut state = state_clone.lock().unwrap();
                const TOLERANCE: f64 = 5.0;
                const CIRCLE_TOLERANCE: f64 = 65.0;
                const LINE_TOLERANCE: f64 = 35.0;

                match event.event_type {
                    EventType::MouseMove { x, y } => {
                        state.points.push_back((x, y));



                        match shape_ref.as_ref() {
                            Segno::Cerchio => {
                                if state.points.len() > 5 {
                                    if check_circle(&mut state.points, CIRCLE_TOLERANCE).is_some() {
                                        *result_clone.lock().unwrap() = true;
                                    }
                                }
                            },
                            Segno::Rettangolo => {
                                if check_edges(&mut state, x, y, logical_width, logical_height, TOLERANCE).is_some() {
                                    *result_clone.lock().unwrap() = true;
                                }
                            },
                            Segno::Meno => {
                                if state.points.len() > 2 {

                                    if check_horizontal_line(&mut state.points, LINE_TOLERANCE).is_some() {
                                        *result_clone.lock().unwrap() = true;
                                    }
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }).unwrap();
        });
    });

    // Attendere la risposta dal thread
    loop {
        // Verifica se è passato il tempo di timeout
        if !is_first_recognize && start_time.elapsed() > Duration::from_secs(10) {
            return false; // Timeout scaduto
        }

        if *result.lock().unwrap() {
            return true; // Riconoscimento riuscito
        }

        thread::sleep(Duration::from_millis(100)); // Pausa breve per evitare un loop di polling intensivo
    }
}

// Funzioni di controllo per cerchio, rettangolo e segno
fn check_edges(state: &mut MouseState, x: f64, y: f64, width: f64, height: f64, tolerance: f64) -> Option<()> {
    if y <= tolerance {
        state.sides[0] = true;
    }
    if y >= height - tolerance {
        state.sides[1] = true;
    }
    if x <= tolerance {
        state.sides[2] = true;
    }
    if x >= width - tolerance {
        state.sides[3] = true;
    }

    if state.sides.iter().all(|&side| side) {
        state.sides = [false; 4];
        return Some(());
    }
    None
}

// Funzione migliorata per il riconoscimento del cerchio
/*
fn check_circle(points: &mut VecDeque<(f64, f64)>, tolerance: f64) -> Option<()> {
    if points.len() < 5 {
        return None;
    }

    // Definisci un raggio minimo locale
    const MIN_RADIUS: f64 = 50.0;

    // Stima un centro approssimativo usando il primo e l'ultimo punto
    let first_point = points.front().unwrap();
    let last_point = points.back().unwrap();
    let center_x = (first_point.0 + last_point.0) / 2.0;
    let center_y = (first_point.1 + last_point.1) / 2.0;

    // Calcola il raggio approssimativo
    let radius = ((first_point.0 - center_x).powi(2) + (first_point.1 - center_y).powi(2)).sqrt();
   // println!("{:?}", radius);
    // Controllo sul raggio minimo
    if radius < MIN_RADIUS {

        points.clear();
        return None;
    }

    // Verifica che tutti i punti siano a una distanza approssimativamente uguale al raggio
    for &(x, y) in points.iter() {
        let distance = ((x - center_x).powi(2) + (y - center_y).powi(2)).sqrt();
        if (distance - radius).abs() > tolerance {
            points.clear();
            println!("dist {:?} raggio {:?} diff {:?} ", distance, radius, (distance - radius).abs());
            return None;
        }
    }

    points.clear(); // Se il cerchio è riconosciuto, puliamo i punti

    Some(())
}
*/
fn check_circle(points: &mut VecDeque<(f64, f64)>, tolerance: f64) -> Option<()> {
    if points.len() < 5 {
        return None;
    }

    // Definisci un raggio minimo locale
    const MIN_RADIUS: f64 = 20.0;

    // Stima un centro approssimativo usando il primo e l'ultimo punto
    let first_point = points.front().unwrap();
    let last_point = points.back().unwrap();
    let center_x = (first_point.0 + last_point.0) / 2.0;
    let center_y = (first_point.1 + last_point.1) / 2.0;

    // Calcola il raggio approssimativo
    let radius = ((first_point.0 - center_x).powi(2) + (first_point.1 - center_y).powi(2)).sqrt();

    // Controllo sul raggio minimo
    if radius < MIN_RADIUS {
        points.clear();
        return None;
    }

    // Tolleranza relativa: 5% del raggio
    let relative_tolerance = tolerance * radius / 100.0;

    // Verifica che tutti i punti siano a una distanza approssimativamente uguale al raggio
    for &(x, y) in points.iter() {
        let distance = ((x - center_x).powi(2) + (y - center_y).powi(2)).sqrt();
        if (distance - radius).abs() > relative_tolerance {
            //println!("dist {:?} raggio {:?} diff {:?} relat {:?}", distance, radius, (distance - radius).abs(), relative_tolerance);
            points.clear();
            return None;
        }
    }

    points.clear(); // Se il cerchio è riconosciuto, puliamo i punti
    Some(())
}

fn check_horizontal_line(points: &mut VecDeque<(f64, f64)>, tolerance: f64) -> Option<()> {
    const MIN_LINE_LENGTH: f64 = 150.0; // Lunghezza minima della linea

    if points.len() < 2 {
        return None; // Non ci sono abbastanza punti per verificare una linea
    }

    let (x_start, y_start) = points.front()?;
    let (x_end, y_end) = points.back()?;

    // Verifica se la differenza in altezza (y) è entro la tolleranza
    if (y_end - y_start).abs() <= tolerance {
        // Verifica che la lunghezza della linea sia almeno il valore minimo
        if (x_end - x_start).abs() >= MIN_LINE_LENGTH {
            return Some(()); // La linea è orizzontale e sufficientemente lunga
        }
    }
    points.clear();
    None // La differenza supera la tolleranza o la linea non è abbastanza lunga
}


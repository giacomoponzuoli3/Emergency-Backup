use crate::MouseState;
use rdev::{listen, EventType, Event};
use std::sync::{Arc, Mutex};
use std::thread;
use std::collections::VecDeque;
use std::time::{Duration, Instant};
use crate::gui::Segno;

pub(crate) fn shape_recognizer(shape: Arc<Segno>, state: Arc<Mutex<MouseState>>, logical_width: f64, logical_height: f64, is_first_recognize:bool) -> bool {
    let shape_ref = Arc::clone(&shape);

    let result = Arc::new(Mutex::new(false));
    let start_time = Instant::now(); // Tempo di avvio

    // Thread per ascoltare gli eventi del mouse
    let result_clone = Arc::clone(&result);

    thread::spawn(move || {
        let _ = std::panic::catch_unwind(|| {
            listen(move |event: Event| {
                let mut state = state.lock().unwrap();
                const TOLERANCE: f64 = 5.0;
                const CIRCLE_TOLERANCE: f64 = 23.0;

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
                                _ => {}
                            }

                    }
                    _ => {}
                }
            }).unwrap();
        });
    });

    // Attendere la risposta dal thread
    loop {

            //println!("{:?}", start_time.elapsed());
            // Verifica se è passato il tempo di timeout
        if !is_first_recognize {
            if start_time.elapsed() > Duration::from_secs(10) {
                return false; // Timeout scaduto
            }
        }


        if *result.lock().unwrap() {
            return true; // Riconoscimento riuscito
        }

        thread::sleep(Duration::from_millis(100)); // Pausa breve per evitare un loop di polling intensivo
    }
}

// Funzioni di controllo per cerchio, rettangolo e segno -
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

fn check_circle(points: &mut VecDeque<(f64, f64)>, tolerance: f64) -> Option<()> {
    if points.len() < 5 {
        return None;
    }

    let first_point = points.front().unwrap();
    let last_point = points.back().unwrap();
    let distance = ((last_point.0 - first_point.0).powi(2) + (last_point.1 - first_point.1).powi(2)).sqrt();

    if distance <= tolerance {
        points.clear();
        return Some(());
    }
    None
}

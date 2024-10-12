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
                const CIRCLE_TOLERANCE: f64 = 40.0;

                match event.event_type {
                    EventType::MouseMove { x, y } => {
                        state.points.push_back((x, y));


                            match shape_ref.as_ref() {
                                Segno::Cerchio => {
                                    if state.points.len() > 5 {
                                        //thread::sleep(Duration::from_millis(500));
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



/*
fn check_circle(points: &mut VecDeque<(f64, f64)>, tolerance: f64) -> Option<()> {
    if points.len() < 5 {
        return None;
    }

    // 1. Calcolare il centro approssimativo (media delle coordinate)
    let (mut sum_x, mut sum_y) = (0.0, 0.0);
    for &(x, y) in points.iter() {
        sum_x += x;
        sum_y += y;
    }
    let num_points = points.len() as f64;
    let center_x = sum_x / num_points;
    let center_y = sum_y / num_points;

    // 2. Calcolare il raggio medio e determinare il range accettabile
    let mut min_radius = f64::MAX;
    let mut max_radius = f64::MIN;

    for &(x, y) in points.iter() {
        let distance = ((x - center_x).powi(2) + (y - center_y).powi(2)).sqrt();
        min_radius = min_radius.min(distance);
        max_radius = max_radius.max(distance);
    }

    // 3. Verificare che il raggio rientri nel range accettabile
    if (max_radius - min_radius) <= tolerance {
        // 4. Verificare che il primo e l'ultimo punto siano vicini (chiusura del cerchio)
        if let (Some(first_point), Some(last_point)) = (points.front(), points.back()) {
            let closure_distance = ((last_point.0 - first_point.0).powi(2) + (last_point.1 - first_point.1).powi(2)).sqrt();

            if closure_distance <= tolerance {
                points.clear();
                return Some(());
            }
        } else {
            return None; // Se non riesci a ottenere i punti front e back
        }
    }
    points.clear();
    None
}
*/
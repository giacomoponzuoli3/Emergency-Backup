use rdev::{listen, EventType, Event};
use std::sync::{Arc, Mutex};
use std::thread;
use winit::event_loop::{EventLoop, EventLoopWindowTarget};
use winit::window::WindowBuilder;
use winit::dpi::{PhysicalSize, LogicalSize};
use std::collections::VecDeque;

struct MouseState {
    sides: [bool; 4], // [top, bottom, left, right]
    points: VecDeque<(f64, f64)>, // Punti del mouse per cerchio, rettangolo e segno -
}

pub fn shape_recognizer() {
    // Crea un event loop per ottenere la dimensione dello schermo
    let event_loop = EventLoop::new();
    let primary_monitor = event_loop.primary_monitor().unwrap();

    // Recupera le dimensioni fisiche del monitor
    let size = primary_monitor.size(); // Dimensioni fisiche
    let scale_factor = primary_monitor.scale_factor(); // Fattore di scaling

    // Calcola le dimensioni logiche
    let logical_width = (size.width as f64 / scale_factor) as f64;
    let logical_height = (size.height as f64 / scale_factor) as f64;

    let state = Arc::new(Mutex::new(MouseState {
        sides: [false; 4], // Inizializza a false
        points: VecDeque::new(), // Inizializza i punti per cerchio, rettangolo e segno -
    }));

    let state_clone = Arc::clone(&state);

    // Avvia un thread per ascoltare gli eventi del mouse
    thread::spawn(move || {
        listen(move |event: Event| {
            let mut state = state_clone.lock().unwrap();
            const TOLERANCE: f64 = 5.0; // Tolleranza per il rilevamento dei bordi
            const CIRCLE_TOLERANCE: f64 = 25.0; // Tolleranza per riconoscere un cerchio
            const HORIZONTAL_TOLERANCE: f64 = 5.0; // Tolleranza per riconoscere il segno -

            match event.event_type {
                EventType::MouseMove { x, y } => {
                    // Stampa le coordinate del mouse
                   // println!("Coordinate del mouse: ({}, {})", x, y);

                    // Aggiungi il punto alle coordinate
                    state.points.push_back((x, y));

                    // Controlla i bordi dello schermo
                    check_edges(&mut state, x, y, logical_width, logical_height, TOLERANCE);

                    // Controlla se i punti formano un cerchio
                    if state.points.len() > 5 {
                        check_circle(&mut state.points, CIRCLE_TOLERANCE);
                        check_horizontal_line(&mut state.points, HORIZONTAL_TOLERANCE);
                    }
                }
                _ => {}
            }
        }).unwrap();
    });

    // Mantieni il thread principale attivo
    loop {
        thread::park(); // Attende fino a quando non viene risvegliato
    }
}

// Funzione per controllare se il mouse tocca i bordi dello schermo
fn check_edges(state: &mut MouseState, x: f64, y: f64, width: f64, height: f64, tolerance: f64) {
    if y <= tolerance {
        state.sides[0] = true; // Superiore
    }
    if y >= height - tolerance {
        state.sides[1] = true; // Inferiore
    }
    if x <= tolerance {
        state.sides[2] = true; // Sinistro
    }
    if x >= width - tolerance {
        state.sides[3] = true; // Destro
    }

    // Verifica se tutti i lati sono stati toccati
    if state.sides.iter().all(|&side| side) {
        println!("OK");
        state.sides = [false; 4]; // Resetta lo stato
    }
}

// Funzione per verificare se i punti formano un cerchio
fn check_circle(points: &mut VecDeque<(f64, f64)>, tolerance: f64) {
    if points.len() < 5 {
        return;
    }

    let first_point = points.front().unwrap();
    let last_point = points.back().unwrap();

    // Calcola la distanza tra il primo e l'ultimo punto
    let distance = ((last_point.0 - first_point.0).powi(2) + (last_point.1 - first_point.1).powi(2)).sqrt();

    // Se la distanza è entro la tolleranza, considera un cerchio
    if distance <= tolerance {
        println!("Cerchio riconosciuto!");
        points.clear(); // Resetta i punti dopo aver riconosciuto il cerchio
    }
}

// Funzione per verificare se i punti formano una linea orizzontale (-)
fn check_horizontal_line(points: &mut VecDeque<(f64, f64)>, tolerance: f64) {
    if points.len() < 5 {
        return;
    }

    // Calcola le coordinate estreme
    let min_x = points.iter().map(|(x, _)| *x).fold(f64::INFINITY, f64::min);
    let max_x = points.iter().map(|(x, _)| *x).fold(f64::NEG_INFINITY, f64::max);
    let min_y = points.iter().map(|(_, y)| *y).fold(f64::INFINITY, f64::min);
    let max_y = points.iter().map(|(_, y)| *y).fold(f64::NEG_INFINITY, f64::max);

    // Calcola il centro della linea orizzontale
    let horizontal_center = (min_y + max_y) / 2.0;

    // Controlla se ci sono punti nella linea orizzontale
    let horizontal_line = points.iter().filter(|(_, y)| (y - horizontal_center).abs() <= tolerance);

    // Verifica se ci sono punti sufficienti nella linea orizzontale
    if horizontal_line.count() >= 2 {
        println!("Segno - riconosciuto!");
        points.clear(); // Resetta i punti dopo aver riconosciuto il segno -
    }
}

use std::collections::VecDeque;

pub struct MouseState {
    pub sides: [bool; 4], // [top, bottom, left, right]
    pub points: VecDeque<(f64, f64)>, // Punti del mouse per cerchio, rettangolo e segno -
    pub recognized_shape: Option<String>, // Forma riconosciuta
}
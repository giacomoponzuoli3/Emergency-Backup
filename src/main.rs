use crate::backup::backup_execute;
use crate::log::log_with_tick;
use crate::shapeRecognize::shape_recognizer;

mod shapeRecognize;
mod backup;
mod log;
mod mainBackground;
mod uninstallBackground;

fn main() {
    //shape_recognizer()
    backup_execute();
    //log_with_tick();
}

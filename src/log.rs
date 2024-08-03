use std::fs::File;
use std::io::{Error, Write};
use std::time::{Duration, Instant};
use crossbeam_channel::tick;

// ESEMPIO BASE SCRITTURA FILE LOG OGNI 2 MINUTI
// bisogna prendere l'utilizzo di CPU del processo
pub fn log_with_tick() -> Result<(), Error> {

    let mut file = File::create("example.txt")?;
    let start = Instant::now();
    let ticker  = tick(Duration::from_secs(1));
    for i in 0..5 {
        ticker.recv().unwrap();
        println!("Tempo trascorso: {:?}", start.elapsed());

        file.write_all(i.to_string().as_bytes()).unwrap()
    }
    Ok(())

}
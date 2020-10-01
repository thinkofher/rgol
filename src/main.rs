pub mod life;

use std::env;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time;

fn clear() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

fn main() {
    let width = env::args()
        .nth(1)
        .expect("You have to provide width of game.")
        .parse::<u16>()
        .expect("Width has to be positive number.");
    let height = env::args()
        .nth(2)
        .expect("You have to provide height of game.")
        .parse::<u16>()
        .expect("Height has to be positive number.");

    let mut g = life::new_game(width, height);
    life::randomize(&mut g);

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler.");

    let mut s: String;
    while running.load(Ordering::SeqCst) {
        life::advance(&mut g);
        clear();
        s = life::show(&g, width, height);
        println!("{}", s);
        thread::sleep(time::Duration::from_millis(25));
    }
}

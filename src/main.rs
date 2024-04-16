mod engine;

use std::io;
use std::thread;
use std::time::Duration;
use std::sync::mpsc::channel;
use engine::Engine;

fn main() {
    let (tx, rx) = channel();
    let mut engine = Engine::init(&rx);
    engine.listen();

    let handle = thread::spawn(move || {
        loop {
            let mut input = String::new();
            let mut quit = false;
            match io::stdin().read_line(&mut input) {
                Ok(_goes_into_input_above) => {
                    if input == "quit" {
                        quit = true;
                    }
                    let _ = tx.send(input);
                }
                Err(_no_updates_is_fine) => {
    
                }
            }
            if quit {
                break;
            }
            thread::sleep(Duration::from_millis(1));
        }
    });
    
    handle.join();
}
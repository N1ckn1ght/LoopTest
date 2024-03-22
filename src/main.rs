mod engine;

use std::io;
use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use engine::Engine;

const TIME: u128 = 3000; // ms

fn main() {
    let mut engine = Engine::default();

    /* Required states:
        0 - Do nothing
        1 - Clear engine variable
        2 - Spend TIME on thinking and send generated variable
        3 - Just think, do not send a variable
        4 - Send variable NOW and stop thinking, doing so
    */
    
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = Arc::clone(&pair);

    thread::spawn(|| {
        
    });
}
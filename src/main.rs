mod engine;

use std::io;
use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use engine::Engine;

fn main() {
    let mut engine = Engine::default();
    let mut state = 0;

    /* Required states:
        0 | stop    - Do nothing
        1 | clear   - Clear engine variable
        2 | start X - Spend X ms on thinking and send generated variable
        3 | ponder  - Just think, do not send a variable
        4 | now     - Send variable NOW and stop thinking, if doing so
        5 | quit    - Exit program
    */

    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = Arc::clone(&pair);
    thread::spawn(move || {
        let (lock, cvar) = &*pair2;
        let mut started = lock.lock().unwrap();
        *started = true;

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_goes_into_input_above) => {
                let line = input.trim().split(" ").collect::<Vec<&str>>();
                match line[0] {
                    "stop" => {
                        engine.abort = true;
                        state = 0;
                        cvar.notify_one();
                    },
                    "clear" => {
                        engine.variable = 0;
                        state = 1;
                    }
                    "start" => {
                        engine.time_limit_ms = line[1].parse::<u128>().unwrap();
                        state = 2;
                        cvar.notify_one();
                    },
                    "ponder" => {
                        engine.time_limit_ms = 1 << 63;
                        state = 3;
                        cvar.notify_one();
                    },
                    "now" => {
                        engine.abort = true;
                        state = 4;
                        cvar.notify_one();
                    },
                    "quit" => {
                        state = 5;
                        cvar.notify_one();
                    },
                    _ => {
                        println!("Wrong command");
                    }
                }
            },
            Err(_no_updates_is_fine) => {
                println!("No updates");
            }
        }
    });

    loop {
        let &(ref lock, ref cvar) = &*pair;
        let mut started = lock.lock().unwrap();
        while !*started {
            started = cvar.wait(started).unwrap();
        }

        match state {
            0 => {

            },
            1 => {

            },
            2 => {

            },
            3 => {
                println!("Quit signal reached.");
                break;
            },
            _ => {
                println!("Wrong state achieved.");
            }
        }
    }
}
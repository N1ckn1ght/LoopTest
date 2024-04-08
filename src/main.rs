mod engine;

use std::io;
use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use std::time::{Duration, Instant};
use engine::Engine;

enum State {
    Halted,
    Started
}

fn main() {
    let mut engine = Engine::default();
    let mut state = State::Halted;
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_goes_into_input_above) => {
                let line = input.trim().split(" ").collect::<Vec<&str>>();
                match line[0] {
                    "stop" => {
                        match state {
                            State::Halted => {

                            },
                            State::Started => {

                            }
                        }
                    },
                    "clear" => {
                        match state {
                            State::Halted => {

                            },
                            State::Started => {

                            }
                        }
                    },
                    "start" => {
                        match state {
                            State::Halted => {

                            },
                            State::Started => {

                            }
                        }
                    },
                    "ponder" => {
                        match state {
                            State::Halted => {

                            },
                            State::Started => {

                            }
                        }
                    },
                    "now" => {
                        match state {
                            State::Halted => {

                            },
                            State::Started => {

                            }
                        }
                    },
                    "quit" => {
                        match state {
                            State::Halted => {

                            },
                            State::Started => {

                            }
                        }
                    },
                    _ => {
                        
                    }
                }
            }
            Err(_no_updates_is_fine) => {
                println!("No updates");
            }
        }
        thread::sleep(Duration::from_millis(1));
    }
}
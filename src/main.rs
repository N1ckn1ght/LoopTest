mod common;
mod engine;

use std::io;
use std::thread;
use std::time::{Duration, Instant};
use engine::Engine;

pub enum State {
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
                            State::Halted => {},
                            State::Started => {
                                engine.abort = true;
                                thread::sleep(Duration::from_millis(1));
                                state = State::Halted;
                            }
                        }
                    },
                    "clear" => {
                        match state {
                            State::Halted => {
                                engine.variable = 0;
                            },
                            State::Started => {
                                engine.abort = true;
                                thread::sleep(Duration::from_millis(1));
                                engine.variable = 0;
                                state = State::Halted;
                            }
                        }
                    },
                    "start" => {
                        match state {
                            State::Halted => {
                                state = State::Started;
                                let gtl = line[1].parse::<u128>().unwrap();
                                let ntl = gtl * 1000;
                                thread::spawn(move || {
                                    engine.calc(ntl); 
                                });
                            },
                            State::Started => {
                                let gtl = line[1].parse::<u128>().unwrap();
                                let ntl = gtl * 1000;
                                engine.time_limit_ms = ntl;
                                engine.time_start_point = Instant::now();
                            }
                        }
                    },
                    "ponder" => {
                        match state {
                            State::Halted => {
                                state = State::Started;
                                thread::spawn(move || {
                                    engine.calc(1 << 63); 
                                });
                            },
                            State::Started => {
                                engine.time_limit_ms = 1 << 63;
                            }
                        }
                    },
                    "now" => {
                        match state {
                            State::Halted => {
                                println!("Nothing to return!");
                            },
                            State::Started => {
                                engine.abort = true;
                                println!("{}", engine.variable);
                            }
                        }
                    },
                    "quit" => {
                        println!("Shutdown signal reached.");
                        return;
                    },
                    _ => {
                        println!("Unknown command.");
                    }
                }
            }
            Err(_no_updates_is_fine) => {
                println!("No updates...?");
            }
        }
        thread::sleep(Duration::from_millis(1));
        match state {
            State::Halted => {},
            State::Started => {
                if engine.abort {
                    state = State::Halted;
                }
            }
        }
    }
}
use std::thread;
use std::time::{Duration, Instant};
use std::sync::mpsc::Receiver;

enum Signal {
    Think,
    Ponder,
    Stop,
    Now,
    Clear,
    Quit,
    Error
}

pub struct Engine {
    pub variable: u128,
    pub abort: bool,

    pub time_start_point: Instant,
    pub time_limit_ms: u128,

    pub rx: Receiver<String>,
    pub print: bool,
}

impl Engine {
    pub fn init(rx: Receiver<String>) -> Engine {
        Self {
            variable: 0,
            abort: true,
            time_start_point: Instant::now(),
            time_limit_ms: 0,
            rx,
            print: false
        }
    }

    fn calc(&mut self, new_time_limit_ms: u128) -> u128 {
        self.time_start_point = Instant::now();
        self.time_limit_ms = new_time_limit_ms;
        self.abort = false;
        
        let mut n: u16 = 0;
        loop {
            n += 1;
            self.variable += 1;
            
            if self.abort {
                break;
            }
            if n & 2047 == 0 {
                n = 0;
                self.update();
            }
        }

        self.variable
    }

    fn clear(&mut self) {
        self.variable = 0;
    }

    fn update(&mut self) {
        thread::sleep(Duration::from_millis(1));
        
        let last = self.rx.try_recv();
        if last.is_ok() {
            let (command, value) = parse_command(&last.unwrap());
            match command {
                Signal::Stop => {
                    println!("update(), stopping now...");
                    self.abort = true;
                    self.print = false;
                },
                Signal::Think => {
                    println!("update(), thinking now...");
                    self.time_limit_ms = ntl(value);
                },
                Signal::Ponder => {
                    println!("update(), pondering now...");
                    self.time_limit_ms = 1 << 64;
                },
                Signal::Now => {
                    self.abort = true;
                },
                _ => {
                    println!("update() error: impossible command?")
                }
            }
        }

        // some other conditional code...
        if self.abort {
            return;
        }
        if self.time_start_point.elapsed().as_millis() > self.time_limit_ms {
            println!("calculated {}", self.variable);
            self.abort = true;
        }
    }

    pub fn listen(&mut self) {
        loop {
            thread::sleep(Duration::from_millis(1));

            let last = self.rx.try_recv();
            if last.is_err() {
                continue;
            }
            let (command, value) = parse_command(&last.unwrap());
            match command {
                Signal::Think => {
                    println!("listen(), thinking now...");
                    self.print = true;
                    self.calc(ntl(value));
                    if self.print {
                        println!("listen(), current: {}", self.variable);
                    }
                },
                Signal::Ponder => {
                    println!("listen(), pondering now...");
                    self.print = true;
                    self.calc(ntl(value));
                    if self.print {
                        println!("listen(), current: {}", self.variable);
                    }
                },
                Signal::Clear => {
                    println!("listen(), clear.");
                    self.clear();
                },
                Signal::Quit => {
                    println!("listen(), quit.");
                    return;
                },
                _ => {
                    println!("listen() error: impossible command?")
                }
            }
        }
    }
}

fn parse_command(input: &String) -> (Signal, u32) {
    let line = input.trim().split(" ").collect::<Vec<&str>>();
    match line[0] {
        "stop" => {
            return (Signal::Stop, 0);
        },
        "think" => {
            let gtl = line[1].parse::<u32>().unwrap();
            return (Signal::Think, gtl);
        },
        "ponder" => {
            return (Signal::Ponder, 0);
        },
        "now" => {
            return (Signal::Now, 0);
        },
        "clear" => {
            return (Signal::Clear, 0);
        },
        "quit" => {
            return (Signal::Quit, 0);
        }
        _ => {
            return (Signal::Error, 0)
        }
    }
}

#[inline]
fn ntl(gtl: u32) -> u128 {
    gtl as u128
}
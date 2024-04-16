use std::thread;
use std::time::{Duration, Instant};
use std::sync::mpsc::Receiver;

pub struct Engine<'a> {
    pub variable: u128,
    pub abort: bool,

    pub time_start_point: Instant,
    pub time_limit_ms: u128,

    pub rx: &'a Receiver<String>
}
impl Engine<'static> {
    pub fn init (rx: &'static Receiver<String>) -> Engine {
        Self {
            variable: 0,
            abort: true,
            time_start_point: Instant::now(),
            time_limit_ms: 0,
            rx
        }
    }

    pub fn calc(&mut self, new_time_limit_ms: u128) -> u128 {
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

    pub fn clear(&mut self) {
        self.variable = 0;
    }

    pub fn update(&mut self) {
        thread::sleep(Duration::from_millis(1));
        
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
            // recieve: TODO
            if true {
                break;
            }
        }
    }
}
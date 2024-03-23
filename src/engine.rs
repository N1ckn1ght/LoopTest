use std::time::Instant;

pub struct Engine {
    pub variable: u128,
    pub abort: bool,

    pub time_start_point: Instant,
    pub time_limit_ms: u128
}

impl Default for Engine {
    fn default() -> Engine {
        Self {
            variable: 0,
            abort: true,
            time_start_point: Instant::now(),
            time_limit_ms: 0
        }
    }
}

impl Engine {
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
        if self.time_start_point.elapsed().as_millis() > self.time_limit_ms {
            self.abort = true;
        }

        // some other conditional code...
    }
}
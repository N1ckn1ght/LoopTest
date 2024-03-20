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
    pub fn calc(&mut self, new_time_limit_ms: u128) {
        self.time_limit_ms = new_time_limit_ms;
        

    }

    pub fn listen(&mut self) {
        
    }
}
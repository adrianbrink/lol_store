use std::time::SystemTime;
use std::time::Duration;

pub struct RateLimiter {
    one_gate: Gate,
    two_gate: Gate,
}

impl RateLimiter {
    pub fn new() -> RateLimiter {
        RateLimiter {
            one_gate: Gate {
                count: 0,
                limit: 10,
                time: SystemTime::now(),
            },
            two_gate: Gate {
                count: 0,
                limit: 500,
                time: SystemTime::now(),
            },
        }
    }
}

struct Gate {
    count: i32,
    limit: i32,
    //duration: Duration,
    time: SystemTime,
}
/*
impl Gate {
    pub fn is_allowed(&self) -> bool {
        let current_time = SystemTime::now();
        let difference = current_time.duration_since(self.time);
        if difference
    }
}
*/

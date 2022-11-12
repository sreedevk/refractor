use chrono::{DateTime, Duration, Utc};

pub struct Benchmark;

impl Benchmark {
    pub fn run<T>(func: fn() -> T) -> (Duration, T) {
        let start_time = Utc::now();
        let ret = func();
        let end_time = Utc::now();

        (end_time - start_time, ret)
    }
}

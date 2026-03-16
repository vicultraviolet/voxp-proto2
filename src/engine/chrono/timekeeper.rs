use std::time::{Duration, Instant};

use spin_sleep::{SpinSleeper, SpinStrategy};

pub struct Timekeeper
{
    sleeper: SpinSleeper,
    frame_duration: Duration,
    previous_time: Instant,
    dt: f64
}

impl Timekeeper
{
    pub fn new(target_fps: u64) -> Self
    {
        Self{
            sleeper: SpinSleeper::new(1_500_000).with_spin_strategy(SpinStrategy::YieldThread),
            frame_duration: Duration::from_secs_f64(1.0 / target_fps as f64),
            previous_time: Instant::now(),
            dt: 0.0
        } 
    }

    pub fn tick(&mut self) -> f64
    {
        let current_time = Instant::now();

        let delta = current_time - self.previous_time;
        self.previous_time = current_time;

        self.dt = delta.as_secs_f64();
        self.dt
    }

    pub fn pace(&self)
    {
        self.sleeper.sleep_until(self.previous_time + self.frame_duration);
    }

    pub fn set_target_fps(&mut self, fps: f64)
    {
        self.frame_duration = Duration::from_secs_f64(1.0 / fps);
    }

    pub fn dt(&self) -> f64 { self.dt }
}


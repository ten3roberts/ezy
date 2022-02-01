use std::time::Duration;

use crate::traits::Lerp;

/// Describes how a value should be interpolated over time.
#[derive(Debug, Clone)]
pub struct Tween<T> {
    target: T,
    duration: Duration,
}

impl<T: Lerp + Clone> Tween<T> {
    pub fn new(target: T, duration: Duration) -> Self {
        Self { target, duration }
    }

    pub fn start(&self, start: T) -> TweenInstance<T> {
        TweenInstance {
            start,
            target: self.target.clone(),
            time: Duration::ZERO,
            duration: self.duration,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TweenInstance<T> {
    start: T,
    target: T,
    time: Duration,
    duration: Duration,
}

impl<T: Lerp> TweenInstance<T> {
    pub fn new(start: T, target: T, time: Duration, duration: Duration) -> Self {
        Self {
            start,
            target,
            time,
            duration,
        }
    }

    pub fn finished(&self) -> bool {
        self.time >= self.duration
    }

    pub fn progress(&mut self, val: &mut T, dt: Duration) {
        self.time += dt;
        let t = (self.time.as_secs_f32() / self.duration.as_secs_f32()).min(1.0);
        dbg!(t);
        *val = self.start.lerp(&self.target, t);
    }
}

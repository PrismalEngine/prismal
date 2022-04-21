use std::time::Duration;

cfg_if::cfg_if! {
    if #[cfg(target_arch="wasm32")] {
        type TimePoint = Duration;
        fn now() -> TimePoint {
            let window = web_sys::window().expect("should have a window in this context");
            let performance = window
                .performance()
                .expect("performance should be available");
            let amt = performance.now();
            let secs = (amt as u64) / 1_000;
            let nanos = (((amt as u64) % 1_000) as u32) * 1_000_000;
            Duration::new(secs, nanos)
        }
    } else {
        use std::time::Instant;
        type TimePoint = Instant;
        fn now() -> TimePoint {
            Instant::now()
        }
    }
}

pub struct Time {
    frame_delta: Duration,
    app_start_time: TimePoint,
    last_frame_time: TimePoint,
}

impl Time {
    pub fn new() -> Self {
        let n = now();
        Self {
            frame_delta: Duration::from_millis(20),
            last_frame_time: n,
            app_start_time: n,
        }
    }
    pub fn seconds(&self) -> Duration {
        let n = now();
        n - self.app_start_time
    }
    pub fn frame_delta(&self) -> Duration {
        self.frame_delta
    }
    pub(crate) fn after_frame(&mut self) {
        let n = now();
        self.frame_delta = n - self.last_frame_time;
        self.last_frame_time = n;
    }
}

impl Default for Time {
    fn default() -> Self {
        Self::new()
    }
}

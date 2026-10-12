pub struct HighResolutionTimer(u64, u64);

impl HighResolutionTimer {
    pub fn new() -> Self {
        Self(0, 0)
    }

    pub fn start(&mut self) {
        self.0 = sdl3::timer::performance_counter();
    }

    pub fn stop(&mut self) {
        self.1 = sdl3::timer::performance_counter();
    }

    pub fn nanoseconds(&self) -> u32 {
        ((1_000_000_000 * (self.1 - self.0)) / sdl3::timer::performance_frequency())
            .try_into()
            .unwrap_or(u32::MAX)
    }
}

impl Default for HighResolutionTimer {
    fn default() -> Self {
        Self::new()
    }
}

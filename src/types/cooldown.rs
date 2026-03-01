#[derive(Default)]
pub struct Cooldown {
    pub start_time: Option<f64>,
    pub duration: f64,
}

impl Cooldown {
    pub fn update(&mut self, time: f64) {
        if let Some(start) = self.start_time {
            if time >= start + self.duration {
                self.start_time = None;
            }
        }
    }

    pub fn start(&mut self, time: f64) {
        if self.start_time.is_none() {
            self.start_time = Some(time);
        }
    }

    pub fn reset(&mut self) {
        self.start_time = None;
    }

    pub fn new(duration: f64) -> Self {
        Self {
            start_time: None,
            duration,
        }
    }
}
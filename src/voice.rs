pub struct VoiceList {
    voices: [Option<Voice>; 16],
}

impl VoiceList {
    pub fn new() -> Self {
        Self { voices: [None; 16] }
    }
    pub fn play(&mut self, sample_rate: f32) -> f32 {
        self.voices
            .iter_mut()
            .map(|v| v.as_mut().map(|v| v.step(sample_rate)).unwrap_or(0.0))
            .sum()
    }
    pub fn add_voice(&mut self, frequency: f32) {
        if let Some(voice) = self.voices.iter_mut().find(|v| v.is_none()) {
            *voice = Some(Voice::new(frequency));
        }
    }
}

#[derive(Clone, Copy)]
pub struct Voice {
    frequency: f32,
    phase: f32,
    time: u32,
}

impl Voice {
    fn new(frequency: f32) -> Self {
        Self {
            frequency,
            phase: 0.0,
            time: 0,
        }
    }
    fn step(&mut self, sample_rate: f32) -> f32 {
        self.time += 1;
        self.calculate_sine(Voice::calculate_delta(self.frequency, sample_rate))
    }
    fn calculate_delta(frequency: f32, sample_rate: f32) -> f32 {
        frequency / sample_rate
    }
    fn calculate_sine(&mut self, phase_delta: f32) -> f32 {
        let sine = (self.phase * std::f32::consts::TAU).sin();

        self.phase += phase_delta;
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }

        sine
    }
}

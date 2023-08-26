pub struct VoiceList {
    voices: [Option<Oscillator>; 16],
}

impl VoiceList {
    pub fn new() -> Self {
        Self { voices: [None; 16] }
    }
    pub fn play(&mut self, params: &OscParams) -> f32 {
        self.voices
            .iter_mut()
            .map(|v| v.as_mut().map(|v| v.step_with_envelope(params)).unwrap_or(0.0))
            .sum()
    }
    pub fn add_voice(&mut self, note: u8, params: &OscParams) {
        if let Some(voice) = self.voices.iter_mut().find(|v| v.is_none()) {
            *voice = Some(Oscillator::new(note, params));
        } else {
            *self.voices.get_mut(0).unwrap() = Some(Oscillator::new(note, params));
        }
    }
    pub fn release_voice(&mut self, note: u8, params: &OscParams) {
        for slot in self.voices.iter_mut() {
            if let Some(voice) = slot {
                if voice.midi_id == note {
                    voice.release(params);
                }
            }
        }
    }
    pub fn remove_voices(&mut self, params: &OscParams) {
        for slot in self.voices.iter_mut() {
            if let Some(voice) = slot {
                if voice.is_done(params) {
                    *slot = None;
                }
            }
        }
    }
    pub fn update(&mut self, params: &OscParams) {
        for slot in self.voices.iter_mut() {
            if let Some(voice) = slot {
                voice.update_pitch(params);
            }
        }
    }
}

pub struct Voice {
    oscillators: [Oscillator; 2],
}
impl Voice {
    pub fn play(&mut self, params: &[OscParams]) -> f32 {
        self.oscillators
            .iter_mut()
            .zip(params.iter())
            .map(|(v, param)| v.step_with_envelope(param))
            .sum()
    }
    pub fn new(note: u8, params1: &OscParams, params2: &OscParams) -> Self {
        Self {
            oscillators: [
                Oscillator::new(note, params1),
                Oscillator::new(note, params2),
            ]
        }
    }
    pub fn release_voice(&mut self, note: u8, params: &OscParams) {
        for osc in self.oscillators.iter_mut() {
            if osc.midi_id == note {
                osc.release(params);
            }
        }
    }
    pub fn is_done(&mut self, params: &OscParams) -> bool {
        self.oscillators.iter().any(|f| f.is_done(params))
    }
    pub fn update(&mut self, params: &OscParams) {
        self.oscillators.iter_mut().for_each(|v| v.update_pitch(params));
    }
}

pub struct OscParams {
    pub sample_rate: f32,
    pub coarse: f32,
    pub fine: f32,
    pub frequency_mult: f32,
    pub attack: f32,
    pub decay: f32,
    pub sustain: f32,
    pub release: f32,
}

#[derive(Clone, Copy)]
pub struct Oscillator {
    frequency: f32,
    midi_id: u8,
    phase: f32,
    time: u32,
    release_time: Option<u32>,
    release_level: f32,
}

impl Oscillator {
    fn new(midi_id: u8, params: &OscParams) -> Self {
        let frequency = 2.0f32
            .powf((midi_id as f32 + params.coarse + params.fine / 100.0 - 69.0) / 12.0)
            * 440.0 * params.frequency_mult;
        Self {
            frequency,
            midi_id,
            phase: 0.0,
            time: 0,
            release_time: None,
            release_level: 0.0,
        }
    }
    fn envelope(&self, params: &OscParams) -> f32 {
        let time = self.time as f32 / params.sample_rate;
        if let Some(released_time) = self.released_time() {
            let delta = released_time as f32 / params.sample_rate;
            self.release_level * (1.0 - (delta as f32 / params.release)).powi(2)
        } else if time < params.attack {
            time / params.attack
        } else if time < params.attack + params.decay {
            (1.0 - ((time - params.attack) / params.decay)).powi(2) * (1.0 - params.sustain)
        } else {
            params.sustain
        }.min(1.0).max(0.0)
    }
    fn update_pitch(&mut self, params: &OscParams) {
        self.frequency = 2.0f32
        .powf((self.midi_id as f32 + params.coarse + params.fine / 100.0 - 69.0) / 12.0)
        * 440.0 * params.frequency_mult;
    }
    fn step(&mut self, params: &OscParams) -> f32 {
        self.time += 1;
        self.calculate_sine(Oscillator::calculate_delta(self.frequency, params.sample_rate))
    }
    fn step_with_envelope(&mut self, params: &OscParams) -> f32 {
        self.step(params) * self.envelope(params)
    }
    fn release(&mut self, params: &OscParams) {
        self.release_level = self.envelope(params);
        self.release_time = Some(self.time);
    }
    fn released_time(&self) -> Option<u32> {
        if let Some(release_time) = self.release_time {
            Some(self.time - release_time)
        } else {
            None
        }
    }
    fn is_done(&self, params: &OscParams) -> bool {
        if let Some(released_time) = self.released_time() {
            released_time as f32 / params.sample_rate > params.release
        } else {
            false
        }
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

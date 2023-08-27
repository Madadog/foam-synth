pub struct VoiceList {
    voices: [Option<Voice>; 32],
}

impl VoiceList {
    pub fn new() -> Self {
        Self { voices: [None; 32] }
    }
    pub fn play(&mut self, params: &[OscParams], pm_matrix: [f32; 2]) -> f32 {
        self.voices
            .iter_mut()
            .filter_map(|voice| voice.as_mut())
            .map(|v| v.play(params, pm_matrix))
            .sum()
    }
    pub fn add_voice(&mut self, note: u8, params: &[OscParams], velocity: f32) {
        if let Some(voice) = self.voices.iter_mut().find(|v| v.is_none()) {
            *voice = Some(Voice::new(note, params, velocity));
        } else {
            *self.voices.get_mut(note as usize % 16).unwrap() =
                Some(Voice::new(note, params, velocity));
        }
    }
    pub fn release_voice(&mut self, note: u8, params: &[OscParams]) {
        for slot in self.voices.iter_mut() {
            if let Some(voice) = slot {
                if voice.midi_id == note {
                    voice.release(params);
                }
            }
        }
    }
    pub fn remove_voices(&mut self, params: &[OscParams]) {
        for slot in self.voices.iter_mut() {
            if let Some(voice) = slot {
                if voice.is_done(params) {
                    *slot = None;
                }
            }
        }
    }
    pub fn update(&mut self, params: &[OscParams]) {
        for slot in self.voices.iter_mut() {
            if let Some(voice) = slot {
                voice.update(params);
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Voice {
    oscillators: [Oscillator; 2],
    midi_id: u8,
}
impl Voice {
    pub fn play(&mut self, params: &[OscParams], mut pm_matrix: [f32; 2]) -> f32 {
        pm_matrix
            .iter_mut()
            .zip(self.oscillators.iter().rev())
            .for_each(|(amt, osc)| *amt = *amt * osc.previous());
        self.oscillators
            .iter_mut()
            .zip(params.iter())
            .zip(pm_matrix)
            .map(|((v, params), pm)| v.step_with_envelope(params, pm))
            .sum()
    }
    pub fn new(midi_id: u8, params: &[OscParams], velocity: f32) -> Self {
        Self {
            oscillators: [
                Oscillator::new(midi_id, &params[0], velocity),
                Oscillator::new(midi_id, &params[1], velocity),
            ],
            midi_id,
        }
    }
    pub fn release(&mut self, params: &[OscParams]) {
        for (osc, params) in self.oscillators.iter_mut().zip(params.iter()) {
            osc.release(params);
        }
    }
    pub fn is_done(&mut self, params: &[OscParams]) -> bool {
        self.oscillators
            .iter()
            .zip(params.iter())
            .all(|(osc, params)| osc.is_done(params))
    }
    pub fn update(&mut self, params: &[OscParams]) {
        self.oscillators
            .iter_mut()
            .zip(params.iter())
            .for_each(|(osc, params)| osc.update_pitch(params));
    }
}

#[derive(Debug, Clone, Copy)]
pub struct OscParams {
    pub output_gain: f32,
    pub sample_rate: f32,
    pub coarse: f32,
    pub fine: f32,
    pub frequency_mult: f32,
    pub attack: f32,
    pub decay: f32,
    pub sustain: f32,
    pub release: f32,
    pub feedback: f32,
    pub velocity_sensitivity: f32,
    pub keyscaling: f32,
    pub octave_stretch: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct Oscillator {
    frequency: f32,
    midi_id: u8,
    phase: f32,
    time: u32,
    release_time: Option<u32>,
    release_level: f32,
    previous_sine: [f32; 2],
    previous_output: f32,
    gain: f32,
}

impl Oscillator {
    fn new(midi_id: u8, params: &OscParams, velocity: f32) -> Self {
        let frequency = Oscillator::get_pitch(midi_id, params);
        let keyscaling = 1.0 - params.keyscaling * (midi_id as f32 - 69.0) / 69.0;
        Self {
            frequency,
            midi_id,
            phase: 0.0,
            time: 0,
            release_time: None,
            release_level: 0.0,
            previous_sine: [0.0; 2],
            previous_output: 0.0,
            gain: (params.velocity_sensitivity * velocity + 1.0
                - params.velocity_sensitivity.max(0.0))
                * keyscaling,
        }
    }
    fn envelope(&self, params: &OscParams) -> f32 {
        let time = self.time as f32 / params.sample_rate;
        if let Some(released_time) = self.released_time() {
            let delta = released_time as f32 / params.sample_rate;
            self.release_level * (1.0 - (delta as f32 / params.release)).max(0.0).powi(2)
        } else if time < params.attack {
            time / params.attack
        } else if time < params.attack + params.decay {
            (1.0 - ((time - params.attack) / params.decay)).powi(2) * (1.0 - params.sustain)
                + params.sustain
        } else {
            params.sustain
        }
        .max(0.0)
    }
    fn get_pitch(midi_id: u8, params: &OscParams) -> f32 {
        2.0f32.powf((midi_id as f32 + params.coarse + params.fine / 100.0 - 69.0) / (12.0 / params.octave_stretch))
            * 440.0
            * params.frequency_mult
    }
    fn update_pitch(&mut self, params: &OscParams) {
        self.frequency = Oscillator::get_pitch(self.midi_id, params);
    }
    fn step(&mut self, params: &OscParams, pm: f32) -> f32 {
        self.time += 1;
        // Feedback implementation from the Surge XT FM2/FM3/Sine oscillators, which in turn were based on the DX7 feedback
        let prev = (self.previous_sine[0] + self.previous_sine[1]) / 2.0;
        let feedback = if params.feedback.is_sign_negative() {
            prev.powi(2)
        } else {
            prev
        } * params.feedback.abs();
        let phase = self.phase + feedback + pm;
        self.previous_sine[1] = self.previous_sine[0];
        self.previous_sine[0] = (phase * std::f32::consts::TAU).sin();
        self.add_phase(Oscillator::calculate_delta(
            self.frequency,
            params.sample_rate,
        ));
        self.previous_sine[0] * self.gain
    }
    fn step_with_envelope(&mut self, params: &OscParams, pm: f32) -> f32 {
        self.previous_output = self.step(params, pm) * self.envelope(params);
        self.previous_output * params.output_gain
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
    fn add_phase(&mut self, phase_delta: f32) {
        self.phase += phase_delta;
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }
    }
    fn calculate_sine(&mut self, phase_delta: f32) -> f32 {
        let sine = (self.phase * std::f32::consts::TAU).sin();

        self.add_phase(phase_delta);

        sine
    }
    fn previous(&self) -> f32 {
        self.previous_output * self.gain
    }
}

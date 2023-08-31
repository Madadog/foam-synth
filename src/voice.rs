use crate::svf_simper::{FilterType, SvfSimper};

pub struct VoiceList {
    voices: [Option<Voice>; 32],
}

impl VoiceList {
    pub fn new() -> Self {
        Self { voices: [None; 32] }
    }
    pub fn play(&mut self, params: &[OscParams], pm_matrix: [[f32; 5]; 6]) -> f32 {
        self.voices
            .iter_mut()
            .filter_map(|voice| voice.as_mut())
            .map(|v| v.play(params, pm_matrix))
            .sum()
    }
    pub fn add_voice(
        &mut self,
        note: u8,
        osc_params: &[OscParams],
        velocity: f32,
        voice_params: VoiceParams,
    ) {
        if let Some(voice) = self.voices.iter_mut().find(|v| v.is_none()) {
            *voice = Some(Voice::new(note, osc_params, velocity, voice_params));
        } else {
            *self.voices.get_mut(note as usize % 16).unwrap() =
                Some(Voice::new(note, osc_params, velocity, voice_params));
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
    pub fn update(&mut self, osc_params: &[OscParams], voice_params: VoiceParams) {
        for slot in self.voices.iter_mut() {
            if let Some(voice) = slot {
                voice.update(osc_params, voice_params);
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Voice {
    oscillators: [Oscillator; 6],
    midi_id: u8,
    filter: Option<SvfSimper>,
    time: u32,
    released_time: Option<u32>,
    release_level: f32,
}
impl Voice {
    pub fn play(&mut self, params: &[OscParams], pm_matrix: [[f32; 5]; 6]) -> f32 {
        self.time += 1;
        let matrix = [
            pm_matrix[0][0] * self.oscillators[1].previous()
                + pm_matrix[0][1] * self.oscillators[2].previous()
                + pm_matrix[0][2] * self.oscillators[3].previous()
                + pm_matrix[0][3] * self.oscillators[4].previous()
                + pm_matrix[0][4] * self.oscillators[5].previous(),
            pm_matrix[1][0] * self.oscillators[0].previous()
                + pm_matrix[1][1] * self.oscillators[2].previous()
                + pm_matrix[1][2] * self.oscillators[3].previous()
                + pm_matrix[1][3] * self.oscillators[4].previous()
                + pm_matrix[1][4] * self.oscillators[5].previous(),
            pm_matrix[2][0] * self.oscillators[0].previous()
                + pm_matrix[2][1] * self.oscillators[1].previous()
                + pm_matrix[2][2] * self.oscillators[3].previous()
                + pm_matrix[2][3] * self.oscillators[4].previous()
                + pm_matrix[2][4] * self.oscillators[5].previous(),
            pm_matrix[3][0] * self.oscillators[0].previous()
                + pm_matrix[3][1] * self.oscillators[1].previous()
                + pm_matrix[3][2] * self.oscillators[2].previous()
                + pm_matrix[3][3] * self.oscillators[4].previous()
                + pm_matrix[3][4] * self.oscillators[5].previous(),
            pm_matrix[4][0] * self.oscillators[0].previous()
                + pm_matrix[4][1] * self.oscillators[1].previous()
                + pm_matrix[4][2] * self.oscillators[2].previous()
                + pm_matrix[4][3] * self.oscillators[3].previous()
                + pm_matrix[4][4] * self.oscillators[5].previous(),
            pm_matrix[5][0] * self.oscillators[0].previous()
                + pm_matrix[5][1] * self.oscillators[1].previous()
                + pm_matrix[5][2] * self.oscillators[2].previous()
                + pm_matrix[5][3] * self.oscillators[3].previous()
                + pm_matrix[5][4] * self.oscillators[4].previous(),
        ];
        let out = self
            .oscillators
            .iter_mut()
            .zip(params.iter())
            .zip(matrix)
            .map(|((v, params), pm)| v.step_with_envelope(params, pm))
            .sum();
        if let Some(filter) = self.filter.as_mut() {
            filter.process(out)
        } else {
            out
        }
    }
    pub fn new(
        midi_id: u8,
        osc_params: &[OscParams],
        velocity: f32,
        voice_params: VoiceParams,
    ) -> Self {
        Self {
            oscillators: [
                Oscillator::new(midi_id, &osc_params[0], velocity),
                Oscillator::new(midi_id, &osc_params[1], velocity),
                Oscillator::new(midi_id, &osc_params[2], velocity),
                Oscillator::new(midi_id, &osc_params[3], velocity),
                Oscillator::new(midi_id, &osc_params[4], velocity),
                Oscillator::new(midi_id, &osc_params[5], velocity),
            ],
            midi_id,
            filter: if voice_params.filter_enabled {
                let mut filter = SvfSimper::new(
                    Voice::calc_filter_cutoff(
                        &voice_params,
                        envelope(
                            voice_params.sample_rate,
                            0,
                            voice_params.filter_attack,
                            voice_params.filter_decay,
                            voice_params.filter_sustain,
                        ),
                    ),
                    voice_params.filter_resonance,
                    voice_params.sample_rate,
                );
                filter.filter_type = voice_params.filter_type;
                Some(filter)
            } else {
                None
            },
            time: 0,
            released_time: None,
            release_level: 0.0,
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
    pub fn update(&mut self, osc_params: &[OscParams], voice_params: VoiceParams) {
        self.oscillators
            .iter_mut()
            .zip(osc_params.iter())
            .for_each(|(osc, params)| osc.update_pitch(params));
        if !voice_params.filter_enabled {
            self.filter = None
        } else {
            let cutoff =
                Voice::calc_filter_cutoff(&voice_params, self.calc_filter_envelope(&voice_params));
            let filter = self.filter.get_or_insert(SvfSimper::new(
                cutoff,
                voice_params.filter_resonance,
                voice_params.sample_rate,
            ));
            filter.set(
                cutoff,
                voice_params.filter_resonance,
                voice_params.sample_rate,
            );
            filter.set_filter_type(voice_params.filter_type);
        }
    }
    fn calc_filter_envelope(&self, voice_params: &VoiceParams) -> f32 {
        if let Some(released_time) = self.released_time {
            release_envelope(
                voice_params.sample_rate,
                self.time - released_time,
                voice_params.filter_release,
                self.release_level,
            )
        } else {
            envelope(
                voice_params.sample_rate,
                self.time,
                voice_params.filter_attack,
                voice_params.filter_decay,
                voice_params.filter_sustain,
            )
        }
    }
    fn calc_filter_cutoff(voice_params: &VoiceParams, envelope: f32) -> f32 {
        (voice_params.filter_cutoff
            + voice_params.filter_envelope_amount * 22000.0 * envelope.powi(2))
        .clamp(20.0, 22000.0)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct VoiceParams {
    pub sample_rate: f32,
    pub filter_enabled: bool,
    pub filter_type: FilterType,
    pub filter_cutoff: f32,
    pub filter_resonance: f32,
    pub filter_envelope_amount: f32,
    pub filter_attack: f32,
    pub filter_decay: f32,
    pub filter_sustain: f32,
    pub filter_release: f32,
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

pub fn envelope(sample_rate: f32, time: u32, attack: f32, decay: f32, sustain: f32) -> f32 {
    let time = time as f32 / sample_rate;
    if time < attack {
        time / attack
    } else if time < attack + decay {
        (1.0 - ((time - attack) / decay)).powi(2) * (1.0 - sustain) + sustain
    } else {
        sustain
    }
    .max(0.0)
}

pub fn release_envelope(sample_rate: f32, time: u32, release: f32, release_level: f32) -> f32 {
    let delta = time as f32 / sample_rate;
    release_level * (1.0 - (delta as f32 / release)).max(0.0).powi(2)
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
        if let Some(released_time) = self.released_time() {
            release_envelope(
                params.sample_rate,
                released_time,
                params.release,
                self.release_level,
            )
        } else {
            envelope(
                params.sample_rate,
                self.time,
                params.attack,
                params.decay,
                params.sustain,
            )
        }
    }
    fn get_pitch(midi_id: u8, params: &OscParams) -> f32 {
        2.0f32.powf(
            (midi_id as f32 + params.coarse + params.fine / 100.0 - 69.0)
                / (12.0 / params.octave_stretch),
        ) * 440.0
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

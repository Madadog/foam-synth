use std::{
    array,
    f32::consts::{PI, TAU},
};

use itertools::izip;
use nih_plug::params::enums::Enum;
// use wide::{f32x8, u32x8, CmpLe};
use wide::*;

use crate::{
    dsp::interpolation::{lerp, lerpx8},
    svf_simper::{FilterType, SvfSimper, SvfSimperBatch},
};

pub struct VoiceList {
    voices: [Option<Voice>; 32],
}

impl VoiceList {
    pub fn new() -> Self {
        Self { voices: [None; 32] }
    }
    pub fn play(
        &mut self,
        osc_params: &OscParamsBatch,
        voice_params: &VoiceParams,
        pm_matrix: [f32x8; 8],
    ) -> f32 {
        self.voices
            .iter_mut()
            .filter_map(|voice| voice.as_mut())
            .map(|v| v.play(osc_params, voice_params, pm_matrix))
            .sum()
    }
    pub fn add_voice(
        &mut self,
        note: u8,
        osc_params: &OscParamsBatch,
        velocity: f32,
        voice_params: VoiceParams,
    ) {
        if let Some(voice) = self.voices.iter_mut().find(|v| v.is_none()) {
            *voice = Some(Voice::new(note, osc_params, velocity, voice_params));
        } else {
            let ((_, released), (_, unreleased)) = self.voices.iter_mut().flatten().fold(
                ((0, None), (0, None)),
                |(released, unreleased), voice| {
                    if voice.is_released() {
                        if voice.time >= released.0 {
                            ((voice.time, Some(voice)), unreleased)
                        } else {
                            (released, unreleased)
                        }
                    } else {
                        if voice.time >= unreleased.0 {
                            (released, (voice.time, Some(voice)))
                        } else {
                            (released, unreleased)
                        }
                    }
                },
            );
            if let Some(released) = released {
                *released = Voice::new(note, osc_params, velocity, voice_params);
            } else {
                *unreleased.expect("Could not find any voice slots... Panicking!!!") =
                    Voice::new(note, osc_params, velocity, voice_params);
            }
        }
    }
    pub fn release_voice(
        &mut self,
        note: u8,
        osc_params: &OscParamsBatch,
        voice_params: &VoiceParams,
    ) {
        for slot in self.voices.iter_mut() {
            if let Some(voice) = slot {
                if voice.midi_id == note {
                    voice.release(osc_params, voice_params);
                }
            }
        }
    }
    pub fn remove_voices(&mut self, osc_params: &OscParamsBatch, voice_params: &VoiceParams) {
        for slot in self.voices.iter_mut() {
            if let Some(voice) = slot {
                if voice.is_done(osc_params, voice_params) {
                    *slot = None;
                }
            }
        }
    }
    pub fn block_update(&mut self, osc_params: &OscParamsBatch, voice_params: VoiceParams) {
        for slot in self.voices.iter_mut() {
            if let Some(voice) = slot {
                voice.block_update(osc_params, voice_params);
            }
        }
    }
    pub fn sample_update(&mut self, osc_params: &OscParamsBatch, voice_params: VoiceParams) {
        for slot in self.voices.iter_mut() {
            if let Some(voice) = slot {
                voice.sample_update(osc_params, voice_params);
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Voice {
    oscillators: OscillatorBatch,
    midi_id: u8,
    filter: Option<SvfSimper>,
    time: u32,
    released_time: Option<u32>,
    amp_release_level: f32,
    filter_release_level: f32,
}
impl Voice {
    pub fn play(
        &mut self,
        params: &OscParamsBatch,
        voice_params: &VoiceParams,
        pm_matrix: [f32x8; 8],
    ) -> f32 {
        self.time += 1;
        let matrix: [f32; 8] =
            array::from_fn(|i| (pm_matrix[i] * self.oscillators.previous()).reduce_add());
        let out = self
            .oscillators
            .step_with_envelope(params, f32x8::from(matrix))
            .reduce_add();
        (if let Some(filter) = self.filter.as_mut() {
            filter.process(out)
        } else {
            out
        }) * self.calc_amp_envelope(voice_params)
    }
    pub fn new(
        midi_id: u8,
        osc_params: &OscParamsBatch,
        velocity: f32,
        voice_params: VoiceParams,
    ) -> Self {
        Self {
            oscillators: OscillatorBatch::new(midi_id, osc_params, velocity),
            midi_id,
            filter: if voice_params.filter_enabled {
                let mut filter = SvfSimper::new(
                    Voice::calc_filter_cutoff(
                        midi_id,
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
            amp_release_level: 0.0,
            filter_release_level: 0.0,
        }
    }
    pub fn release(&mut self, params: &OscParamsBatch, voice_params: &VoiceParams) {
        self.amp_release_level = self.calc_amp_envelope(voice_params);
        self.oscillators.release(params);
        self.released_time = Some(self.time);
    }
    pub fn is_released(&self) -> bool {
        self.released_time.is_some()
    }
    fn time_since_release(&self) -> Option<u32> {
        if let Some(release_time) = self.released_time {
            Some(self.time - release_time)
        } else {
            None
        }
    }
    pub fn is_done(&mut self, osc_params: &OscParamsBatch, voice_params: &VoiceParams) -> bool {
        self.oscillators.is_done(osc_params)
            || if let Some(released_time) = self.time_since_release() {
                (released_time as f32 / voice_params.sample_rate) >= voice_params.global_release
            } else {
                false
            }
    }
    pub fn block_update(&mut self, osc_params: &OscParamsBatch, voice_params: VoiceParams) {
        self.oscillators.update_pitch(osc_params);
    }
    pub fn sample_update(&mut self, _params: &OscParamsBatch, voice_params: VoiceParams) {
        if voice_params.filter_enabled {
            let cutoff = Self::calc_filter_cutoff(
                self.midi_id,
                &voice_params,
                self.calc_filter_envelope(&voice_params),
            );
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
        } else {
            self.filter = None
        }
    }
    fn calc_amp_envelope(&self, voice_params: &VoiceParams) -> f32 {
        if let Some(released_time) = self.released_time {
            release_envelope(
                voice_params.sample_rate,
                self.time - released_time,
                voice_params.global_release,
                self.amp_release_level,
            )
        } else {
            envelope(
                voice_params.sample_rate,
                self.time,
                voice_params.global_attack,
                voice_params.global_decay,
                voice_params.global_sustain,
            )
        }
    }
    fn calc_filter_envelope(&self, voice_params: &VoiceParams) -> f32 {
        if let Some(released_time) = self.released_time {
            release_envelope(
                voice_params.sample_rate,
                self.time - released_time,
                voice_params.filter_release,
                self.filter_release_level,
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
    fn calc_filter_cutoff(midi_id: u8, voice_params: &VoiceParams, envelope: f32) -> f32 {
        let keyscaling = 2.0f32.powf((midi_id as f32 - 69.0) * voice_params.filter_keytrack / 12.0);
        (voice_params.filter_cutoff * keyscaling
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
    pub filter_keytrack: f32,
    pub global_attack: f32,
    pub global_decay: f32,
    pub global_sustain: f32,
    pub global_release: f32,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct OscParams {
    pub output_gain: f32,
    pub sample_rate: f32,
    pub coarse: f32,
    pub fine: f32,
    pub frequency_mult: f32,
    pub hz_detune: f32,
    pub phase_offset: f32,
    pub phase_rand: f32,
    pub attack_level: f32,
    pub release_level: f32,
    pub delay: f32,
    pub attack: f32,
    pub hold: f32,
    pub decay: f32,
    pub sustain: f32,
    pub release: f32,
    pub feedback: f32,
    pub velocity_sensitivity: f32,
    pub keyscaling: f32,
    pub octave_stretch: f32,
    pub waveshaper: Waveshaper,
    pub waveshaper_amount: f32,
    pub phaseshaper: Waveshaper,
    pub phaseshaper_amount: f32,
}

pub fn envelope(sample_rate: f32, time: u32, attack: f32, decay: f32, sustain: f32) -> f32 {
    let time = time as f32 / sample_rate;
    if time < attack {
        time / attack
    } else if time < attack + decay {
        (1.0 - ((time - attack) / decay)).powf(2.0) * (1.0 - sustain) + sustain
    } else {
        sustain
    }
    .max(0.0)
}

pub fn release_envelope(sample_rate: f32, time: u32, release: f32, release_level: f32) -> f32 {
    let delta = time as f32 / sample_rate;
    release_level * (1.0 - (delta as f32 / release)).max(0.0).powi(2)
}

#[derive(Debug, Clone, Copy, PartialEq, Default, Enum)]
pub enum Waveshaper {
    #[default]
    None,
    Power,
    InversePower,
    BiasedPower,
    BiasedInversePower,
    Sync,
    Sine,
    Quantize,
    HalfRectify,
    FullRectify,
    HardClip,
    HardGate,
}
impl Waveshaper {
    /// `x` should be between -1 and +1, `amount` should be between 0 and 1
    pub fn waveshape(&self, x: f32, amount: f32) -> f32 {
        match self {
            Waveshaper::None => x,
            Waveshaper::Power => {
                let amount = amount * 200.0 + 1.0;
                x.abs().powf(amount) * x.signum()
            },
            Waveshaper::InversePower => {
                let amount = amount * 20.0 + 1.0;
                x.abs().powf(1.0 / amount) * x.signum()
            },
            Waveshaper::BiasedPower => {
                let amount = amount * 400.0 + 1.0;
                (x * 0.5 + 0.5).powf(amount) * 2.0 - 1.0
            },
            Waveshaper::BiasedInversePower => {
                let amount = amount * 8.0 + 1.0;
                (x * 0.5 + 0.5).powf(1.0 / amount) * 2.0 - 1.0
            },
            Waveshaper::Sync => {
                let amount = amount * 20.0 + 1.0;
                (x * 0.999999 * amount).fract()
            },
            Waveshaper::Sine => {
                let amount = amount * 100.0 + 1.0;
                (x * amount).sin()
            },
            Waveshaper::Quantize => {
                let amount = 1.0 - amount;
                let amount = (amount * amount) * 100.0 + 1.0;
                (x * amount).round() / amount
            }
            Waveshaper::HalfRectify => {
                x.max(-1.0 + amount)
            },
            Waveshaper::FullRectify => {
                let amount = 1.0 - amount;
                (x + amount).abs() - amount
            },
            Waveshaper::HardClip => {
                x.max(-1.01 + amount).min(1.01 - amount) / (1.01 - amount)
            },
            Waveshaper::HardGate => {
                let amount = amount * 0.99;
                (x.abs().max(amount) - amount) * x.signum() / (1.0 - amount)
            },
        }
    }
    /// `x` should be between 0 and +1, `amount` should be between 0 and 1
    pub fn phaseshape(&self, x: f32, amount: f32) -> f32 {
        match self {
            Waveshaper::None => x,
            Waveshaper::Power => {
                let amount = amount * 50.0 + 1.0;
                let bipolar = x * 2.0 - 1.0;
                (bipolar.abs().powf(amount) * bipolar.signum() + 1.0) * 0.5
            }
            Waveshaper::InversePower => {
                let amount = amount * 50.0 + 1.0;
                let bipolar = x * 2.0 - 1.0;
                (bipolar.abs().powf(1.0 / amount) * bipolar.signum() + 1.0) * 0.5
            }
            Waveshaper::BiasedPower => {
                let amount = amount * 50.0 + 1.0;
                x.powf(amount)
            },
            Waveshaper::BiasedInversePower => {
                let amount = amount * 50.0 + 1.0;
                x.powf(1.0 / amount)
            },
            Waveshaper::Sync => {
                let amount = amount * 50.0 + 1.0;
                (x * 0.999999 * amount).fract()
            },
            Waveshaper::Sine => {
                let amount = amount * 50.0 + 1.0;
                (x * amount).sin()
            },
            Waveshaper::Quantize => {
                let amount = 1.0 - amount;
                let amount = (amount * amount) * 100.0 + 1.0;
                (x * (amount + 1.0)).round() / amount
            },
            Waveshaper::HalfRectify => {
                (x*(amount*50.0 + 1.0)).min(1.0)
            },
            Waveshaper::FullRectify => {
                let amount = 1.0 - amount;
                (x + amount).abs() - amount
            },
            Waveshaper::HardClip => {
                x.max(-1.0 + amount).min(1.0 - amount)
            },
            Waveshaper::HardGate => {
                x.abs().max(amount) - amount
            },
        }
    }
    pub fn waveshape_batch(&self, x: f32x8, amount: f32x8) -> f32x8 {
        match self {
            Waveshaper::None => x,
            Waveshaper::Power => x.abs().pow_f32x8(amount) ^ x.sign_bit(),
            Waveshaper::InversePower => x.abs().pow_f32x8(1.0 / amount) ^ x.sign_bit(),
            Waveshaper::BiasedPower => (x * 0.5 + 0.5).pow_f32x8(amount) * 2.0 - 1.0,
            Waveshaper::BiasedInversePower => (x * 0.5 + 0.5).pow_f32x8(1.0 / amount) * 2.0 - 1.0,
            Waveshaper::Sync => {
                // ((x * 0.999999 * amount) % 1.0) * 2.0 - 1.0
                x
            },
            Waveshaper::Sine => (x * amount).sin(),
            Waveshaper::Quantize => {
                let amount = (201.0 - amount) * 0.5;
                (x * f32x8::splat(2.0).pow_f32x8(amount)).round() / (f32x8::splat(2.0).pow_f32x8(amount) + 1.0)
            }
            _ => todo!(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct OscParamsBatch {
    pub output_gain: f32x8,
    pub sample_rate: f32x8,
    pub coarse: f32x8,
    pub fine: f32x8,
    pub frequency_mult: f32x8,
    pub hz_detune: f32x8,
    pub phase_offset: f32x8,
    pub phase_rand: f32x8,
    pub attack_level: f32x8,
    pub release_level: f32x8,
    pub delay: f32x8,
    pub attack: f32x8,
    pub hold: f32x8,
    pub decay: f32x8,
    pub sustain: f32x8,
    pub release: f32x8,
    pub feedback: f32x8,
    pub velocity_sensitivity: f32x8,
    pub keyscaling: f32x8,
    pub octave_stretch: f32x8,
    pub waveshaper: [Waveshaper; 8],
    pub waveshaper_amount: f32x8,
    pub phaseshaper: [Waveshaper; 8],
    pub phaseshaper_amount: f32x8,
}
macro_rules! aos_to_soa {
    // The `tt` (token tree) designator is used for
    // operators and tokens.
    ($a:expr, $field:ident) => {
        [
            $a[0].$field,
            $a[1].$field,
            $a[2].$field,
            $a[3].$field,
            $a[4].$field,
            $a[5].$field,
            $a[6].$field,
            $a[7].$field,
        ]
    };
}
impl From<[OscParams; 8]> for OscParamsBatch {
    fn from(value: [OscParams; 8]) -> Self {
        Self {
            output_gain: f32x8::from(aos_to_soa!(value, output_gain)),
            sample_rate: f32x8::from(aos_to_soa!(value, sample_rate)),
            coarse: f32x8::from(aos_to_soa!(value, coarse)),
            fine: f32x8::from(aos_to_soa!(value, fine)),
            frequency_mult: f32x8::from(aos_to_soa!(value, frequency_mult)),
            hz_detune: f32x8::from(aos_to_soa!(value, hz_detune)),
            phase_offset: f32x8::from(aos_to_soa!(value, phase_offset)),
            phase_rand: f32x8::from(aos_to_soa!(value, phase_rand)),
            attack_level: f32x8::from(aos_to_soa!(value, attack_level)),
            release_level: f32x8::from(aos_to_soa!(value, release_level)),
            delay: f32x8::from(aos_to_soa!(value, delay)),
            attack: f32x8::from(aos_to_soa!(value, attack)),
            hold: f32x8::from(aos_to_soa!(value, hold)),
            decay: f32x8::from(aos_to_soa!(value, decay)),
            sustain: f32x8::from(aos_to_soa!(value, sustain)),
            release: f32x8::from(aos_to_soa!(value, release)),
            feedback: f32x8::from(aos_to_soa!(value, feedback)),
            velocity_sensitivity: f32x8::from(aos_to_soa!(value, velocity_sensitivity)),
            keyscaling: f32x8::from(aos_to_soa!(value, keyscaling)),
            octave_stretch: f32x8::from(aos_to_soa!(value, octave_stretch)),
            waveshaper: aos_to_soa!(value, waveshaper),
            waveshaper_amount: f32x8::from(aos_to_soa!(value, waveshaper_amount)),
            phaseshaper: aos_to_soa!(value, phaseshaper),
            phaseshaper_amount: f32x8::from(aos_to_soa!(value, phaseshaper_amount)),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct OscillatorBatch {
    pub frequency: f32x8,
    midi_id: u8,
    phase: f32x8,
    time: f32x8,
    release_time: Option<f32x8>,
    release_start_level: f32x8,
    previous_wave: [f32x8; 2],
    previous_output: f32x8,
    pub gain: f32x8,
}

impl OscillatorBatch {
    pub fn new(midi_id: u8, params: &OscParamsBatch, velocity: f32) -> Self {
        let frequency = OscillatorBatch::get_pitch(midi_id, params);
        // let keyscaling = f32x8::splat(2.0f32.powf((midi_id as f32 - 69.0) * -params.keyscaling / 12.0));
        let keyscaling = f32x8::splat(2.0f32)
            .pow_f32x8(f32x8::splat(midi_id as f32 - 69.0) * -params.keyscaling / 12.0);
        Self {
            frequency,
            midi_id,
            phase: f32x8::splat(0.0),
            time: f32x8::splat(0.0),
            release_time: None,
            release_start_level: f32x8::splat(0.0),
            previous_wave: [f32x8::splat(0.0); 2],
            previous_output: f32x8::splat(0.0),
            gain: (params.velocity_sensitivity * velocity + 1.0
                - params.velocity_sensitivity.max(f32x8::splat(0.0)))
                * keyscaling,
        }
    }
    pub fn envelope(&self, params: &OscParamsBatch) -> f32x8 {
        if let Some(released_time) = self.time_since_release() {
            Self::release_envelope(
                params.sample_rate,
                released_time,
                params.release,
                self.release_start_level,
                params.release_level,
            )
        } else {
            Self::ads_envelope(
                params.sample_rate,
                self.time,
                params.delay,
                params.attack_level,
                params.attack,
                params.hold,
                params.decay,
                params.sustain,
            )
        }
    }
    pub fn ads_envelope(
        sample_rate: f32x8,
        time: f32x8,
        delay: f32x8,
        attack_level: f32x8,
        attack: f32x8,
        hold: f32x8,
        decay: f32x8,
        sustain: f32x8,
    ) -> f32x8 {
        // let time = time. as f32 / sample_rate;
        // if time < attack {
        //     time / attack
        // } else if time < attack + decay {
        //     (1.0 - ((time - attack) / decay)).powf(2.0) * (1.0 - sustain) + sustain
        // } else {
        //     sustain
        // }
        // .max(0.0)
        let time = time / sample_rate;
        let attack_level = lerpx8(
            attack_level,
            f32x8::splat(1.0),
            (time - delay).fast_max(0.0.into()) / attack,
        ) & time.cmp_lt(delay + attack);
        let hold_level =
            f32x8::splat(1.0) & time.cmp_lt(delay + attack + hold) & time.cmp_ge(delay + attack);
        let decay_level = ((1.0 - ((time - delay - attack - hold) / decay)).powf(2.0)
            * (1.0 - sustain)
            + sustain)
            & time.cmp_lt(delay + attack + hold + decay)
            & time.cmp_ge(delay + attack + hold);
        let sustain = sustain & time.cmp_ge(delay + attack + hold + decay);
        (attack_level + hold_level + decay_level + sustain).max(0.0.into())
    }

    pub fn release_envelope(
        sample_rate: f32x8,
        time: f32x8,
        release: f32x8,
        release_start_level: f32x8,
        release_end_level: f32x8,
    ) -> f32x8 {
        let delta = time / sample_rate;
        lerpx8(
            release_end_level,
            release_start_level,
            (1.0 - delta / release)
                .fast_max(0.0.into())
                .fast_min(1.0.into())
                .powf(2.0),
        )
    }
    pub fn get_pitch(midi_id: u8, params: &OscParamsBatch) -> f32x8 {
        (f32x8::splat(2.0).pow_f32x8(
            (midi_id as f32 + params.coarse + params.fine / 100.0 - 69.0)
                / (12.0 / params.octave_stretch),
        ) * 440.0
            * params.frequency_mult
            + params.hz_detune)
            .fast_max(f32x8::splat(0.0))
    }
    pub fn update_pitch(&mut self, params: &OscParamsBatch) {
        self.frequency = OscillatorBatch::get_pitch(self.midi_id, params);
    }
    pub fn step(&mut self, params: &OscParamsBatch, pm: f32x8) -> f32x8 {
        self.time = self.time + 1.0;
        // Feedback implementation from the Surge XT FM2/FM3/Sine oscillators, which in turn were based on the DX7 feedback
        let prev = (self.previous_wave[0] + self.previous_wave[1]) / 2.0;
        // let feedback = if params.feedback.is_sign_negative() {
        //     prev.powi(2)
        // } else {
        //     prev
        // } * params.feedback.abs();
        let feedback = {
            let negative_feedback = (prev * prev) & params.feedback.cmp_lt(0.0);
            let positive_feedback = prev & params.feedback.cmp_ge(0.0);
            (negative_feedback + positive_feedback) * params.feedback.abs()
        };
        let phase = {
            let phase = self.phase + feedback + pm;
            let mut phase = phase.to_array();
            let phaseshape_amount = params.phaseshaper_amount * 0.01;
            for (phase, amount, waveshaper) in izip!(
                phase.iter_mut(),
                phaseshape_amount.as_array_ref(),
                &params.phaseshaper
            ) {
                let inner_phase = (phase.fract() + 1.0).fract();
                *phase = waveshaper.phaseshape(inner_phase, *amount);
            }
            f32x8::from(phase)
        };
        // let phase = self_phase + feedback;
        self.previous_wave[1] = self.previous_wave[0];
        let out = {
            let mut sine = (phase * std::f32::consts::TAU + params.phase_offset)
                .sin()
                .to_array();
            let waveshape_amount = params.waveshaper_amount * 0.01;
            for (sine, amount, waveshaper) in izip!(
                sine.iter_mut(),
                waveshape_amount.as_array_ref(),
                &params.waveshaper
            ) {
                *sine = waveshaper.waveshape(*sine, *amount);
            }
            f32x8::from(sine)
        };
        self.previous_wave[0] = out;
        self.add_phase(OscillatorBatch::calculate_delta(
            self.frequency,
            params.sample_rate,
        ));
        out * self.gain
    }
    pub fn step_with_envelope(&mut self, params: &OscParamsBatch, pm: f32x8) -> f32x8 {
        self.previous_output = self.step(params, pm) * self.envelope(params);
        self.previous_output * params.output_gain
    }
    pub fn release(&mut self, params: &OscParamsBatch) {
        self.release_start_level = self.envelope(params);
        self.release_time = Some(self.time);
    }
    pub fn time_since_release(&self) -> Option<f32x8> {
        if let Some(release_time) = self.release_time {
            Some(self.time - release_time)
        } else {
            None
        }
    }
    pub fn is_done(&self, params: &OscParamsBatch) -> bool {
        if let Some(released_time) = self.time_since_release() {
            (released_time / params.sample_rate)
                .cmp_ge(params.release)
                .all()
        } else {
            false
        }
    }
    pub fn calculate_delta(frequency: f32x8, sample_rate: f32x8) -> f32x8 {
        frequency / sample_rate
    }
    pub fn add_phase(&mut self, phase_delta: f32x8) {
        self.phase += phase_delta;
        // if self.phase >= 1.0 {
        //     self.phase -= 1.0;
        // }
        self.phase -= f32x8::splat(1.0) & self.phase.cmp_ge(1.0);
    }
    pub fn calculate_sine(&mut self, phase_delta: f32x8) -> f32x8 {
        let sine = (self.phase * std::f32::consts::TAU).sin();

        self.add_phase(phase_delta);

        sine
    }
    pub fn previous(&self) -> f32x8 {
        self.previous_output * self.gain
    }
}

#[derive(Debug, Clone, Copy)]
/// Generates a sawtooth from 0.0 to 1.0 at its set frequency
pub struct PhaseGenerator {
    pub frequency: f32x8,
    pub phase: f32x8,
}

impl PhaseGenerator {
    fn new(midi_id: u8, params: &OscParamsBatch, velocity: f32) -> Self {
        let frequency = OscillatorBatch::get_pitch(midi_id, params);
        Self {
            frequency,
            phase: f32x8::splat(0.0),
        }
    }
    fn calculate_delta(frequency: f32x8, sample_rate: f32x8) -> f32x8 {
        frequency / sample_rate
    }
    fn add_phase(&mut self, phase_delta: f32x8) {
        self.phase += phase_delta;
        self.phase -= f32x8::splat(1.0) & self.phase.cmp_ge(1.0);
    }
    fn update_pitch(&mut self, params: &OscParamsBatch, midi_id: u8) {
        self.frequency = OscillatorBatch::get_pitch(midi_id, params);
    }
    fn step(&mut self, sample_rate: f32x8) -> f32x8 {
        let out = self.phase;
        self.add_phase(Self::calculate_delta(self.frequency, sample_rate));
        out
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SineGenerator;
impl SineGenerator {
    pub fn calculate_sine(&mut self, phase: f32x8) -> f32x8 {
        (phase * std::f32::consts::TAU).sin()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DahdsrEnvelope {
    pub release_start_level: f32x8,
}
impl DahdsrEnvelope {
    fn envelope(&self, params: &OscParamsBatch, time: f32x8, release_time: f32x8) -> f32x8 {
        time.cmp_ge(release_time)
            & Self::release_envelope(
                release_time / params.sample_rate,
                params.release,
                self.release_start_level,
                params.release_level,
                f32x8::splat(2.0),
            ) + time.cmp_lt(release_time)
            & Self::ads_envelope(
                time / params.sample_rate,
                params.delay,
                params.attack_level,
                params.attack,
                f32x8::splat(1.0),
                params.hold,
                params.decay,
                f32x8::splat(2.0),
                params.sustain,
            )
    }
    fn ads_envelope(
        seconds: f32x8,
        delay: f32x8,
        attack_level: f32x8,
        attack_time: f32x8,
        attack_slope: f32x8,
        hold_time: f32x8,
        decay_time: f32x8,
        decay_slope: f32x8,
        sustain_level: f32x8,
    ) -> f32x8 {
        let attack_level = lerpx8(
            attack_level,
            f32x8::splat(1.0),
            ((seconds - delay).fast_max(0.0.into()) / attack_time).pow_f32x8(attack_slope),
        ) & seconds.cmp_lt(delay + attack_time);
        let hold_level = f32x8::splat(1.0)
            & seconds.cmp_lt(delay + attack_time + hold_time)
            & seconds.cmp_ge(delay + attack_time);
        let decay_level = ((1.0 - ((seconds - delay - attack_time - hold_time) / decay_time))
            .pow_f32x8(decay_slope)
            * (1.0 - sustain_level)
            + sustain_level)
            & seconds.cmp_lt(delay + attack_time + hold_time + decay_time)
            & seconds.cmp_ge(delay + attack_time + hold_time);
        let sustain = sustain_level & seconds.cmp_ge(delay + attack_time + hold_time + decay_time);
        (attack_level + hold_level + decay_level + sustain).fast_max(0.0.into())
    }

    fn release_envelope(
        seconds: f32x8,
        release_time: f32x8,
        release_start_level: f32x8,
        release_end_level: f32x8,
        slope_power: f32x8,
    ) -> f32x8 {
        lerpx8(
            release_end_level,
            release_start_level,
            (1.0 - seconds / release_time)
                .fast_max(0.0.into())
                .fast_min(1.0.into())
                .pow_f32x8(slope_power),
        )
    }
}

#[derive(Debug, Clone, Copy)]
/// SIMD acceleration attempt 2: vectorise voices instead of oscillators, so
/// we can accelerate the waveshapers.
pub struct PolyFoam {
    voice_mask: i32x8,
    voice_id: i32x8,
    time: i32x8,
    release_time: i32x8,

    phases: [PhaseGenerator; 8],
    phaseshapers: [Waveshaper; 8],
    waves: [SineGenerator; 8],
    waveshapers: [Waveshaper; 8],
    oscillator_envelopes: [DahdsrEnvelope; 8],
    last_oscillator_output: f32x8,

    filter: SvfSimperBatch,
    filter_envelope: DahdsrEnvelope,
    global_envelope: DahdsrEnvelope,
}
impl PolyFoam {
    pub fn step_oscillators(&mut self, params: OscParamsBatch) -> f32x8 {
        for (phase_gen, phaseshaper, wave, waveshaper, envelope) in izip!(
            self.phases.iter_mut(),
            self.phaseshapers.iter_mut(),
            self.waves.iter_mut(),
            self.waveshapers.iter_mut(),
            self.oscillator_envelopes.iter_mut()
        ) {
            let phase = phase_gen.step(params.sample_rate);
            let phase = phaseshaper.waveshape_batch(phase, params.phaseshaper_amount);
            let wave = wave.calculate_sine(phase);
            let wave = waveshaper.waveshape_batch(wave, params.waveshaper_amount);
            let envelope = envelope.envelope(&params, self.time.round_float(), self.release_time.round_float());
            self.last_oscillator_output = wave * envelope;
        }
        self.last_oscillator_output
    }
}

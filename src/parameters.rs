use nih_plug::prelude::*;
use nih_plug_iced::IcedState;
use wide::f32x8;
use std::f32::consts::PI;
use std::sync::Arc;

use crate::editor;
use crate::svf_simper::FilterType;
use crate::voice::{LegatoMode, Phaseshaper, Waveshaper};

const ATTACK_DECAY_RANGE: FloatRange = FloatRange::Skewed {
    min: 0.0,
    max: 20.0,
    factor: 0.4,
};
const RELEASE_RANGE: FloatRange = FloatRange::Skewed {
    min: 0.025,
    max: 20.0,
    factor: 0.4,
};
const FREQ_MULT_DIV_RANGE: FloatRange = FloatRange::Skewed {
    min: 1.0,
    max: 64.0,
    factor: 0.7,
};
const FM_RANGE: FloatRange = FloatRange::Skewed {
    min: 0.0,
    max: 1.0,
    factor: 0.4,
};
const SMOOTH_TIME: f32 = 20.0;

#[derive(Params)]
pub struct OscillatorParams {
    #[id = "amp"]
    pub amp: FloatParam,
    #[id = "coarse"]
    pub coarse: FloatParam,
    #[id = "fine"]
    pub fine: FloatParam,
    #[id = "freq_mult"]
    pub freq_mult: FloatParam,
    #[id = "freq_div"]
    pub freq_div: FloatParam,
    #[id = "hz_detune"]
    pub hz_detune: FloatParam,
    #[id = "phase_offset"]
    pub phase_offset: FloatParam,
    #[id = "phase_rand"]
    pub phase_rand: FloatParam,
    #[id = "initial_level"]
    pub attack_level: FloatParam,
    #[id = "release_level"]
    pub release_level: FloatParam,
    #[id = "delay"]
    pub delay: FloatParam,
    #[id = "attack"]
    pub attack: FloatParam,
    #[id = "hold"]
    pub hold: FloatParam,
    #[id = "decay"]
    pub decay: FloatParam,
    #[id = "sustain"]
    pub sustain: FloatParam,
    #[id = "release"]
    pub release: FloatParam,
    #[id = "feedback"]
    pub feedback: FloatParam,
    #[id = "velocity_sensitivity"]
    pub velocity_sensitivity: FloatParam,
    #[id = "keyscaling"]
    pub keyscaling: FloatParam,
    #[id = "waveshaper"]
    pub waveshaper: EnumParam<Waveshaper>,
    #[id = "waveshaper_amount"]
    pub waveshaper_amount: FloatParam,
    #[id = "phaseshaper"]
    pub phaseshaper: EnumParam<Phaseshaper>,
    #[id = "phaseshaper_amount"]
    pub phaseshaper_amount: FloatParam,
}
impl OscillatorParams {
    pub fn new(index: usize, default_amp: f32) -> Self {
        Self {
            amp: FloatParam::new(
                format!("Osc{} Amp", index + 1),
                default_amp,
                FloatRange::Linear {
                    min: 0.0,
                    max: 100.0,
                },
            )
            .with_unit("%")
            .with_smoother(SmoothingStyle::Linear(SMOOTH_TIME)),
            coarse: FloatParam::new(
                format!("Osc{} Coarse", index + 1),
                0.0,
                FloatRange::Linear {
                    min: -48.0,
                    max: 48.0,
                },
            )
            .with_step_size(1.0),
            fine: FloatParam::new(
                format!("Osc{} Fine", index + 1),
                0.0,
                FloatRange::Linear {
                    min: -100.0,
                    max: 100.0,
                },
            )
            .with_unit("%"),
            freq_mult: FloatParam::new(
                format!("Osc{} Freq Mult", index + 1),
                1.0,
                FREQ_MULT_DIV_RANGE,
            )
            .with_unit("x")
            .with_step_size(1.0),
            freq_div: FloatParam::new(
                format!("Osc{} Freq Div", index + 1),
                1.0,
                FREQ_MULT_DIV_RANGE,
            )
            .with_unit("x")
            .with_step_size(1.0),
            hz_detune: FloatParam::new(
                format!("Osc{} +/- Hz", index + 1),
                0.0,
                FloatRange::SymmetricalSkewed {
                    min: -10000.0,
                    max: 10000.0,
                    factor: 0.4,
                    center: 0.0,
                },
            )
            .with_unit(" Hz"),
            phase_offset: FloatParam::new(
                format!("Osc{} Phase", index + 1),
                0.0,
                FloatRange::Linear {
                    min: -180.0,
                    max: 180.0,
                },
            )
            .with_unit("\u{00B0}")
            .with_smoother(SmoothingStyle::Linear(SMOOTH_TIME)),
            phase_rand: FloatParam::new(
                format!("Osc{} Phase Rand", index + 1),
                0.0,
                FloatRange::Linear {
                    min: 0.0,
                    max: 100.0,
                },
            )
            .with_unit("%"),
            attack_level: FloatParam::new(
                format!("Osc{} Atk. Level", index + 1),
                0.0,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            ),
            release_level: FloatParam::new(
                format!("Osc{} Rls. Level", index + 1),
                0.0,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            ),
            delay: FloatParam::new(format!("Osc{} Delay", index + 1), 0.0, ATTACK_DECAY_RANGE)
                .with_unit(" s"),
            attack: FloatParam::new(format!("Osc{} Attack", index + 1), 0.0, ATTACK_DECAY_RANGE)
                .with_unit(" s"),
            hold: FloatParam::new(format!("Osc{} Hold", index + 1), 0.0, ATTACK_DECAY_RANGE)
                .with_unit(" s"),
            decay: FloatParam::new(format!("Osc{} Decay", index + 1), 0.5, ATTACK_DECAY_RANGE)
                .with_unit(" s"),
            sustain: FloatParam::new(
                format!("Osc{} Sustain", index + 1),
                1.0,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            ),
            release: FloatParam::new(format!("Osc{} Release", index + 1), 0.05, RELEASE_RANGE)
                .with_unit(" s"),
            feedback: FloatParam::new(
                format!("Osc{} Feedback", index + 1),
                0.0,
                FloatRange::Linear {
                    min: -1.0,
                    max: 1.0,
                },
            )
            .with_smoother(SmoothingStyle::Linear(SMOOTH_TIME)),
            velocity_sensitivity: FloatParam::new(
                format!("Osc{} Velocity Sens.", index + 1),
                0.0,
                FloatRange::Linear {
                    min: -1.0,
                    max: 1.0,
                },
            ),
            keyscaling: FloatParam::new(
                format!("Osc{} Keyscaling", index + 1),
                0.0,
                FloatRange::Linear {
                    min: -1.0,
                    max: 1.0,
                },
            ),
            waveshaper: EnumParam::new(format!("Osc{} Waveshaper", index + 1), Waveshaper::None),
            waveshaper_amount: FloatParam::new(
                format!("Osc{} Waveshape Amount", index + 1),
                0.0,
                FloatRange::Skewed {
                    min: 0.0,
                    max: 100.0,
                    factor: 0.5,
                },
            )
            .with_unit("%")
            .with_smoother(SmoothingStyle::Linear(SMOOTH_TIME)),
            phaseshaper: EnumParam::new(format!("Osc{} Phaseshaper", index + 1), Phaseshaper::None),
            phaseshaper_amount: FloatParam::new(
                format!("Osc{} Phaseshape Amount", index + 1),
                0.0,
                FloatRange::Skewed {
                    min: 0.0,
                    max: 100.0,
                    factor: 0.5,
                },
            )
            .with_unit("%")
            .with_smoother(SmoothingStyle::Linear(SMOOTH_TIME)),
        }
    }
    pub fn to_osc_params(
        &self,
        sample_rate: f32,
        octave_stretch: f32,
        block_size: u32,
    ) -> crate::voice::OscParams {
        crate::voice::OscParams {
            output_gain: self.amp.smoothed.next_step(block_size) / 100.0,
            sample_rate,
            coarse: self.coarse.value(),
            fine: self.fine.value(),
            frequency_mult: self.freq_mult.value() / self.freq_div.value(),
            hz_detune: self.hz_detune.value(),
            phase_offset: self.phase_offset.smoothed.next_step(block_size) / 180.0 * PI,
            phase_rand: self.phase_rand.value(),
            attack_level: self.attack_level.value(),
            release_level: self.release_level.value(),
            delay: self.delay.value(),
            attack: self.attack.value(),
            hold: self.hold.value(),
            decay: self.decay.value(),
            sustain: self.sustain.value(),
            release: self.release.value(),
            feedback: {
                let feedback = self.feedback.smoothed.next_step(block_size);
                feedback.signum() * feedback.powi(2)
            },
            velocity_sensitivity: self.velocity_sensitivity.value(),
            keyscaling: self.keyscaling.value(),
            octave_stretch,
            waveshaper: self.waveshaper.value(),
            waveshaper_amount: self.waveshaper_amount.smoothed.next_step(block_size),
            phaseshaper: self.phaseshaper.value(),
            phaseshaper_amount: self.phaseshaper_amount.smoothed.next_step(block_size),
        }
    }
}

#[derive(Params)]
pub struct OscMod {
    #[id = "by_osc1"]
    pub by_osc1: FloatParam,
    #[id = "by_osc2"]
    pub by_osc2: FloatParam,
    #[id = "by_osc3"]
    pub by_osc3: FloatParam,
    #[id = "by_osc4"]
    pub by_osc4: FloatParam,
    #[id = "by_osc5"]
    pub by_osc5: FloatParam,
    #[id = "by_osc6"]
    pub by_osc6: FloatParam,
    #[id = "by_osc7"]
    pub by_osc7: FloatParam,
    #[id = "by_osc8"]
    pub by_osc8: FloatParam,
}
impl OscMod {
    pub fn new(target_id: usize) -> Self {
        Self {
            by_osc1: FloatParam::new(format!("Mod Osc{target_id} by Osc1"), 0.0, FM_RANGE),
            by_osc2: FloatParam::new(format!("Mod Osc{target_id} by Osc2"), 0.0, FM_RANGE),
            by_osc3: FloatParam::new(format!("Mod Osc{target_id} by Osc3"), 0.0, FM_RANGE),
            by_osc4: FloatParam::new(format!("Mod Osc{target_id} by Osc4"), 0.0, FM_RANGE),
            by_osc5: FloatParam::new(format!("Mod Osc{target_id} by Osc5"), 0.0, FM_RANGE),
            by_osc6: FloatParam::new(format!("Mod Osc{target_id} by Osc6"), 0.0, FM_RANGE),
            by_osc7: FloatParam::new(format!("Mod Osc{target_id} by Osc7"), 0.0, FM_RANGE),
            by_osc8: FloatParam::new(format!("Mod Osc{target_id} by Osc8"), 0.0, FM_RANGE),
        }
    }
    pub fn to_array(&self) -> [f32; 8] {
        [
            self.by_osc1.value(),
            self.by_osc2.value(),
            self.by_osc3.value(),
            self.by_osc4.value(),
            self.by_osc5.value(),
            self.by_osc6.value(),
            self.by_osc7.value(),
            self.by_osc8.value(),
        ]
    }
}

#[derive(Params)]
pub struct SynthPluginParams {
    #[persist = "editor-state"]
    pub(crate) editor_state: Arc<IcedState>,

    #[id = "gain"]
    pub gain: FloatParam,
    #[id = "global_coarse"]
    pub global_coarse: FloatParam,
    #[id = "octave_multiplier"]
    pub octave_stretch: FloatParam,
    #[id = "bend_range"]
    pub bend_range: FloatParam,

    #[nested(group = "mod", id_prefix = "mod_osc1_")]
    pub osc1_fm_mod: OscMod,
    #[nested(group = "mod", id_prefix = "mod_osc2_")]
    pub osc2_fm_mod: OscMod,
    #[nested(group = "mod", id_prefix = "mod_osc3_")]
    pub osc3_fm_mod: OscMod,
    #[nested(group = "mod", id_prefix = "mod_osc4_")]
    pub osc4_fm_mod: OscMod,
    #[nested(group = "mod", id_prefix = "mod_osc5_")]
    pub osc5_fm_mod: OscMod,
    #[nested(group = "mod", id_prefix = "mod_osc6_")]
    pub osc6_fm_mod: OscMod,
    #[nested(group = "mod", id_prefix = "mod_osc7_")]
    pub osc7_fm_mod: OscMod,
    #[nested(group = "mod", id_prefix = "mod_osc8_")]
    pub osc8_fm_mod: OscMod,

    #[nested(group = "osc1", id_prefix = "osc1")]
    pub osc1_params: OscillatorParams,
    #[nested(group = "osc2", id_prefix = "osc2")]
    pub osc2_params: OscillatorParams,
    #[nested(group = "osc3", id_prefix = "osc3")]
    pub osc3_params: OscillatorParams,
    #[nested(group = "osc4", id_prefix = "osc4")]
    pub osc4_params: OscillatorParams,
    #[nested(group = "osc5", id_prefix = "osc5")]
    pub osc5_params: OscillatorParams,
    #[nested(group = "osc6", id_prefix = "osc6")]
    pub osc6_params: OscillatorParams,
    #[nested(group = "osc7", id_prefix = "osc7")]
    pub osc7_params: OscillatorParams,
    #[nested(group = "osc8", id_prefix = "osc8")]
    pub osc8_params: OscillatorParams,

    #[id = "filter_enabled"]
    pub filter_enabled: BoolParam,
    #[id = "filter_type"]
    pub filter_type: EnumParam<FilterType>,
    #[id = "filter_cutoff"]
    pub filter_cutoff: FloatParam,
    #[id = "filter_resonance"]
    pub filter_resonance: FloatParam,
    #[id = "filter_keytrack"]
    pub filter_keytrack: FloatParam,
    #[id = "filter_envelope_enabled"]
    pub filter_envelope_amount: FloatParam,
    #[id = "filter_envelope_attack"]
    pub filter_envelope_attack: FloatParam,
    #[id = "filter_envelope_decay"]
    pub filter_envelope_decay: FloatParam,
    #[id = "filter_envelope_sustain"]
    pub filter_envelope_sustain: FloatParam,
    #[id = "filter_envelope_release"]
    pub filter_envelope_release: FloatParam,

    #[id = "global_attack"]
    pub global_attack: FloatParam,
    #[id = "global_decay"]
    pub global_decay: FloatParam,
    #[id = "global_sustain"]
    pub global_sustain: FloatParam,
    #[id = "global_release"]
    pub global_release: FloatParam,
}

impl Default for SynthPluginParams {
    fn default() -> Self {
        Self {
            editor_state: editor::default_state(),
            // This gain is stored as linear gain. NIH-plug comes with useful conversion functions
            // to treat these kinds of parameters as if we were dealing with decibels. Storing this
            // as decibels is easier to work with, but requires a conversion for every sample.
            gain: FloatParam::new(
                "Gain",
                util::db_to_gain(-18.0),
                FloatRange::Skewed {
                    min: util::db_to_gain(-70.0),
                    max: util::db_to_gain(0.0),
                    // This makes the range appear as if it was linear when displaying the values as
                    // decibels
                    factor: FloatRange::gain_skew_factor(-70.0, 0.0),
                },
            )
            // Because the gain parameter is stored as linear gain instead of storing the value as
            // decibels, we need logarithmic smoothing
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit(" dB")
            // There are many predefined formatters we can use here. If the gain was stored as
            // decibels instead of as a linear gain value, we could have also used the
            // `.with_step_size(0.1)` function to get internal rounding.
            .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
            .with_string_to_value(formatters::s2v_f32_gain_to_db()),
            global_coarse: FloatParam::new(
                "Glob. Coarse",
                0.0,
                FloatRange::Linear {
                    min: -48.0,
                    max: 48.0,
                },
            )
            .with_step_size(1.0),
            octave_stretch: FloatParam::new(
                "Octave Stretch",
                1.0,
                FloatRange::Linear {
                    min: 0.99,
                    max: 1.01,
                },
            )
            .with_unit("x"),
            bend_range: FloatParam::new(
                "Bend Range",
                2.0,
                FloatRange::Linear {
                    min: 0.0,
                    max: 48.0,
                },
            )
            .with_step_size(1.0),

            osc1_fm_mod: OscMod::new(1),
            osc2_fm_mod: OscMod::new(2),
            osc3_fm_mod: OscMod::new(3),
            osc4_fm_mod: OscMod::new(4),
            osc5_fm_mod: OscMod::new(5),
            osc6_fm_mod: OscMod::new(6),
            osc7_fm_mod: OscMod::new(7),
            osc8_fm_mod: OscMod::new(8),

            osc1_params: OscillatorParams::new(0, 100.0),
            osc2_params: OscillatorParams::new(1, 0.0),
            osc3_params: OscillatorParams::new(2, 0.0),
            osc4_params: OscillatorParams::new(3, 0.0),
            osc5_params: OscillatorParams::new(4, 0.0),
            osc6_params: OscillatorParams::new(5, 0.0),
            osc7_params: OscillatorParams::new(6, 0.0),
            osc8_params: OscillatorParams::new(7, 0.0),

            filter_enabled: BoolParam::new("Filter Enabled", true),
            filter_type: EnumParam::new("Filter Type", FilterType::Lowpass),
            filter_cutoff: FloatParam::new(
                "Filter Cutoff",
                22000.0,
                FloatRange::Skewed {
                    min: 20.0,
                    max: 22000.0,
                    factor: FloatRange::skew_factor(-2.0),
                },
            )
            .with_unit(" Hz"),
            filter_resonance: FloatParam::new(
                "Filter Resonance",
                0.3,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            ),
            filter_keytrack: FloatParam::new(
                "Filter Keytrack",
                0.0,
                FloatRange::Linear {
                    min: -1.0,
                    max: 1.0,
                },
            ),
            filter_envelope_amount: FloatParam::new(
                "Filter Env. Amount",
                0.0,
                FloatRange::Linear {
                    min: -1.0,
                    max: 1.0,
                },
            ),
            filter_envelope_attack: FloatParam::new("Filter Env. Attack", 0.0, ATTACK_DECAY_RANGE)
                .with_unit(" s"),
            filter_envelope_decay: FloatParam::new("Filter Env. Decay", 0.5, ATTACK_DECAY_RANGE)
                .with_unit(" s"),
            filter_envelope_sustain: FloatParam::new(
                "Filter Env. Sustain",
                0.0,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            ),
            filter_envelope_release: FloatParam::new("Filter Env. Release", 0.05, RELEASE_RANGE)
                .with_unit(" s"),

            global_attack: FloatParam::new("Global Attack", 0.0, ATTACK_DECAY_RANGE)
                .with_unit(" s"),
            global_decay: FloatParam::new("Global Decay", 0.5, ATTACK_DECAY_RANGE).with_unit(" s"),
            global_sustain: FloatParam::new(
                "Global Sustain",
                1.0,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            ),
            global_release: FloatParam::new("Global Release", 0.05, RELEASE_RANGE).with_unit(" s"),
        }
    }
}

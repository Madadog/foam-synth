use nih_plug::prelude::*;
use nih_plug_iced::IcedState;
use std::sync::Arc;

use crate::editor;
use crate::svf_simper::FilterType;

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
    factor: 0.7,
};

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
    #[id = "attack"]
    pub attack: FloatParam,
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
}
impl OscillatorParams {
    pub fn new(index: usize, default_amp: f32) -> Self {
        Self {
            amp: FloatParam::new(
                dbg!(format!("Osc{} Amp", dbg!(index + 1))),
                default_amp,
                FloatRange::Linear {
                    min: 0.0,
                    max: 100.0,
                },
            ),
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
            .with_step_size(1.0),
            freq_div: FloatParam::new(
                format!("Osc{} Freq Div", index + 1),
                1.0,
                FREQ_MULT_DIV_RANGE,
            )
            .with_step_size(1.0),
            attack: FloatParam::new(format!("Osc{} Attack", index + 1), 0.0, ATTACK_DECAY_RANGE)
                .with_unit("s"),
            decay: FloatParam::new(format!("Osc{} Decay", index + 1), 0.5, ATTACK_DECAY_RANGE)
                .with_unit("s"),
            sustain: FloatParam::new(
                format!("Osc{} Sustain", index + 1),
                1.0,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            ),
            release: FloatParam::new(format!("Osc{} Release", index + 1), 0.05, RELEASE_RANGE)
                .with_unit("s"),
            feedback: FloatParam::new(
                format!("Osc{} Feedback", index + 1),
                0.0,
                FloatRange::Linear {
                    min: -1.0,
                    max: 1.0,
                },
            ),
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
        }
    }
    pub fn to_osc_params(&self, sample_rate: f32, octave_stretch: f32) -> crate::voice::OscParams {
        crate::voice::OscParams {
            output_gain: self.amp.value() / 100.0,
            sample_rate: sample_rate,
            coarse: self.coarse.value(),
            fine: self.fine.value(),
            frequency_mult: self.freq_mult.value() / self.freq_div.value(),
            initial_phase: 0.0,
            attack: self.attack.value(),
            decay: self.decay.value(),
            sustain: self.sustain.value(),
            release: self.release.value(),
            feedback: self.feedback.value().signum() * self.feedback.value().powi(2),
            velocity_sensitivity: self.velocity_sensitivity.value(),
            keyscaling: self.keyscaling.value(),
            octave_stretch: octave_stretch,
        }
    }
}

#[derive(Params)]
pub struct SynthPluginParams {
    #[persist = "editor-state"]
    pub(crate) editor_state: Arc<IcedState>,

    #[id = "gain"]
    pub gain: FloatParam,
    #[id = "octave_multiplier"]
    pub octave_stretch: FloatParam,

    #[id = "mod_osc1_by_osc2"]
    pub mod_osc1_by_osc2: FloatParam,
    #[id = "mod_osc1_by_osc3"]
    pub mod_osc1_by_osc3: FloatParam,
    #[id = "mod_osc1_by_osc4"]
    pub mod_osc1_by_osc4: FloatParam,
    #[id = "mod_osc1_by_osc5"]
    pub mod_osc1_by_osc5: FloatParam,
    #[id = "mod_osc1_by_osc6"]
    pub mod_osc1_by_osc6: FloatParam,

    #[id = "mod_osc2_by_osc1"]
    pub mod_osc2_by_osc1: FloatParam,
    #[id = "mod_osc2_by_osc3"]
    pub mod_osc2_by_osc3: FloatParam,
    #[id = "mod_osc2_by_osc4"]
    pub mod_osc2_by_osc4: FloatParam,
    #[id = "mod_osc2_by_osc5"]
    pub mod_osc2_by_osc5: FloatParam,
    #[id = "mod_osc2_by_osc6"]
    pub mod_osc2_by_osc6: FloatParam,

    #[id = "mod_osc3_by_osc1"]
    pub mod_osc3_by_osc1: FloatParam,
    #[id = "mod_osc3_by_osc2"]
    pub mod_osc3_by_osc2: FloatParam,
    #[id = "mod_osc3_by_osc4"]
    pub mod_osc3_by_osc4: FloatParam,
    #[id = "mod_osc3_by_osc5"]
    pub mod_osc3_by_osc5: FloatParam,
    #[id = "mod_osc3_by_osc6"]
    pub mod_osc3_by_osc6: FloatParam,

    #[id = "mod_osc4_by_osc1"]
    pub mod_osc4_by_osc1: FloatParam,
    #[id = "mod_osc4_by_osc2"]
    pub mod_osc4_by_osc2: FloatParam,
    #[id = "mod_osc4_by_osc3"]
    pub mod_osc4_by_osc3: FloatParam,
    #[id = "mod_osc4_by_osc5"]
    pub mod_osc4_by_osc5: FloatParam,
    #[id = "mod_osc4_by_osc6"]
    pub mod_osc4_by_osc6: FloatParam,

    #[id = "mod_osc5_by_osc1"]
    pub mod_osc5_by_osc1: FloatParam,
    #[id = "mod_osc5_by_osc2"]
    pub mod_osc5_by_osc2: FloatParam,
    #[id = "mod_osc5_by_osc3"]
    pub mod_osc5_by_osc3: FloatParam,
    #[id = "mod_osc5_by_osc4"]
    pub mod_osc5_by_osc4: FloatParam,
    #[id = "mod_osc5_by_osc6"]
    pub mod_osc5_by_osc6: FloatParam,

    #[id = "mod_osc6_by_osc1"]
    pub mod_osc6_by_osc1: FloatParam,
    #[id = "mod_osc6_by_osc2"]
    pub mod_osc6_by_osc2: FloatParam,
    #[id = "mod_osc6_by_osc3"]
    pub mod_osc6_by_osc3: FloatParam,
    #[id = "mod_osc6_by_osc4"]
    pub mod_osc6_by_osc4: FloatParam,
    #[id = "mod_osc6_by_osc5"]
    pub mod_osc6_by_osc5: FloatParam,

    #[nested(group = "osc1")]
    pub osc1_params: OscillatorParams,

    #[id = "osc2_amp"]
    pub osc2_amp: FloatParam,
    #[id = "osc2_coarse"]
    pub osc2_coarse: FloatParam,
    #[id = "osc2_fine"]
    pub osc2_fine: FloatParam,
    #[id = "osc2_freq_mult"]
    pub osc2_freq_mult: FloatParam,
    #[id = "osc2_freq_div"]
    pub osc2_freq_div: FloatParam,
    #[id = "osc2_attack"]
    pub osc2_attack: FloatParam,
    #[id = "osc2_decay"]
    pub osc2_decay: FloatParam,
    #[id = "osc2_sustain"]
    pub osc2_sustain: FloatParam,
    #[id = "osc2_release"]
    pub osc2_release: FloatParam,
    #[id = "osc2_feedback"]
    pub osc2_feedback: FloatParam,
    #[id = "osc2_velocity_sensitivity"]
    pub osc2_velocity_sensitivity: FloatParam,
    #[id = "osc2_keyscaling"]
    pub osc2_keyscaling: FloatParam,

    #[id = "osc3_amp"]
    pub osc3_amp: FloatParam,
    #[id = "osc3_coarse"]
    pub osc3_coarse: FloatParam,
    #[id = "osc3_fine"]
    pub osc3_fine: FloatParam,
    #[id = "osc3_freq_mult"]
    pub osc3_freq_mult: FloatParam,
    #[id = "osc3_freq_div"]
    pub osc3_freq_div: FloatParam,
    #[id = "osc3_attack"]
    pub osc3_attack: FloatParam,
    #[id = "osc3_decay"]
    pub osc3_decay: FloatParam,
    #[id = "osc3_sustain"]
    pub osc3_sustain: FloatParam,
    #[id = "osc3_release"]
    pub osc3_release: FloatParam,
    #[id = "osc3_feedback"]
    pub osc3_feedback: FloatParam,
    #[id = "osc3_velocity_sensitivity"]
    pub osc3_velocity_sensitivity: FloatParam,
    #[id = "osc3_keyscaling"]
    pub osc3_keyscaling: FloatParam,

    #[id = "osc4_amp"]
    pub osc4_amp: FloatParam,
    #[id = "osc4_coarse"]
    pub osc4_coarse: FloatParam,
    #[id = "osc4_fine"]
    pub osc4_fine: FloatParam,
    #[id = "osc4_freq_mult"]
    pub osc4_freq_mult: FloatParam,
    #[id = "osc4_freq_div"]
    pub osc4_freq_div: FloatParam,
    #[id = "osc4_attack"]
    pub osc4_attack: FloatParam,
    #[id = "osc4_decay"]
    pub osc4_decay: FloatParam,
    #[id = "osc4_sustain"]
    pub osc4_sustain: FloatParam,
    #[id = "osc4_release"]
    pub osc4_release: FloatParam,
    #[id = "osc4_feedback"]
    pub osc4_feedback: FloatParam,
    #[id = "osc4_velocity_sensitivity"]
    pub osc4_velocity_sensitivity: FloatParam,
    #[id = "osc4_keyscaling"]
    pub osc4_keyscaling: FloatParam,

    #[id = "osc5_amp"]
    pub osc5_amp: FloatParam,
    #[id = "osc5_coarse"]
    pub osc5_coarse: FloatParam,
    #[id = "osc5_fine"]
    pub osc5_fine: FloatParam,
    #[id = "osc5_freq_mult"]
    pub osc5_freq_mult: FloatParam,
    #[id = "osc5_freq_div"]
    pub osc5_freq_div: FloatParam,
    #[id = "osc5_attack"]
    pub osc5_attack: FloatParam,
    #[id = "osc5_decay"]
    pub osc5_decay: FloatParam,
    #[id = "osc5_sustain"]
    pub osc5_sustain: FloatParam,
    #[id = "osc5_release"]
    pub osc5_release: FloatParam,
    #[id = "osc5_feedback"]
    pub osc5_feedback: FloatParam,
    #[id = "osc5_velocity_sensitivity"]
    pub osc5_velocity_sensitivity: FloatParam,
    #[id = "osc5_keyscaling"]
    pub osc5_keyscaling: FloatParam,

    #[id = "osc6_amp"]
    pub osc6_amp: FloatParam,
    #[id = "osc6_coarse"]
    pub osc6_coarse: FloatParam,
    #[id = "osc6_fine"]
    pub osc6_fine: FloatParam,
    #[id = "osc6_freq_mult"]
    pub osc6_freq_mult: FloatParam,
    #[id = "osc6_freq_div"]
    pub osc6_freq_div: FloatParam,
    #[id = "osc6_attack"]
    pub osc6_attack: FloatParam,
    #[id = "osc6_decay"]
    pub osc6_decay: FloatParam,
    #[id = "osc6_sustain"]
    pub osc6_sustain: FloatParam,
    #[id = "osc6_release"]
    pub osc6_release: FloatParam,
    #[id = "osc6_feedback"]
    pub osc6_feedback: FloatParam,
    #[id = "osc6_velocity_sensitivity"]
    pub osc6_velocity_sensitivity: FloatParam,
    #[id = "osc6_keyscaling"]
    pub osc6_keyscaling: FloatParam,

    #[nested(group = "osc7")]
    pub osc7_params: OscillatorParams,

    #[nested(group = "osc8")]
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
                util::db_to_gain(-12.0),
                FloatRange::Skewed {
                    min: util::db_to_gain(-70.0),
                    max: util::db_to_gain(0.0),
                    // This makes the range appear as if it was linear when displaying the values as
                    // decibels
                    factor: 1.0,
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
            octave_stretch: FloatParam::new(
                "Octave Stretch",
                1.0,
                FloatRange::Linear {
                    min: 0.99,
                    max: 1.01,
                },
            ),

            mod_osc1_by_osc2: FloatParam::new("Mod Osc1 by Osc2", 0.0, FM_RANGE),
            mod_osc1_by_osc3: FloatParam::new("Mod Osc1 by Osc3", 0.0, FM_RANGE),
            mod_osc1_by_osc4: FloatParam::new("Mod Osc1 by Osc4", 0.0, FM_RANGE),
            mod_osc1_by_osc5: FloatParam::new("Mod Osc1 by Osc5", 0.0, FM_RANGE),
            mod_osc1_by_osc6: FloatParam::new("Mod Osc1 by Osc6", 0.0, FM_RANGE),

            mod_osc2_by_osc1: FloatParam::new("Mod Osc2 by Osc1", 0.0, FM_RANGE),
            mod_osc2_by_osc3: FloatParam::new("Mod Osc2 by Osc3", 0.0, FM_RANGE),
            mod_osc2_by_osc4: FloatParam::new("Mod Osc2 by Osc4", 0.0, FM_RANGE),
            mod_osc2_by_osc5: FloatParam::new("Mod Osc2 by Osc5", 0.0, FM_RANGE),
            mod_osc2_by_osc6: FloatParam::new("Mod Osc2 by Osc6", 0.0, FM_RANGE),

            mod_osc3_by_osc1: FloatParam::new("Mod Osc3 by Osc1", 0.0, FM_RANGE),
            mod_osc3_by_osc2: FloatParam::new("Mod Osc3 by Osc2", 0.0, FM_RANGE),
            mod_osc3_by_osc4: FloatParam::new("Mod Osc3 by Osc4", 0.0, FM_RANGE),
            mod_osc3_by_osc5: FloatParam::new("Mod Osc3 by Osc5", 0.0, FM_RANGE),
            mod_osc3_by_osc6: FloatParam::new("Mod Osc3 by Osc6", 0.0, FM_RANGE),

            mod_osc4_by_osc1: FloatParam::new("Mod Osc4 by Osc1", 0.0, FM_RANGE),
            mod_osc4_by_osc2: FloatParam::new("Mod Osc4 by Osc2", 0.0, FM_RANGE),
            mod_osc4_by_osc3: FloatParam::new("Mod Osc4 by Osc3", 0.0, FM_RANGE),
            mod_osc4_by_osc5: FloatParam::new("Mod Osc4 by Osc5", 0.0, FM_RANGE),
            mod_osc4_by_osc6: FloatParam::new("Mod Osc4 by Osc6", 0.0, FM_RANGE),

            mod_osc5_by_osc1: FloatParam::new("Mod Osc5 by Osc1", 0.0, FM_RANGE),
            mod_osc5_by_osc2: FloatParam::new("Mod Osc5 by Osc2", 0.0, FM_RANGE),
            mod_osc5_by_osc3: FloatParam::new("Mod Osc5 by Osc3", 0.0, FM_RANGE),
            mod_osc5_by_osc4: FloatParam::new("Mod Osc5 by Osc4", 0.0, FM_RANGE),
            mod_osc5_by_osc6: FloatParam::new("Mod Osc5 by Osc6", 0.0, FM_RANGE),

            mod_osc6_by_osc1: FloatParam::new("Mod Osc6 by Osc1", 0.0, FM_RANGE),
            mod_osc6_by_osc2: FloatParam::new("Mod Osc6 by Osc2", 0.0, FM_RANGE),
            mod_osc6_by_osc3: FloatParam::new("Mod Osc6 by Osc3", 0.0, FM_RANGE),
            mod_osc6_by_osc4: FloatParam::new("Mod Osc6 by Osc4", 0.0, FM_RANGE),
            mod_osc6_by_osc5: FloatParam::new("Mod Osc6 by Osc5", 0.0, FM_RANGE),

            osc1_params: OscillatorParams::new(0, 100.0),

            osc2_amp: FloatParam::new(
                "Osc2 Amp",
                0.0,
                FloatRange::Linear {
                    min: 0.0,
                    max: 100.0,
                },
            ),
            osc2_coarse: FloatParam::new(
                "Osc2 Coarse",
                0.0,
                FloatRange::Linear {
                    min: -48.0,
                    max: 48.0,
                },
            )
            .with_step_size(1.0),
            osc2_fine: FloatParam::new(
                "Osc2 Fine",
                0.0,
                FloatRange::Linear {
                    min: -100.0,
                    max: 100.0,
                },
            )
            .with_unit("%"),
            osc2_freq_mult: FloatParam::new("Osc2 Freq Mult", 1.0, FREQ_MULT_DIV_RANGE)
                .with_step_size(1.0),
            osc2_freq_div: FloatParam::new("Osc2 Freq Div", 1.0, FREQ_MULT_DIV_RANGE)
                .with_step_size(1.0),
            osc2_attack: FloatParam::new("Osc2 Attack", 0.0, ATTACK_DECAY_RANGE).with_unit("s"),
            osc2_decay: FloatParam::new("Osc2 Decay", 0.5, ATTACK_DECAY_RANGE).with_unit("s"),
            osc2_sustain: FloatParam::new(
                "Osc2 Sustain",
                1.0,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            ),
            osc2_release: FloatParam::new("Osc2 Release", 0.05, RELEASE_RANGE).with_unit("s"),
            osc2_feedback: FloatParam::new(
                "Osc2 Feedback",
                0.0,
                FloatRange::Linear {
                    min: -1.0,
                    max: 1.0,
                },
            ),
            osc2_velocity_sensitivity: FloatParam::new(
                "Osc2 Velocity Sens.",
                0.0,
                FloatRange::Linear {
                    min: -1.0,
                    max: 1.0,
                },
            ),
            osc2_keyscaling: FloatParam::new(
                "Osc2 Keyscaling",
                0.0,
                FloatRange::Linear {
                    min: -1.0,
                    max: 1.0,
                },
            ),

            osc3_amp: FloatParam::new(
                "Osc3 Amp",
                0.0,
                FloatRange::Linear {
                    min: 0.0,
                    max: 100.0,
                },
            ),
            osc3_coarse: FloatParam::new(
                "Osc3 Coarse",
                0.0,
                FloatRange::Linear {
                    min: -48.0,
                    max: 48.0,
                },
            )
            .with_step_size(1.0),
            osc3_fine: FloatParam::new(
                "Osc3 Fine",
                0.0,
                FloatRange::Linear {
                    min: -100.0,
                    max: 100.0,
                },
            )
            .with_unit("%"),
            osc3_freq_mult: FloatParam::new("Osc3 Freq Mult", 1.0, FREQ_MULT_DIV_RANGE)
                .with_step_size(1.0),
            osc3_freq_div: FloatParam::new("Osc3 Freq Div", 1.0, FREQ_MULT_DIV_RANGE)
                .with_step_size(1.0),
            osc3_attack: FloatParam::new("Osc3 Attack", 0.0, ATTACK_DECAY_RANGE).with_unit("s"),
            osc3_decay: FloatParam::new("Osc3 Decay", 0.5, ATTACK_DECAY_RANGE).with_unit("s"),
            osc3_sustain: FloatParam::new(
                "Osc3 Sustain",
                1.0,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            ),
            osc3_release: FloatParam::new("Osc3 Release", 0.05, RELEASE_RANGE).with_unit("s"),
            osc3_feedback: FloatParam::new(
                "Osc3 Feedback",
                0.0,
                FloatRange::Linear {
                    min: -1.0,
                    max: 1.0,
                },
            ),
            osc3_velocity_sensitivity: FloatParam::new(
                "Osc3 Velocity Sens.",
                0.0,
                FloatRange::Linear {
                    min: -1.0,
                    max: 1.0,
                },
            ),
            osc3_keyscaling: FloatParam::new(
                "Osc3 Keyscaling",
                0.0,
                FloatRange::Linear {
                    min: -1.0,
                    max: 1.0,
                },
            ),

            osc4_amp: FloatParam::new(
                "Osc4 Amp",
                0.0,
                FloatRange::Linear {
                    min: 0.0,
                    max: 100.0,
                },
            ),
            osc4_coarse: FloatParam::new(
                "Osc4 Coarse",
                0.0,
                FloatRange::Linear {
                    min: -48.0,
                    max: 48.0,
                },
            )
            .with_step_size(1.0),
            osc4_fine: FloatParam::new(
                "Osc4 Fine",
                0.0,
                FloatRange::Linear {
                    min: -100.0,
                    max: 100.0,
                },
            )
            .with_unit("%"),
            osc4_freq_mult: FloatParam::new("Osc4 Freq Mult", 1.0, FREQ_MULT_DIV_RANGE)
                .with_step_size(1.0),
            osc4_freq_div: FloatParam::new("Osc4 Freq Div", 1.0, FREQ_MULT_DIV_RANGE)
                .with_step_size(1.0),
            osc4_attack: FloatParam::new("Osc4 Attack", 0.0, ATTACK_DECAY_RANGE).with_unit("s"),
            osc4_decay: FloatParam::new("Osc4 Decay", 0.5, ATTACK_DECAY_RANGE).with_unit("s"),
            osc4_sustain: FloatParam::new(
                "Osc4 Sustain",
                1.0,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            ),
            osc4_release: FloatParam::new("Osc4 Release", 0.05, RELEASE_RANGE).with_unit("s"),
            osc4_feedback: FloatParam::new(
                "Osc4 Feedback",
                0.0,
                FloatRange::Linear {
                    min: -1.0,
                    max: 1.0,
                },
            ),
            osc4_velocity_sensitivity: FloatParam::new(
                "Osc4 Velocity Sens.",
                0.0,
                FloatRange::Linear {
                    min: -1.0,
                    max: 1.0,
                },
            ),
            osc4_keyscaling: FloatParam::new(
                "Osc4 Keyscaling",
                0.0,
                FloatRange::Linear {
                    min: -1.0,
                    max: 1.0,
                },
            ),

            osc5_amp: FloatParam::new(
                "Osc5 Amp",
                0.0,
                FloatRange::Linear {
                    min: 0.0,
                    max: 100.0,
                },
            ),
            osc5_coarse: FloatParam::new(
                "Osc5 Coarse",
                0.0,
                FloatRange::Linear {
                    min: -48.0,
                    max: 48.0,
                },
            )
            .with_step_size(1.0),
            osc5_fine: FloatParam::new(
                "Osc5 Fine",
                0.0,
                FloatRange::Linear {
                    min: -100.0,
                    max: 100.0,
                },
            )
            .with_unit("%"),
            osc5_freq_mult: FloatParam::new("Osc5 Freq Mult", 1.0, FREQ_MULT_DIV_RANGE)
                .with_step_size(1.0),
            osc5_freq_div: FloatParam::new("Osc5 Freq Div", 1.0, FREQ_MULT_DIV_RANGE)
                .with_step_size(1.0),
            osc5_attack: FloatParam::new("Osc5 Attack", 0.0, ATTACK_DECAY_RANGE).with_unit("s"),
            osc5_decay: FloatParam::new("Osc5 Decay", 0.5, ATTACK_DECAY_RANGE).with_unit("s"),
            osc5_sustain: FloatParam::new(
                "Osc5 Sustain",
                1.0,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            ),
            osc5_release: FloatParam::new("Osc5 Release", 0.05, RELEASE_RANGE).with_unit("s"),
            osc5_feedback: FloatParam::new(
                "Osc5 Feedback",
                0.0,
                FloatRange::Linear {
                    min: -1.0,
                    max: 1.0,
                },
            ),
            osc5_velocity_sensitivity: FloatParam::new(
                "Osc5 Velocity Sens.",
                0.0,
                FloatRange::Linear {
                    min: -1.0,
                    max: 1.0,
                },
            ),
            osc5_keyscaling: FloatParam::new(
                "Osc5 Keyscaling",
                0.0,
                FloatRange::Linear {
                    min: -1.0,
                    max: 1.0,
                },
            ),

            osc6_amp: FloatParam::new(
                "Osc6 Amp",
                0.0,
                FloatRange::Linear {
                    min: 0.0,
                    max: 100.0,
                },
            ),
            osc6_coarse: FloatParam::new(
                "Osc6 Coarse",
                0.0,
                FloatRange::Linear {
                    min: -48.0,
                    max: 48.0,
                },
            )
            .with_step_size(1.0),
            osc6_fine: FloatParam::new(
                "Osc6 Fine",
                0.0,
                FloatRange::Linear {
                    min: -100.0,
                    max: 100.0,
                },
            )
            .with_unit("%"),
            osc6_freq_mult: FloatParam::new("Osc6 Freq Mult", 1.0, FREQ_MULT_DIV_RANGE)
                .with_step_size(1.0),
            osc6_freq_div: FloatParam::new("Osc6 Freq Div", 1.0, FREQ_MULT_DIV_RANGE)
                .with_step_size(1.0),
            osc6_attack: FloatParam::new("Osc6 Attack", 0.0, ATTACK_DECAY_RANGE).with_unit("s"),
            osc6_decay: FloatParam::new("Osc6 Decay", 0.5, ATTACK_DECAY_RANGE).with_unit("s"),
            osc6_sustain: FloatParam::new(
                "Osc6 Sustain",
                1.0,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            ),
            osc6_release: FloatParam::new("Osc6 Release", 0.05, RELEASE_RANGE).with_unit("s"),
            osc6_feedback: FloatParam::new(
                "Osc6 Feedback",
                0.0,
                FloatRange::Linear {
                    min: -1.0,
                    max: 1.0,
                },
            ),
            osc6_velocity_sensitivity: FloatParam::new(
                "Osc6 Velocity Sens.",
                0.0,
                FloatRange::Linear {
                    min: -1.0,
                    max: 1.0,
                },
            ),
            osc6_keyscaling: FloatParam::new(
                "Osc6 Keyscaling",
                0.0,
                FloatRange::Linear {
                    min: -1.0,
                    max: 1.0,
                },
            ),
            osc7_params: OscillatorParams::new(6, 0.0),
            osc8_params: OscillatorParams::new(7, 0.0),

            filter_enabled: BoolParam::new("Filter Enabled", true),
            filter_type: EnumParam::new("Filter Type", FilterType::Lowpass),
            filter_cutoff: FloatParam::new(
                "Filter Cutoff",
                22000.0,
                FloatRange::Linear {
                    min: 20.0,
                    max: 22000.0,
                },
            ),
            filter_resonance: FloatParam::new(
                "Filter Resonance",
                0.0,
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
            filter_envelope_attack: FloatParam::new("Filter Env. Attack", 0.0, ATTACK_DECAY_RANGE),
            filter_envelope_decay: FloatParam::new("Filter Env. Decay", 0.5, ATTACK_DECAY_RANGE),
            filter_envelope_sustain: FloatParam::new(
                "Filter Env. Sustain",
                0.0,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            ),
            filter_envelope_release: FloatParam::new("Filter Env. Release", 0.05, RELEASE_RANGE),
        }
    }
}

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

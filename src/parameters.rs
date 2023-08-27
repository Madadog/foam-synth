use nih_plug::prelude::*;

#[derive(Params)]
pub struct SynthPluginParams {
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

    #[id = "mod_osc2_by_osc1"]
    pub mod_osc2_by_osc1: FloatParam,
    #[id = "mod_osc2_by_osc3"]
    pub mod_osc2_by_osc3: FloatParam,
    #[id = "mod_osc2_by_osc4"]
    pub mod_osc2_by_osc4: FloatParam,

    #[id = "mod_osc3_by_osc1"]
    pub mod_osc3_by_osc1: FloatParam,
    #[id = "mod_osc3_by_osc2"]
    pub mod_osc3_by_osc2: FloatParam,
    #[id = "mod_osc3_by_osc4"]
    pub mod_osc3_by_osc4: FloatParam,

    #[id = "mod_osc4_by_osc1"]
    pub mod_osc4_by_osc1: FloatParam,
    #[id = "mod_osc4_by_osc2"]
    pub mod_osc4_by_osc2: FloatParam,
    #[id = "mod_osc4_by_osc3"]
    pub mod_osc4_by_osc3: FloatParam,

    #[id = "osc1_amp"]
    pub osc1_amp: FloatParam,
    #[id = "osc1_coarse"]
    pub osc1_coarse: FloatParam,
    #[id = "osc1_fine"]
    pub osc1_fine: FloatParam,
    #[id = "osc1_freq_mult"]
    pub osc1_freq_mult: FloatParam,
    #[id = "osc1_freq_div"]
    pub osc1_freq_div: FloatParam,
    #[id = "osc1_attack"]
    pub osc1_attack: FloatParam,
    #[id = "osc1_decay"]
    pub osc1_decay: FloatParam,
    #[id = "osc1_sustain"]
    pub osc1_sustain: FloatParam,
    #[id = "osc1_release"]
    pub osc1_release: FloatParam,
    #[id = "osc1_feedback"]
    pub osc1_feedback: FloatParam,
    #[id = "osc1_velocity_sensitivity"]
    pub osc1_velocity_sensitivity: FloatParam,
    #[id = "osc1_keyscaling"]
    pub osc1_keyscaling: FloatParam,

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
}

impl Default for SynthPluginParams {
    fn default() -> Self {
        Self {
            // This gain is stored as linear gain. NIH-plug comes with useful conversion functions
            // to treat these kinds of parameters as if we were dealing with decibels. Storing this
            // as decibels is easier to work with, but requires a conversion for every sample.
            gain: FloatParam::new(
                "Gain",
                util::db_to_gain(-12.0),
                FloatRange::Skewed {
                    min: util::db_to_gain(-70.0),
                    max: util::db_to_gain(24.0),
                    // This makes the range appear as if it was linear when displaying the values as
                    // decibels
                    factor: FloatRange::gain_skew_factor(-70.0, 24.0),
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
                    min: 0.995,
                    max: 1.005,
                },
            ),

            mod_osc1_by_osc2: FloatParam::new(
                "Mod Osc1 by Osc2",
                0.0,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            ),
            mod_osc1_by_osc3: FloatParam::new(
                "Mod Osc1 by Osc3",
                0.0,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            ),
            mod_osc1_by_osc4: FloatParam::new(
                "Mod Osc1 by Osc4",
                0.0,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            ),

            mod_osc2_by_osc1: FloatParam::new(
                "Mod Osc2 by Osc1",
                0.0,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            ),
            mod_osc2_by_osc3: FloatParam::new(
                "Mod Osc2 by Osc3",
                0.0,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            ),
            mod_osc2_by_osc4: FloatParam::new(
                "Mod Osc2 by Osc4",
                0.0,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            ),

            mod_osc3_by_osc1: FloatParam::new(
                "Mod Osc3 by Osc1",
                0.0,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            ),
            mod_osc3_by_osc2: FloatParam::new(
                "Mod Osc3 by Osc2",
                0.0,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            ),
            mod_osc3_by_osc4: FloatParam::new(
                "Mod Osc3 by Osc4",
                0.0,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            ),

            mod_osc4_by_osc1: FloatParam::new(
                "Mod Osc4 by Osc1",
                0.0,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            ),
            mod_osc4_by_osc2: FloatParam::new(
                "Mod Osc4 by Osc2",
                0.0,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            ),
            mod_osc4_by_osc3: FloatParam::new(
                "Mod Osc4 by Osc3",
                0.0,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            ),

            osc1_amp: FloatParam::new(
                "Osc1 Amp",
                100.0,
                FloatRange::Linear {
                    min: 0.0,
                    max: 100.0,
                },
            ),
            osc1_coarse: FloatParam::new(
                "Osc1 Coarse",
                0.0,
                FloatRange::Linear {
                    min: -48.0,
                    max: 48.0,
                },
            )
            .with_step_size(1.0),
            osc1_fine: FloatParam::new(
                "Osc1 Fine",
                0.0,
                FloatRange::Linear {
                    min: -100.0,
                    max: 100.0,
                },
            )
            .with_unit("%"),
            osc1_freq_mult: FloatParam::new(
                "Osc1 Freq Mult",
                1.0,
                FloatRange::Linear {
                    min: 1.0,
                    max: 32.0,
                },
            )
            .with_step_size(1.0),
            osc1_freq_div: FloatParam::new(
                "Osc1 Freq Div",
                1.0,
                FloatRange::Linear {
                    min: 1.0,
                    max: 32.0,
                },
            )
            .with_step_size(1.0),
            osc1_attack: FloatParam::new(
                "Osc1 Attack",
                0.0,
                FloatRange::Skewed {
                    min: 0.0,
                    max: 5.0,
                    factor: FloatRange::skew_factor(-2.0),
                },
            )
            .with_unit("s"),
            osc1_decay: FloatParam::new(
                "Osc1 Decay",
                0.0,
                FloatRange::Skewed {
                    min: 0.0,
                    max: 5.0,
                    factor: FloatRange::skew_factor(-2.0),
                },
            )
            .with_unit("s"),
            osc1_sustain: FloatParam::new(
                "Osc1 Sustain",
                1.0,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            ),
            osc1_release: FloatParam::new(
                "Osc1 Release",
                0.05,
                FloatRange::Skewed {
                    min: 0.025,
                    max: 5.0,
                    factor: FloatRange::skew_factor(-2.0),
                },
            )
            .with_unit("s"),
            osc1_feedback: FloatParam::new(
                "Osc1 Feedback",
                0.0,
                FloatRange::Linear {
                    min: -1.0,
                    max: 1.0,
                },
            ),
            osc1_velocity_sensitivity: FloatParam::new(
                "Osc1 Velocity Sens.",
                0.0,
                FloatRange::Linear {
                    min: -1.0,
                    max: 1.0,
                },
            ),
            osc1_keyscaling: FloatParam::new(
                "Osc1 Keyscaling",
                0.0,
                FloatRange::Linear {
                    min: -1.0,
                    max: 1.0,
                },
            ),

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
            osc2_freq_mult: FloatParam::new(
                "Osc2 Freq Mult",
                1.0,
                FloatRange::Linear {
                    min: 1.0,
                    max: 32.0,
                },
            )
            .with_step_size(1.0),
            osc2_freq_div: FloatParam::new(
                "Osc2 Freq Div",
                1.0,
                FloatRange::Linear {
                    min: 1.0,
                    max: 32.0,
                },
            )
            .with_step_size(1.0),
            osc2_attack: FloatParam::new(
                "Osc2 Attack",
                0.0,
                FloatRange::Skewed {
                    min: 0.0,
                    max: 5.0,
                    factor: FloatRange::skew_factor(-2.0),
                },
            )
            .with_unit("s"),
            osc2_decay: FloatParam::new(
                "Osc2 Decay",
                0.0,
                FloatRange::Skewed {
                    min: 0.0,
                    max: 5.0,
                    factor: FloatRange::skew_factor(-2.0),
                },
            )
            .with_unit("s"),
            osc2_sustain: FloatParam::new(
                "Osc2 Sustain",
                1.0,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            ),
            osc2_release: FloatParam::new(
                "Osc2 Release",
                0.05,
                FloatRange::Skewed {
                    min: 0.025,
                    max: 5.0,
                    factor: FloatRange::skew_factor(-2.0),
                },
            )
            .with_unit("s"),
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
            osc3_freq_mult: FloatParam::new(
                "Osc3 Freq Mult",
                1.0,
                FloatRange::Linear {
                    min: 1.0,
                    max: 32.0,
                },
            )
            .with_step_size(1.0),
            osc3_freq_div: FloatParam::new(
                "Osc3 Freq Div",
                1.0,
                FloatRange::Linear {
                    min: 1.0,
                    max: 32.0,
                },
            )
            .with_step_size(1.0),
            osc3_attack: FloatParam::new(
                "Osc3 Attack",
                0.0,
                FloatRange::Skewed {
                    min: 0.0,
                    max: 5.0,
                    factor: FloatRange::skew_factor(-2.0),
                },
            )
            .with_unit("s"),
            osc3_decay: FloatParam::new(
                "Osc3 Decay",
                0.0,
                FloatRange::Skewed {
                    min: 0.0,
                    max: 5.0,
                    factor: FloatRange::skew_factor(-2.0),
                },
            )
            .with_unit("s"),
            osc3_sustain: FloatParam::new(
                "Osc3 Sustain",
                1.0,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            ),
            osc3_release: FloatParam::new(
                "Osc3 Release",
                0.05,
                FloatRange::Skewed {
                    min: 0.025,
                    max: 5.0,
                    factor: FloatRange::skew_factor(-2.0),
                },
            )
            .with_unit("s"),
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
            osc4_freq_mult: FloatParam::new(
                "Osc4 Freq Mult",
                1.0,
                FloatRange::Linear {
                    min: 1.0,
                    max: 34.0,
                },
            )
            .with_step_size(1.0),
            osc4_freq_div: FloatParam::new(
                "Osc4 Freq Div",
                1.0,
                FloatRange::Linear {
                    min: 1.0,
                    max: 32.0,
                },
            )
            .with_step_size(1.0),
            osc4_attack: FloatParam::new(
                "Osc4 Attack",
                0.0,
                FloatRange::Skewed {
                    min: 0.0,
                    max: 5.0,
                    factor: FloatRange::skew_factor(-2.0),
                },
            )
            .with_unit("s"),
            osc4_decay: FloatParam::new(
                "Osc4 Decay",
                0.0,
                FloatRange::Skewed {
                    min: 0.0,
                    max: 5.0,
                    factor: FloatRange::skew_factor(-2.0),
                },
            )
            .with_unit("s"),
            osc4_sustain: FloatParam::new(
                "Osc4 Sustain",
                1.0,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            ),
            osc4_release: FloatParam::new(
                "Osc4 Release",
                0.05,
                FloatRange::Skewed {
                    min: 0.025,
                    max: 5.0,
                    factor: FloatRange::skew_factor(-2.0),
                },
            )
            .with_unit("s"),
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
        }
    }
}

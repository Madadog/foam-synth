use nih_plug::prelude::*;
use std::sync::Arc;
use voice::{OscParams, VoiceList};

// This is a shortened version of the gain example with most comments removed, check out
// https://github.com/robbert-vdh/nih-plug/blob/master/plugins/examples/gain/src/lib.rs to get
// started

mod voice;

struct SynthPlugin {
    params: Arc<SynthPluginParams>,
    sample_rate: f32,

    voices: VoiceList,
}

#[derive(Params)]
struct SynthPluginParams {
    #[id = "gain"]
    pub gain: FloatParam,
    #[id = "octave_multiplier"]
    pub octave_stretch: FloatParam,

    #[id = "mod_osc2_x_osc1"]
    pub mod_osc2_x_osc1: FloatParam,
    #[id = "mod_osc1_x_osc2"]
    pub mod_osc1_x_osc2: FloatParam,

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
}

impl Default for SynthPlugin {
    fn default() -> Self {
        Self {
            params: Arc::new(SynthPluginParams::default()),
            sample_rate: 1.0,
            voices: VoiceList::new(),
        }
    }
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
                }
            ),
            osc1_amp: FloatParam::new(
                "Osc1 Amp",
                100.0,
                FloatRange::Linear {
                    min: 0.0,
                    max: 100.0,
                },
            ),
            mod_osc2_x_osc1: FloatParam::new(
                "Mod Osc2 X Osc1",
                0.0,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            ),
            mod_osc1_x_osc2: FloatParam::new(
                "Mod Osc1 X Osc2",
                0.0,
                FloatRange::Linear { min: 0.0, max: 1.0 },
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
                }
            ),
            osc2_keyscaling: FloatParam::new(
                "Osc2 Keyscaling",
                0.0,
                FloatRange::Linear {
                    min: -1.0,
                    max: 1.0,
                }
            )
        }
    }
}

impl Plugin for SynthPlugin {
    const NAME: &'static str = "Foam";
    const VENDOR: &'static str = "Madadog";
    const URL: &'static str = env!("CARGO_PKG_HOMEPAGE");
    const EMAIL: &'static str = "evilspamalt@gmail.com";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[
        AudioIOLayout {
            // This is also the default and can be omitted here
            main_input_channels: None,
            main_output_channels: NonZeroU32::new(2),
            ..AudioIOLayout::const_default()
        },
        AudioIOLayout {
            main_input_channels: None,
            main_output_channels: NonZeroU32::new(1),
            ..AudioIOLayout::const_default()
        },
    ];

    const MIDI_INPUT: MidiConfig = MidiConfig::Basic;
    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    // If the plugin can send or receive SysEx messages, it can define a type to wrap around those
    // messages here. The type implements the `SysExMessage` trait, which allows conversion to and
    // from plain byte buffers.
    type SysExMessage = ();
    // More advanced plugins can use this to run expensive background tasks. See the field's
    // documentation for more information. `()` means that the plugin does not have any background
    // tasks.
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        // Resize buffers and perform other potentially expensive initialization operations here.
        // The `reset()` function is always called right after this function. You can remove this
        // function if you do not need it.
        self.sample_rate = buffer_config.sample_rate;
        true
    }

    fn reset(&mut self) {}

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        let mut next_event = context.next_event();
        let params = [
            OscParams {
                output_gain: self.params.osc1_amp.value() / 100.0,
                sample_rate: self.sample_rate,
                coarse: self.params.osc1_coarse.value(),
                fine: self.params.osc1_fine.value(),
                frequency_mult: self.params.osc1_freq_mult.value()
                    / self.params.osc1_freq_div.value(),
                attack: self.params.osc1_attack.value(),
                decay: self.params.osc1_decay.value(),
                sustain: self.params.osc1_sustain.value(),
                release: self.params.osc1_release.value(),
                feedback: self.params.osc1_feedback.value().signum()
                    * self.params.osc1_feedback.value().powi(2),
                velocity_sensitivity: self.params.osc1_velocity_sensitivity.value(),
                keyscaling: self.params.osc1_keyscaling.value(),
                octave_stretch: self.params.octave_stretch.value(),
            },
            OscParams {
                output_gain: self.params.osc2_amp.value() / 100.0,
                sample_rate: self.sample_rate,
                coarse: self.params.osc2_coarse.value(),
                fine: self.params.osc2_fine.value(),
                frequency_mult: self.params.osc2_freq_mult.value()
                    / self.params.osc2_freq_div.value(),
                attack: self.params.osc2_attack.value(),
                decay: self.params.osc2_decay.value(),
                sustain: self.params.osc2_sustain.value(),
                release: self.params.osc2_release.value(),
                feedback: self.params.osc2_feedback.value().signum()
                    * self.params.osc2_feedback.value().powi(2),
                velocity_sensitivity: self.params.osc2_velocity_sensitivity.value(),
                keyscaling: self.params.osc2_keyscaling.value(),
                octave_stretch: self.params.octave_stretch.value(),
            },
        ];
        let pm_matrix = [
            self.params.mod_osc2_x_osc1.value() * 3.0,
            self.params.mod_osc1_x_osc2.value() * 3.0,
        ];
        self.voices.update(&params);
        let block_size = buffer.samples();
        for (sample_id, channel_samples) in buffer.iter_samples().enumerate() {
            // Smoothing is optionally built into the parameters themselves
            let gain = self.params.gain.smoothed.next();

            while let Some(event) = next_event {
                if event.timing() != sample_id as u32 {
                    // println!("Event at {sample_id}: {:?}", event);
                    break;
                }

                match event {
                    NoteEvent::NoteOn { note, velocity, .. } => {
                        self.voices.add_voice(note, &params, velocity);
                    }
                    NoteEvent::NoteOff { note, .. } => {
                        // println!("Note off at {}/{sample_id}: {}", event.timing(), note);
                        self.voices.release_voice(note, &params);
                    }
                    _ => (),
                }

                next_event = context.next_event();
            }

            let output = self.voices.play(&params, pm_matrix);

            for sample in channel_samples {
                *sample = output * util::db_to_gain_fast(gain);
            }
        }
        self.voices.remove_voices(&params);
        ProcessStatus::KeepAlive
    }
}

impl ClapPlugin for SynthPlugin {
    const CLAP_ID: &'static str = "mada.dog.foam";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("8-operator FM synth");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;

    const CLAP_FEATURES: &'static [ClapFeature] = &[
        ClapFeature::Instrument,
        ClapFeature::Synthesizer,
        ClapFeature::Stereo,
        ClapFeature::Mono,
        ClapFeature::Utility,
    ];
}

impl Vst3Plugin for SynthPlugin {
    const VST3_CLASS_ID: [u8; 16] = *b"foam.....madadog";

    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[
        Vst3SubCategory::Instrument,
        Vst3SubCategory::Synth,
        Vst3SubCategory::Tools,
    ];
}

nih_export_clap!(SynthPlugin);
nih_export_vst3!(SynthPlugin);

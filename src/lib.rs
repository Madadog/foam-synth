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

    /// The current phase of the sine wave, always kept between in `[0, 1]`.
    phase: f32,

    /// The MIDI note ID of the active note, if triggered by MIDI.
    midi_note_id: u8,
    /// The frequency if the active note, if triggered by MIDI.
    midi_note_freq: f32,
    /// A simple attack and release envelope to avoid clicks. Controlled through velocity and
    /// aftertouch.
    ///
    /// Smoothing is built into the parameters, but you can also use them manually if you need to
    /// smooth soemthing that isn't a parameter.
    midi_note_gain: Smoother<f32>,

    voices: VoiceList,
}

#[derive(Params)]
struct SynthPluginParams {
    #[id = "gain"]
    pub gain: FloatParam,

    #[id = "freq"]
    pub frequency: FloatParam,

    #[id = "usemid"]
    pub use_midi: BoolParam,

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
}

impl Default for SynthPlugin {
    fn default() -> Self {
        Self {
            params: Arc::new(SynthPluginParams::default()),
            sample_rate: 1.0,
            phase: 0.0,
            midi_note_id: 0,
            midi_note_freq: 1.0,
            midi_note_gain: Smoother::new(SmoothingStyle::Linear(5.0)),
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
                util::db_to_gain(0.0),
                FloatRange::Skewed {
                    min: util::db_to_gain(-30.0),
                    max: util::db_to_gain(30.0),
                    // This makes the range appear as if it was linear when displaying the values as
                    // decibels
                    factor: FloatRange::gain_skew_factor(-30.0, 30.0),
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
            frequency: FloatParam::new(
                "Frequency",
                420.0,
                FloatRange::Skewed {
                    min: 1.0,
                    max: 20_000.0,
                    factor: FloatRange::skew_factor(-2.0),
                },
            )
            .with_smoother(SmoothingStyle::Linear(10.0))
            // We purposely don't specify a step size here, but the parameter should still be
            // displayed as if it were rounded. This formatter also includes the unit.
            .with_value_to_string(formatters::v2s_f32_hz_then_khz(0))
            .with_string_to_value(formatters::s2v_f32_hz_then_khz()),
            use_midi: BoolParam::new("Use MIDI", true),
            osc1_amp: FloatParam::new(
                "Osc1 Amp",
                0.0,
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
        }
    }
}

impl SynthPlugin {
    fn calculate_sine(&mut self, frequency: f32) -> f32 {
        let phase_delta = frequency / self.sample_rate;
        let sine = (self.phase * std::f32::consts::TAU).sin();

        self.phase += phase_delta;
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }

        sine
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

    fn reset(&mut self) {
        self.phase = 0.0;
        self.midi_note_id = 0;
        self.midi_note_freq = 1.0;
        self.midi_note_gain.reset(0.0);
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        let mut next_event = context.next_event();
        let params = [
            OscParams {
                amp: self.params.osc1_amp.value() / 100.0,
                sample_rate: self.sample_rate,
                coarse: self.params.osc1_coarse.value(),
                fine: self.params.osc1_fine.value(),
                frequency_mult: self.params.osc1_freq_mult.value()
                    / self.params.osc1_freq_div.value(),
                attack: self.params.osc1_attack.value(),
                decay: self.params.osc1_decay.value(),
                sustain: self.params.osc1_sustain.value(),
                release: self.params.osc1_release.value(),
                feedback: self.params.osc1_feedback.value().signum() * self.params.osc1_feedback.value().powi(2),
            },
            OscParams {
                amp: self.params.osc2_amp.value() / 100.0,
                sample_rate: self.sample_rate,
                coarse: self.params.osc2_coarse.value(),
                fine: self.params.osc2_fine.value(),
                frequency_mult: self.params.osc2_freq_mult.value()
                    / self.params.osc2_freq_div.value(),
                attack: self.params.osc2_attack.value(),
                decay: self.params.osc2_decay.value(),
                sustain: self.params.osc2_sustain.value(),
                release: self.params.osc2_release.value(),
                feedback: self.params.osc2_feedback.value().signum() * self.params.osc2_feedback.value().powi(2),
            },
        ];
        self.voices.update(&params);
        for (sample_id, channel_samples) in buffer.iter_samples().enumerate() {
            // Smoothing is optionally built into the parameters themselves
            let gain = self.params.gain.smoothed.next();

            while let Some(event) = next_event {
                if event.timing() >= sample_id as u32 {
                    break;
                }

                match event {
                    NoteEvent::NoteOn { note, velocity, .. } => {
                        println!("Note on: {}", note);
                        self.midi_note_id = note;
                        self.midi_note_freq = util::midi_note_to_freq(note);
                        self.midi_note_gain.set_target(self.sample_rate, velocity);
                        self.voices.add_voice(note, &params);
                    }
                    NoteEvent::NoteOff { note, .. } => {
                        println!("Note off: {}", note);
                        self.voices.release_voice(note, &params);
                    }
                    NoteEvent::PolyPressure { note, pressure, .. } if note == self.midi_note_id => {
                        self.midi_note_gain.set_target(self.sample_rate, pressure);
                    }
                    _ => (),
                }

                next_event = context.next_event();
            }

            // This plugin can be either triggered by MIDI or controleld by a parameter
            let output = if self.params.use_midi.value() {
                self.voices.play(&params)
            } else {
                let frequency = self.params.frequency.smoothed.next();
                self.calculate_sine(frequency)
            };

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

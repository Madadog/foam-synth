use nih_plug::prelude::*;
use parameters::SynthPluginParams;
use std::sync::Arc;
use voice::{OscParams, VoiceList, VoiceParams};

mod parameters;
mod voice;
mod svf_simper;

struct SynthPlugin {
    params: Arc<SynthPluginParams>,
    sample_rate: f32,

    voices: VoiceList,
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
            OscParams {
                output_gain: self.params.osc3_amp.value() / 100.0,
                sample_rate: self.sample_rate,
                coarse: self.params.osc3_coarse.value(),
                fine: self.params.osc3_fine.value(),
                frequency_mult: self.params.osc3_freq_mult.value()
                    / self.params.osc3_freq_div.value(),
                attack: self.params.osc3_attack.value(),
                decay: self.params.osc3_decay.value(),
                sustain: self.params.osc3_sustain.value(),
                release: self.params.osc3_release.value(),
                feedback: self.params.osc3_feedback.value().signum()
                    * self.params.osc3_feedback.value().powi(2),
                velocity_sensitivity: self.params.osc3_velocity_sensitivity.value(),
                keyscaling: self.params.osc3_keyscaling.value(),
                octave_stretch: self.params.octave_stretch.value(),
            },
            OscParams {
                output_gain: self.params.osc4_amp.value() / 100.0,
                sample_rate: self.sample_rate,
                coarse: self.params.osc4_coarse.value(),
                fine: self.params.osc4_fine.value(),
                frequency_mult: self.params.osc4_freq_mult.value()
                    / self.params.osc4_freq_div.value(),
                attack: self.params.osc4_attack.value(),
                decay: self.params.osc4_decay.value(),
                sustain: self.params.osc4_sustain.value(),
                release: self.params.osc4_release.value(),
                feedback: self.params.osc4_feedback.value().signum()
                    * self.params.osc4_feedback.value().powi(2),
                velocity_sensitivity: self.params.osc4_velocity_sensitivity.value(),
                keyscaling: self.params.osc4_keyscaling.value(),
                octave_stretch: self.params.octave_stretch.value(),
            },
        ];
        let mut pm_matrix = [
            [
                self.params.mod_osc1_by_osc2.value(),
                self.params.mod_osc1_by_osc3.value(),
                self.params.mod_osc1_by_osc4.value(),
            ],
            [
                self.params.mod_osc2_by_osc1.value(),
                self.params.mod_osc2_by_osc3.value(),
                self.params.mod_osc2_by_osc4.value(),
            ],
            [
                self.params.mod_osc3_by_osc1.value(),
                self.params.mod_osc3_by_osc2.value(),
                self.params.mod_osc3_by_osc4.value(),
            ],
            [
                self.params.mod_osc4_by_osc1.value(),
                self.params.mod_osc4_by_osc2.value(),
                self.params.mod_osc4_by_osc3.value(),
            ]
        ];
        pm_matrix.iter_mut().flatten().for_each(|x| *x *= 6.0);
        let voice_params = VoiceParams {
            sample_rate: self.sample_rate,
            filter_enabled: self.params.filter_enabled.value(),
            filter_type: self.params.filter_type.value(),
            filter_cutoff: self.params.filter_cutoff.value(),
            filter_resonance: self.params.filter_resonance.value(),
            filter_envelope_amount: self.params.filter_envelope_amount.value(),
            filter_attack: self.params.filter_envelope_attack.value(),
            filter_decay: self.params.filter_envelope_decay.value(),
            filter_sustain: self.params.filter_envelope_sustain.value(),
            filter_release: self.params.filter_envelope_release.value(),
        };
        self.voices.update(&params, voice_params);
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
                        self.voices.add_voice(note, &params, velocity, voice_params);
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
    const CLAP_DESCRIPTION: Option<&'static str> = Some("4-operator FM synth");
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

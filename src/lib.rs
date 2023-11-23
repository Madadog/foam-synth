use nih_plug::prelude::*;
use parameters::SynthPluginParams;
use std::sync::Arc;
use voice::{OscParams, OscParamsBatch, VoiceList, VoiceParams};
use wide::f32x8;

mod editor;
mod parameters;
mod svf_simper;
mod voice;
mod dsp;

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

    const MIDI_INPUT: MidiConfig = MidiConfig::MidiCCs;
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

    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        editor::create(self.params.clone(), self.params.editor_state.clone())
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
        let block_size = buffer.samples() as u32;
        let osc_params = [
            self.params
                .osc1_params
                .to_osc_params(self.sample_rate, self.params.octave_stretch.value(), block_size),
            self.params
                .osc2_params
                .to_osc_params(self.sample_rate, self.params.octave_stretch.value(), block_size),
            self.params
                .osc3_params
                .to_osc_params(self.sample_rate, self.params.octave_stretch.value(), block_size),
            self.params
                .osc4_params
                .to_osc_params(self.sample_rate, self.params.octave_stretch.value(), block_size),
            self.params
                .osc5_params
                .to_osc_params(self.sample_rate, self.params.octave_stretch.value(), block_size),
            self.params
                .osc6_params
                .to_osc_params(self.sample_rate, self.params.octave_stretch.value(), block_size),
            self.params
                .osc7_params
                .to_osc_params(self.sample_rate, self.params.octave_stretch.value(), block_size),
            self.params
                .osc8_params
                .to_osc_params(self.sample_rate, self.params.octave_stretch.value(), block_size),
        ];

        let mut osc_params = OscParamsBatch::from(osc_params);
        osc_params.coarse += f32x8::splat(self.params.global_coarse.value());
        
        let mut pm_matrix = [
            f32x8::from(self.params.osc1_fm_mod.to_array()),
            f32x8::from(self.params.osc2_fm_mod.to_array()),
            f32x8::from(self.params.osc3_fm_mod.to_array()),
            f32x8::from(self.params.osc4_fm_mod.to_array()),
            f32x8::from(self.params.osc5_fm_mod.to_array()),
            f32x8::from(self.params.osc6_fm_mod.to_array()),
            f32x8::from(self.params.osc7_fm_mod.to_array()),
            f32x8::from(self.params.osc8_fm_mod.to_array()),
        ];
        pm_matrix.iter_mut().for_each(|x| *x = *x * 6.0);
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
            filter_keytrack: self.params.filter_keytrack.value(),
            global_attack: self.params.global_attack.value(),
            global_decay: self.params.global_decay.value(),
            global_sustain: self.params.global_sustain.value(),
            global_release: self.params.global_release.value(),
        };
        self.voices.block_update(&osc_params, voice_params, self.params.bend_range.value());
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
                        self.voices
                        .add_voice(note, &osc_params, velocity, voice_params, self.params.bend_range.value());
                    }
                    NoteEvent::NoteOff { note, .. } => {
                        // println!("Note off at {}/{sample_id}: {}", event.timing(), note);
                        self.voices.release_voice(note, &osc_params, &voice_params);
                    }
                    NoteEvent::MidiPitchBend { timing, channel, value } => { 
                        let value = (value - 0.5) * 2.0;
                        self.voices.pitch_bend = value;
                    }
                    _ => (),
                }
                
                next_event = context.next_event();
            }
            
            self.voices.sample_update(&osc_params, voice_params);
            let output = self.voices.play(&osc_params, &voice_params, pm_matrix);

            for sample in channel_samples {
                *sample = output * gain;
            }
        }
        self.voices.remove_voices(&osc_params, &voice_params);
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
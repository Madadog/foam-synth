use nih_plug::prelude::FloatParam;
use nih_plug::prelude::{util, Editor, GuiContext};
use nih_plug_iced::widgets as nih_widgets;
use nih_plug_iced::IcedState;
use nih_plug_iced::*;
use std::sync::Arc;

use crate::parameters::SynthPluginParams;

use self::param_slider::ParamSlider;

mod param_slider;

pub(crate) fn default_state() -> Arc<IcedState> {
    IcedState::from_size(640, 480)
}

pub(crate) fn create(
    params: Arc<SynthPluginParams>,
    editor_state: Arc<IcedState>,
) -> Option<Box<dyn Editor>> {
    create_iced_editor::<SynthPluginEditor>(editor_state, params)
}

struct SynthPluginEditor {
    params: Arc<SynthPluginParams>,
    context: Arc<dyn GuiContext>,

    gain_slider_state: param_slider::State,
    scrollable: widget::scrollable::State,

    osc1_amp: param_slider::State,
    osc1_coarse: param_slider::State,
    osc1_fine: param_slider::State,
    osc1_freq_mult: param_slider::State,
    osc1_freq_div: param_slider::State,
    osc1_attack: param_slider::State,
    osc1_decay: param_slider::State,
    osc1_sustain: param_slider::State,
    osc1_release: param_slider::State,
    osc1_feedback: param_slider::State,
    osc1_velocity_sensitivity: param_slider::State,
    osc1_keyscaling: param_slider::State,

    osc2_amp: param_slider::State,
    osc2_coarse: param_slider::State,
    osc2_fine: param_slider::State,
    osc2_freq_mult: param_slider::State,
    osc2_freq_div: param_slider::State,
    osc2_attack: param_slider::State,
    osc2_decay: param_slider::State,
    osc2_sustain: param_slider::State,
    osc2_release: param_slider::State,
    osc2_feedback: param_slider::State,
    osc2_velocity_sensitivity: param_slider::State,
    osc2_keyscaling: param_slider::State,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    /// Update a parameter's value.
    ParamUpdate(nih_widgets::ParamMessage),
}

impl IcedEditor for SynthPluginEditor {
    type Executor = executor::Default;
    type Message = Message;
    type InitializationFlags = Arc<SynthPluginParams>;

    fn new(
        params: Self::InitializationFlags,
        context: Arc<dyn GuiContext>,
    ) -> (Self, Command<Self::Message>) {
        let editor = SynthPluginEditor {
            params,
            context,

            gain_slider_state: Default::default(),
            scrollable: Default::default(),

            osc1_amp: Default::default(),
            osc1_coarse: Default::default(),
            osc1_fine: Default::default(),
            osc1_freq_mult: Default::default(),
            osc1_freq_div: Default::default(),
            osc1_attack: Default::default(),
            osc1_decay: Default::default(),
            osc1_sustain: Default::default(),
            osc1_release: Default::default(),
            osc1_feedback: Default::default(),
            osc1_velocity_sensitivity: Default::default(),
            osc1_keyscaling: Default::default(),

            osc2_amp: Default::default(),
            osc2_coarse: Default::default(),
            osc2_fine: Default::default(),
            osc2_freq_mult: Default::default(),
            osc2_freq_div: Default::default(),
            osc2_attack: Default::default(),
            osc2_decay: Default::default(),
            osc2_sustain: Default::default(),
            osc2_release: Default::default(),
            osc2_feedback: Default::default(),
            osc2_velocity_sensitivity: Default::default(),
            osc2_keyscaling: Default::default(),
        };

        (editor, Command::none())
    }

    fn context(&self) -> &dyn GuiContext {
        self.context.as_ref()
    }

    fn update(
        &mut self,
        _window: &mut WindowQueue,
        message: Self::Message,
    ) -> Command<Self::Message> {
        match message {
            Message::ParamUpdate(message) => self.handle_param_message(message),
        }

        Command::none()
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        Scrollable::new(&mut self.scrollable)
            .width(Length::Fill)
            .align_items(Alignment::Center)
            .push(
                Text::new("Foam Synth GUI")
                    .font(assets::NOTO_SANS_LIGHT)
                    .size(24)
                    .width(Length::Fill)
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .vertical_alignment(alignment::Vertical::Center),
            )
            .push(
                Text::new("WARNING: GUI IS INCOMPLETE, DOES NOT EXPOSE ALL CONTROLS. USE DEFAULT VST3/CLAP GUI INSTEAD.")
                    .font(assets::NOTO_SANS_BOLD)
                    .size(12)
                    .width(Length::Fill)
                    .color(Color::from_rgb8(255, 80, 80))
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .vertical_alignment(alignment::Vertical::Center),
            )
            .push(
                Text::new("Output Gain")
                    .height(20.into())
                    .width(Length::Fill)
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .vertical_alignment(alignment::Vertical::Center),
            )
            .push(
                ParamSlider::new(&mut self.gain_slider_state, &self.params.gain)
                    .map(Message::ParamUpdate),
            )
            .push(Space::with_height(10.into()))
            .push(
                // Layout oscillators horizontally
                Row::new()
                    .padding(Padding::from(10))
                    .push(
                        Column::new()
                            .width(Length::Fill)
                            .push(
                                Text::new("Osc 1")
                                    .height(20.into())
                                    .horizontal_alignment(alignment::Horizontal::Center),
                            )
                            .push(
                                Row::new()
                                    .push(
                                        Column::new()
                                            .push(Text::new("Amp."))
                                            .push(
                                                ParamSlider::new(
                                                    &mut self.osc1_amp,
                                                    &self.params.osc1_amp,
                                                )
                                                .width(110.into())
                                                .height(20.into())
                                                .map(Message::ParamUpdate),
                                            )
                                            .push(Text::new("Attack"))
                                            .push(
                                                ParamSlider::new(
                                                    &mut self.osc1_attack,
                                                    &self.params.osc1_attack,
                                                )
                                                .width(110.into())
                                                .height(20.into())
                                                .map(Message::ParamUpdate),
                                            )
                                            .push(Text::new("Decay"))
                                            .push(
                                                ParamSlider::new(
                                                    &mut self.osc1_decay,
                                                    &self.params.osc1_decay,
                                                )
                                                .width(110.into())
                                                .height(20.into())
                                                .map(Message::ParamUpdate),
                                            )
                                            .push(Text::new("Sustain"))
                                            .push(
                                                ParamSlider::new(
                                                    &mut self.osc1_sustain,
                                                    &self.params.osc1_sustain,
                                                )
                                                .width(110.into())
                                                .height(20.into())
                                                .map(Message::ParamUpdate),
                                            )
                                            .push(Text::new("Release"))
                                            .push(
                                                ParamSlider::new(
                                                    &mut self.osc1_release,
                                                    &self.params.osc1_release,
                                                )
                                                .width(110.into())
                                                .height(20.into())
                                                .map(Message::ParamUpdate),
                                            )
                                            .push(Text::new("Velocity Sens."))
                                            .push(
                                                ParamSlider::new(
                                                    &mut self.osc1_velocity_sensitivity,
                                                    &self.params.osc1_velocity_sensitivity,
                                                )
                                                .width(110.into())
                                                .height(20.into())
                                                .map(Message::ParamUpdate),
                                            ),
                                    )
                                    .push(Space::with_width(10.into()))
                                    .push(
                                        Column::new()
                                            .push(Text::new("Coarse Det."))
                                            .push(
                                                ParamSlider::new(
                                                    &mut self.osc1_coarse,
                                                    &self.params.osc1_coarse,
                                                )
                                                .width(110.into())
                                                .height(20.into())
                                                .map(Message::ParamUpdate),
                                            )
                                            .push(Text::new("Fine Detune"))
                                            .push(
                                                ParamSlider::new(
                                                    &mut self.osc1_fine,
                                                    &self.params.osc1_fine,
                                                )
                                                .width(110.into())
                                                .height(20.into())
                                                .map(Message::ParamUpdate),
                                            )
                                            .push(Text::new("Freq. Multiply"))
                                            .push(
                                                ParamSlider::new(
                                                    &mut self.osc1_freq_mult,
                                                    &self.params.osc1_freq_mult,
                                                )
                                                .width(110.into())
                                                .height(20.into())
                                                .map(Message::ParamUpdate),
                                            )
                                            .push(Text::new("Freq. Divide"))
                                            .push(
                                                ParamSlider::new(
                                                    &mut self.osc1_freq_div,
                                                    &self.params.osc1_freq_div,
                                                )
                                                .width(110.into())
                                                .height(20.into())
                                                .map(Message::ParamUpdate),
                                            )
                                            .push(Text::new("Feedback"))
                                            .push(
                                                ParamSlider::new(
                                                    &mut self.osc1_feedback,
                                                    &self.params.osc1_feedback,
                                                )
                                                .width(110.into())
                                                .height(20.into())
                                                .map(Message::ParamUpdate),
                                            )
                                            .push(Text::new("Keyscaling"))
                                            .push(
                                                ParamSlider::new(
                                                    &mut self.osc1_keyscaling,
                                                    &self.params.osc1_keyscaling,
                                                )
                                                .width(110.into())
                                                .height(20.into())
                                                .map(Message::ParamUpdate),
                                            ),
                                    ),
                            ),
                    )
                    .push(Space::with_width(10.into()))
                    .push(
                        Column::new()
                            .push(
                                Text::new("Osc 2")
                                    .height(20.into())
                                    .horizontal_alignment(alignment::Horizontal::Center),
                            )
                            .push(
                                Row::new()
                                    .push(
                                        Column::new()
                                            .push(Text::new("Amp."))
                                            .push(
                                                ParamSlider::new(
                                                    &mut self.osc2_amp,
                                                    &self.params.osc2_amp,
                                                )
                                                .width(110.into())
                                                .height(20.into())
                                                .map(Message::ParamUpdate),
                                            )
                                            .push(Text::new("Attack"))
                                            .push(
                                                ParamSlider::new(
                                                    &mut self.osc2_attack,
                                                    &self.params.osc2_attack,
                                                )
                                                .width(110.into())
                                                .height(20.into())
                                                .map(Message::ParamUpdate),
                                            )
                                            .push(Text::new("Decay"))
                                            .push(
                                                ParamSlider::new(
                                                    &mut self.osc2_decay,
                                                    &self.params.osc2_decay,
                                                )
                                                .width(110.into())
                                                .height(20.into())
                                                .map(Message::ParamUpdate),
                                            )
                                            .push(Text::new("Sustain"))
                                            .push(
                                                ParamSlider::new(
                                                    &mut self.osc2_sustain,
                                                    &self.params.osc2_sustain,
                                                )
                                                .width(110.into())
                                                .height(20.into())
                                                .map(Message::ParamUpdate),
                                            )
                                            .push(Text::new("Release"))
                                            .push(
                                                ParamSlider::new(
                                                    &mut self.osc2_release,
                                                    &self.params.osc2_release,
                                                )
                                                .width(110.into())
                                                .height(20.into())
                                                .map(Message::ParamUpdate),
                                            )
                                            .push(Text::new("Velocity Sens."))
                                            .push(
                                                ParamSlider::new(
                                                    &mut self.osc2_velocity_sensitivity,
                                                    &self.params.osc2_velocity_sensitivity,
                                                )
                                                .width(110.into())
                                                .height(20.into())
                                                .map(Message::ParamUpdate),
                                            ),
                                    )
                                    .push(Space::with_width(10.into()))
                                    .push(
                                        Column::new()
                                            .push(Text::new("Coarse Det."))
                                            .push(
                                                ParamSlider::new(
                                                    &mut self.osc2_coarse,
                                                    &self.params.osc2_coarse,
                                                )
                                                .width(110.into())
                                                .height(20.into())
                                                .map(Message::ParamUpdate),
                                            )
                                            .push(Text::new("Fine Detune"))
                                            .push(
                                                ParamSlider::new(
                                                    &mut self.osc2_fine,
                                                    &self.params.osc2_fine,
                                                )
                                                .width(110.into())
                                                .height(20.into())
                                                .map(Message::ParamUpdate),
                                            )
                                            .push(Text::new("Freq. Multiply"))
                                            .push(
                                                ParamSlider::new(
                                                    &mut self.osc2_freq_mult,
                                                    &self.params.osc2_freq_mult,
                                                )
                                                .width(110.into())
                                                .height(20.into())
                                                .map(Message::ParamUpdate),
                                            )
                                            .push(Text::new("Freq. Divide"))
                                            .push(
                                                ParamSlider::new(
                                                    &mut self.osc2_freq_div,
                                                    &self.params.osc2_freq_div,
                                                )
                                                .width(110.into())
                                                .height(20.into())
                                                .map(Message::ParamUpdate),
                                            )
                                            .push(Text::new("Feedback"))
                                            .push(
                                                ParamSlider::new(
                                                    &mut self.osc2_feedback,
                                                    &self.params.osc2_feedback,
                                                )
                                                .width(110.into())
                                                .height(20.into())
                                                .map(Message::ParamUpdate),
                                            )
                                            .push(Text::new("Keyscaling"))
                                            .push(
                                                ParamSlider::new(
                                                    &mut self.osc2_keyscaling,
                                                    &self.params.osc2_keyscaling,
                                                )
                                                .width(110.into())
                                                .height(20.into())
                                                .map(Message::ParamUpdate),
                                            ),
                                    ),
                            ),
                    ),
            )
            // .push(
            //     nih_widgets::PeakMeter::new(
            //         &mut self.peak_meter_state,
            //         util::gain_to_db(self.peak_meter.load(std::sync::atomic::Ordering::Relaxed)),
            //     )
            //     .hold_time(Duration::from_millis(600)),
            // )
            .into()
    }

    fn background_color(&self) -> nih_plug_iced::Color {
        nih_plug_iced::Color {
            r: 0.98,
            g: 0.98,
            b: 0.98,
            a: 1.0,
        }
    }
}

struct OscillatorWidget {
    pub amp: param_slider::State,
    pub coarse: param_slider::State,
    pub fine: param_slider::State,
    pub freq_mult: param_slider::State,
    pub freq_div: param_slider::State,
    pub attack: param_slider::State,
    pub decay: param_slider::State,
    pub sustain: param_slider::State,
    pub release: param_slider::State,
    pub feedback: param_slider::State,
    pub velocity_sensitivity: param_slider::State,
    pub keyscaling: param_slider::State,
}

impl OscillatorWidget {
    fn new() -> Self {
        Self {
            amp: Default::default(),
            coarse: Default::default(),
            fine: Default::default(),
            freq_mult: Default::default(),
            freq_div: Default::default(),
            attack: Default::default(),
            decay: Default::default(),
            sustain: Default::default(),
            release: Default::default(),
            feedback: Default::default(),
            velocity_sensitivity: Default::default(),
            keyscaling: Default::default(),
        }
    }
    fn row<'a>(
        &'a mut self,
        amp: &'a FloatParam,
        coarse: &'a FloatParam,
        fine: &'a FloatParam,
        freq_mult: &'a FloatParam,
        freq_div: &'a FloatParam,
        attack: &'a FloatParam,
        decay: &'a FloatParam,
        sustain: &'a FloatParam,
        release: &'a FloatParam,
        feedback: &'a FloatParam,
        velocity_sensitivity: &'a FloatParam,
        keyscaling: &'a FloatParam,
    ) -> Row<Message> {
        Row::new().padding(Padding::from(10)).push(
            Column::new()
                .width(Length::Fill)
                .push(
                    Text::new("Osc 1")
                        .height(20.into())
                        .horizontal_alignment(alignment::Horizontal::Center),
                )
                .push(
                    Row::new()
                        .push(
                            Column::new()
                                .push(Text::new("Amp."))
                                .push(
                                    ParamSlider::new(&mut self.amp, amp)
                                        .width(110.into())
                                        .height(20.into())
                                        .map(Message::ParamUpdate),
                                )
                                .push(Text::new("Attack"))
                                .push(
                                    ParamSlider::new(&mut self.attack, attack)
                                        .width(110.into())
                                        .height(20.into())
                                        .map(Message::ParamUpdate),
                                )
                                .push(Text::new("Decay"))
                                .push(
                                    ParamSlider::new(&mut self.decay, decay)
                                        .width(110.into())
                                        .height(20.into())
                                        .map(Message::ParamUpdate),
                                )
                                .push(Text::new("Sustain"))
                                .push(
                                    ParamSlider::new(&mut self.sustain, sustain)
                                        .width(110.into())
                                        .height(20.into())
                                        .map(Message::ParamUpdate),
                                )
                                .push(Text::new("Release"))
                                .push(
                                    ParamSlider::new(&mut self.release, release)
                                        .width(110.into())
                                        .height(20.into())
                                        .map(Message::ParamUpdate),
                                )
                                .push(Text::new("Velocity Sens."))
                                .push(
                                    ParamSlider::new(
                                        &mut self.velocity_sensitivity,
                                        velocity_sensitivity,
                                    )
                                    .width(110.into())
                                    .height(20.into())
                                    .map(Message::ParamUpdate),
                                ),
                        )
                        .push(Space::with_width(10.into()))
                        .push(
                            Column::new()
                                .push(Text::new("Coarse Det."))
                                .push(
                                    ParamSlider::new(&mut self.coarse, coarse)
                                        .width(110.into())
                                        .height(20.into())
                                        .map(Message::ParamUpdate),
                                )
                                .push(Text::new("Fine Detune"))
                                .push(
                                    ParamSlider::new(&mut self.fine, fine)
                                        .width(110.into())
                                        .height(20.into())
                                        .map(Message::ParamUpdate),
                                )
                                .push(Text::new("Freq. Multiply"))
                                .push(
                                    ParamSlider::new(&mut self.freq_mult, freq_mult)
                                        .width(110.into())
                                        .height(20.into())
                                        .map(Message::ParamUpdate),
                                )
                                .push(Text::new("Freq. Divide"))
                                .push(
                                    ParamSlider::new(&mut self.freq_div, freq_div)
                                        .width(110.into())
                                        .height(20.into())
                                        .map(Message::ParamUpdate),
                                )
                                .push(Text::new("Feedback"))
                                .push(
                                    ParamSlider::new(&mut self.feedback, feedback)
                                        .width(110.into())
                                        .height(20.into())
                                        .map(Message::ParamUpdate),
                                )
                                .push(Text::new("Keyscaling"))
                                .push(
                                    ParamSlider::new(&mut self.keyscaling, keyscaling)
                                        .width(110.into())
                                        .height(20.into())
                                        .map(Message::ParamUpdate),
                                ),
                        ),
                ),
        )
    }
}

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
    IcedState::from_size(900, 580)
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

    osc_params_1: OscillatorWidget,
    osc_params_2: OscillatorWidget,
    osc_params_3: OscillatorWidget,
    osc_params_4: OscillatorWidget,
    osc_params_5: OscillatorWidget,
    osc_params_6: OscillatorWidget,
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

            osc_params_1: OscillatorWidget::new("Osc 1"),
            osc_params_2: OscillatorWidget::new("Osc 2"),
            osc_params_3: OscillatorWidget::new("Osc 3"),
            osc_params_4: OscillatorWidget::new("Osc 4"),
            osc_params_5: OscillatorWidget::new("Osc 5"),
            osc_params_6: OscillatorWidget::new("Osc 6"),
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
                Row::new()
                .padding(Padding::from(10))
                .spacing(20)
                .push(
                    Column::new()
                    .align_items(Alignment::Fill)
                    .push(
                        Text::new("Foam Synth GUI")
                            .font(assets::NOTO_SANS_LIGHT)
                            .size(24)
                            .horizontal_alignment(alignment::Horizontal::Center)
                            .vertical_alignment(alignment::Vertical::Center),
                    )
                    .push(
                        Text::new("WARNING: GUI IS INCOMPLETE, DOES NOT EXPOSE ALL CONTROLS. CHECK THE DEFAULT VST3/CLAP GUI.")
                            .font(assets::NOTO_SANS_BOLD)
                            .size(12)
                            .color(Color::from_rgb8(255, 80, 80))
                            .horizontal_alignment(alignment::Horizontal::Center)
                            .vertical_alignment(alignment::Vertical::Center),
                    )
                )
                .push(
                    Column::new()
                    .align_items(Alignment::Fill)
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
                )
            )
            .push(
                // Layout oscillators horizontally
                Row::new()
                    .padding(Padding::from(5))
                    .spacing(20)
                    .push(
                        self.osc_params_1.content(
                            &self.params.osc1_amp,
                            &self.params.osc1_coarse,
                            &self.params.osc1_fine,
                            &self.params.osc1_freq_mult,
                            &self.params.osc1_freq_div,
                            &self.params.osc1_attack,
                            &self.params.osc1_decay,
                            &self.params.osc1_sustain,
                            &self.params.osc1_release,
                            &self.params.osc1_feedback,
                            &self.params.osc1_velocity_sensitivity,
                            &self.params.osc1_keyscaling
                        )
                    )
                    .push(
                        self.osc_params_2.content(
                            &self.params.osc2_amp,
                            &self.params.osc2_coarse,
                            &self.params.osc2_fine,
                            &self.params.osc2_freq_mult,
                            &self.params.osc2_freq_div,
                            &self.params.osc2_attack,
                            &self.params.osc2_decay,
                            &self.params.osc2_sustain,
                            &self.params.osc2_release,
                            &self.params.osc2_feedback,
                            &self.params.osc2_velocity_sensitivity,
                            &self.params.osc2_keyscaling
                        )
                    )
                    .push(
                        self.osc_params_3.content(
                            &self.params.osc3_amp,
                            &self.params.osc3_coarse,
                            &self.params.osc3_fine,
                            &self.params.osc3_freq_mult,
                            &self.params.osc3_freq_div,
                            &self.params.osc3_attack,
                            &self.params.osc3_decay,
                            &self.params.osc3_sustain,
                            &self.params.osc3_release,
                            &self.params.osc3_feedback,
                            &self.params.osc3_velocity_sensitivity,
                            &self.params.osc3_keyscaling
                        )
                    )
                )
            .push(
                Row::new()
                    .padding(Padding::from(5))
                    .spacing(20)
                    .push(
                        self.osc_params_4.content(
                            &self.params.osc4_amp,
                            &self.params.osc4_coarse,
                            &self.params.osc4_fine,
                            &self.params.osc4_freq_mult,
                            &self.params.osc4_freq_div,
                            &self.params.osc4_attack,
                            &self.params.osc4_decay,
                            &self.params.osc4_sustain,
                            &self.params.osc4_release,
                            &self.params.osc4_feedback,
                            &self.params.osc4_velocity_sensitivity,
                            &self.params.osc4_keyscaling
                        )
                    )
                    .push(
                        self.osc_params_5.content(
                            &self.params.osc5_amp,
                            &self.params.osc5_coarse,
                            &self.params.osc5_fine,
                            &self.params.osc5_freq_mult,
                            &self.params.osc5_freq_div,
                            &self.params.osc5_attack,
                            &self.params.osc5_decay,
                            &self.params.osc5_sustain,
                            &self.params.osc5_release,
                            &self.params.osc5_feedback,
                            &self.params.osc5_velocity_sensitivity,
                            &self.params.osc5_keyscaling
                        )
                    )
                    .push(
                        self.osc_params_6.content(
                            &self.params.osc6_amp,
                            &self.params.osc6_coarse,
                            &self.params.osc6_fine,
                            &self.params.osc6_freq_mult,
                            &self.params.osc6_freq_div,
                            &self.params.osc6_attack,
                            &self.params.osc6_decay,
                            &self.params.osc6_sustain,
                            &self.params.osc6_release,
                            &self.params.osc6_feedback,
                            &self.params.osc6_velocity_sensitivity,
                            &self.params.osc6_keyscaling
                        )
                    )
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
    pub name: &'static str,
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
    fn new(name: &'static str) -> Self {
        Self {
            name,
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
    fn content<'a>(
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
    ) -> Column<Message> {
        let param_font_size = 16;
        let slider_font_size = 18;
        let slider_width = 80;
        let slider_height = 20;
        Column::new()
            .push(
                Text::new(self.name)
                    .height(18.into())
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .font(assets::NOTO_SANS_BOLD),
            )
            .push(
                Row::new()
                    .push(
                        Column::new()
                            .push(Text::new("Amplitude").size(param_font_size))
                            .push(
                                ParamSlider::new(&mut self.amp, amp)
                                    .width(slider_width.into())
                                    .height(slider_height.into())
                                    .text_size(slider_font_size)
                                    .map(Message::ParamUpdate),
                            )
                            .push(Text::new("Attack").size(param_font_size))
                            .push(
                                ParamSlider::new(&mut self.attack, attack)
                                    .width(slider_width.into())
                                    .height(slider_height.into())
                                    .text_size(slider_font_size)
                                    .map(Message::ParamUpdate),
                            )
                            .push(Text::new("Decay").size(param_font_size))
                            .push(
                                ParamSlider::new(&mut self.decay, decay)
                                    .width(slider_width.into())
                                    .height(slider_height.into())
                                    .text_size(slider_font_size)
                                    .map(Message::ParamUpdate),
                            )
                            .push(Text::new("Sustain").size(param_font_size))
                            .push(
                                ParamSlider::new(&mut self.sustain, sustain)
                                    .width(slider_width.into())
                                    .height(slider_height.into())
                                    .text_size(slider_font_size)
                                    .map(Message::ParamUpdate),
                            )
                            .push(Text::new("Release").size(param_font_size))
                            .push(
                                ParamSlider::new(&mut self.release, release)
                                    .width(slider_width.into())
                                    .height(slider_height.into())
                                    .text_size(slider_font_size)
                                    .map(Message::ParamUpdate),
                            )
                            .push(Text::new("Velocity Sens.").size(param_font_size))
                            .push(
                                ParamSlider::new(
                                    &mut self.velocity_sensitivity,
                                    velocity_sensitivity,
                                )
                                .width(slider_width.into())
                                .height(slider_height.into())
                                .map(Message::ParamUpdate),
                            ),
                    )
                    .push(Space::with_width(8.into()))
                    .push(
                        Column::new()
                            .push(Text::new("Feedback").size(param_font_size))
                            .push(
                                ParamSlider::new(&mut self.feedback, feedback)
                                    .width(slider_width.into())
                                    .height(slider_height.into())
                                    .text_size(slider_font_size)
                                    .map(Message::ParamUpdate),
                            )
                            .push(Text::new("Coarse Det.").size(param_font_size))
                            .push(
                                ParamSlider::new(&mut self.coarse, coarse)
                                    .width(slider_width.into())
                                    .height(slider_height.into())
                                    .text_size(slider_font_size)
                                    .map(Message::ParamUpdate),
                            )
                            .push(Text::new("Fine Detune").size(param_font_size))
                            .push(
                                ParamSlider::new(&mut self.fine, fine)
                                    .width(slider_width.into())
                                    .height(slider_height.into())
                                    .text_size(slider_font_size)
                                    .map(Message::ParamUpdate),
                            )
                            .push(Text::new("Freq. Multiply").size(param_font_size))
                            .push(
                                ParamSlider::new(&mut self.freq_mult, freq_mult)
                                    .width(slider_width.into())
                                    .height(slider_height.into())
                                    .text_size(slider_font_size)
                                    .map(Message::ParamUpdate),
                            )
                            .push(Text::new("Freq. Divide").size(param_font_size))
                            .push(
                                ParamSlider::new(&mut self.freq_div, freq_div)
                                    .width(slider_width.into())
                                    .height(slider_height.into())
                                    .text_size(slider_font_size)
                                    .map(Message::ParamUpdate),
                            )
                            .push(Text::new("Keyscaling").size(param_font_size))
                            .push(
                                ParamSlider::new(&mut self.keyscaling, keyscaling)
                                    .width(slider_width.into())
                                    .height(slider_height.into())
                                    .text_size(slider_font_size)
                                    .map(Message::ParamUpdate),
                            ),
                    ),
            )
    }
}

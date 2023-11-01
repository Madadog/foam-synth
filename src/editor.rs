use nih_plug::prelude::FloatParam;
use nih_plug::prelude::{util, Editor, GuiContext};
use nih_plug_iced::widget::image;
use nih_plug_iced::widgets as nih_widgets;
use nih_plug_iced::IcedState;
use nih_plug_iced::*;
use std::sync::Arc;

use crate::parameters::SynthPluginParams;

use self::param_slider::ParamSlider;

mod envelope;
mod param_slider;

pub(crate) fn default_state() -> Arc<IcedState> {
    IcedState::from_size(1000, 700)
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
    octave_stretch_slider_state: param_slider::State,
    scrollable: widget::scrollable::State,

    filter_enabled_slider_state: param_slider::State,
    filter_type_slider_state: param_slider::State,
    filter_cutoff_slider_state: param_slider::State,
    filter_resonance_slider_state: param_slider::State,
    filter_keytrack_slider_state: param_slider::State,
    filter_envelope_amount_slider_state: param_slider::State,
    filter_envelope_attack_slider_state: param_slider::State,
    filter_envelope_decay_slider_state: param_slider::State,
    filter_envelope_sustain_slider_state: param_slider::State,
    filter_envelope_release_slider_state: param_slider::State,

    osc_params_1: OscillatorWidget,
    osc_params_2: OscillatorWidget,
    osc_params_3: OscillatorWidget,
    osc_params_4: OscillatorWidget,
    osc_params_5: OscillatorWidget,
    osc_params_6: OscillatorWidget,
    osc_params_7: OscillatorWidget,
    osc_params_8: OscillatorWidget,

    matrix: MatrixWidget,
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
            octave_stretch_slider_state: Default::default(),
            scrollable: Default::default(),

            filter_enabled_slider_state: Default::default(),
            filter_type_slider_state: Default::default(),
            filter_cutoff_slider_state: Default::default(),
            filter_resonance_slider_state: Default::default(),
            filter_keytrack_slider_state: Default::default(),
            filter_envelope_amount_slider_state: Default::default(),
            filter_envelope_attack_slider_state: Default::default(),
            filter_envelope_decay_slider_state: Default::default(),
            filter_envelope_sustain_slider_state: Default::default(),
            filter_envelope_release_slider_state: Default::default(),

            osc_params_1: OscillatorWidget::new("Osc 1"),
            osc_params_2: OscillatorWidget::new("Osc 2"),
            osc_params_3: OscillatorWidget::new("Osc 3"),
            osc_params_4: OscillatorWidget::new("Osc 4"),
            osc_params_5: OscillatorWidget::new("Osc 5"),
            osc_params_6: OscillatorWidget::new("Osc 6"),
            osc_params_7: OscillatorWidget::new("Osc 7"),
            osc_params_8: OscillatorWidget::new("Osc 8"),

            matrix: Default::default(),
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
            // .push(Image::new(image::Handle::from_pixels(2, 2, vec![0, 0, 0, 255])))
            .push(
                Row::new()
                    .padding(Padding::from(10))
                    .spacing(20)
                    .push(title_bar())
                    .push(
                        Column::new()
                            .align_items(Alignment::Fill)
                            .push(
                                Text::new("Output Gain")
                                    .size(16)
                                    .width(Length::Fill)
                                    .horizontal_alignment(alignment::Horizontal::Center)
                                    .vertical_alignment(alignment::Vertical::Center),
                            )
                            .push(
                                ParamSlider::new(&mut self.gain_slider_state, &self.params.gain)
                                    .height(20.into())
                                    .width(100.into())
                                    .map(Message::ParamUpdate),
                                )
                            .push(
                                Text::new("Octave Stretch")
                                .size(16)
                                    .width(Length::Fill)
                                    .horizontal_alignment(alignment::Horizontal::Center)
                                    .vertical_alignment(alignment::Vertical::Center),
                                )
                            .push(
                                ParamSlider::new(
                                    &mut self.octave_stretch_slider_state,
                                    &self.params.octave_stretch,
                                )
                                .height(20.into())
                                .width(100.into())
                                .map(Message::ParamUpdate),
                            ),
                    )
                    .push(self.matrix.ui_matrix(&self.params))
                    .push(
                        Column::new()
                            .padding(Padding::new(10))
                            // .push(Space::with_height(20.into()))
                            .push(
                                Text::new("Filter")
                                    .height(18.into())
                                    .horizontal_alignment(alignment::Horizontal::Center)
                                    .font(assets::NOTO_SANS_BOLD),
                            )
                            .push(
                                Row::new()
                                    .push(
                                        Column::new()
                                            .max_width(110)
                                            .push(
                                                Text::new("Filter Enabled")
                                                    .size(16)
                                                    .width(Length::Fill)
                                                    .vertical_alignment(
                                                        alignment::Vertical::Center,
                                                    ),
                                            )
                                            .push(
                                                ParamSlider::new(
                                                    &mut self.filter_enabled_slider_state,
                                                    &self.params.filter_enabled,
                                                )
                                                .height(20.into())
                                                .width(80.into())
                                                .map(Message::ParamUpdate),
                                            )
                                            .push(
                                                Text::new("Filter Type")
                                                    .size(16)
                                                    .width(Length::Fill)
                                                    .vertical_alignment(
                                                        alignment::Vertical::Center,
                                                    ),
                                            )
                                            .push(
                                                ParamSlider::new(
                                                    &mut self.filter_type_slider_state,
                                                    &self.params.filter_type,
                                                )
                                                .height(20.into())
                                                .width(80.into())
                                                .map(Message::ParamUpdate),
                                            )
                                            .push(
                                                Text::new("Filter Cutoff")
                                                    .size(16)
                                                    .width(Length::Fill)
                                                    .vertical_alignment(
                                                        alignment::Vertical::Center,
                                                    ),
                                            )
                                            .push(
                                                ParamSlider::new(
                                                    &mut self.filter_cutoff_slider_state,
                                                    &self.params.filter_cutoff,
                                                )
                                                .height(20.into())
                                                .width(80.into())
                                                .map(Message::ParamUpdate),
                                            )
                                            .push(
                                                Text::new("Filter Resonance")
                                                    .size(16)
                                                    .width(Length::Fill)
                                                    .vertical_alignment(
                                                        alignment::Vertical::Center,
                                                    ),
                                            )
                                            .push(
                                                ParamSlider::new(
                                                    &mut self.filter_resonance_slider_state,
                                                    &self.params.filter_resonance,
                                                )
                                                .height(20.into())
                                                .width(80.into())
                                                .map(Message::ParamUpdate),
                                            )
                                            .push(
                                                Text::new("Filter Keytrack")
                                                    .size(16)
                                                    .width(Length::Fill)
                                                    .vertical_alignment(
                                                        alignment::Vertical::Center,
                                                    ),
                                            )
                                            .push(
                                                ParamSlider::new(
                                                    &mut self.filter_keytrack_slider_state,
                                                    &self.params.filter_keytrack,
                                                )
                                                .height(20.into())
                                                .width(80.into())
                                                .map(Message::ParamUpdate),
                                            ),
                                    )
                                    .push(
                                        Column::new()
                                            .push(
                                                Text::new("Filter Envelope Amt.")
                                                    .size(16)
                                                    .width(Length::Fill)
                                                    .vertical_alignment(
                                                        alignment::Vertical::Center,
                                                    ),
                                            )
                                            .push(
                                                ParamSlider::new(
                                                    &mut self.filter_envelope_amount_slider_state,
                                                    &self.params.filter_envelope_amount,
                                                )
                                                .height(20.into())
                                                .width(80.into())
                                                .map(Message::ParamUpdate),
                                            )
                                            .push(
                                                Text::new("Filter Env. Attack")
                                                    .size(16)
                                                    .width(Length::Fill)
                                                    .vertical_alignment(
                                                        alignment::Vertical::Center,
                                                    ),
                                            )
                                            .push(
                                                ParamSlider::new(
                                                    &mut self.filter_envelope_attack_slider_state,
                                                    &self.params.filter_envelope_attack,
                                                )
                                                .height(20.into())
                                                .width(80.into())
                                                .map(Message::ParamUpdate),
                                            )
                                            .push(
                                                Text::new("Filter Env. Decay")
                                                    .size(16)
                                                    .width(Length::Fill)
                                                    .vertical_alignment(
                                                        alignment::Vertical::Center,
                                                    ),
                                            )
                                            .push(
                                                ParamSlider::new(
                                                    &mut self.filter_envelope_decay_slider_state,
                                                    &self.params.filter_envelope_decay,
                                                )
                                                .height(20.into())
                                                .width(80.into())
                                                .map(Message::ParamUpdate),
                                            )
                                            .push(
                                                Text::new("Filter Env. Sustain")
                                                    .size(16)
                                                    .width(Length::Fill)
                                                    .vertical_alignment(
                                                        alignment::Vertical::Center,
                                                    ),
                                            )
                                            .push(
                                                ParamSlider::new(
                                                    &mut self.filter_envelope_sustain_slider_state,
                                                    &self.params.filter_envelope_sustain,
                                                )
                                                .height(20.into())
                                                .width(80.into())
                                                .map(Message::ParamUpdate),
                                            )
                                            .push(
                                                Text::new("Filter Env. Release")
                                                    .size(16)
                                                    .width(Length::Fill)
                                                    .vertical_alignment(
                                                        alignment::Vertical::Center,
                                                    ),
                                            )
                                            .push(
                                                ParamSlider::new(
                                                    &mut self.filter_envelope_release_slider_state,
                                                    &self.params.filter_envelope_release,
                                                )
                                                .height(20.into())
                                                .width(80.into())
                                                .map(Message::ParamUpdate),
                                            ),
                                    ),
                            ),
                    ),
            )
            .push(
                Row::new().push(
                    Column::new()
                        .push(
                            // Layout oscillators horizontally
                            Row::new()
                                .padding(Padding::from(5))
                                .spacing(20)
                                .push(self.osc_params_1.content(
                                    &self.params.osc1_params.amp,
                                    &self.params.osc1_params.coarse,
                                    &self.params.osc1_params.fine,
                                    &self.params.osc1_params.freq_mult,
                                    &self.params.osc1_params.freq_div,
                                    &self.params.osc1_params.attack,
                                    &self.params.osc1_params.decay,
                                    &self.params.osc1_params.sustain,
                                    &self.params.osc1_params.release,
                                    &self.params.osc1_params.feedback,
                                    &self.params.osc1_params.velocity_sensitivity,
                                    &self.params.osc1_params.keyscaling,
                                ))
                                .push(self.osc_params_2.content(
                                    &self.params.osc2_params.amp,
                                    &self.params.osc2_params.coarse,
                                    &self.params.osc2_params.fine,
                                    &self.params.osc2_params.freq_mult,
                                    &self.params.osc2_params.freq_div,
                                    &self.params.osc2_params.attack,
                                    &self.params.osc2_params.decay,
                                    &self.params.osc2_params.sustain,
                                    &self.params.osc2_params.release,
                                    &self.params.osc2_params.feedback,
                                    &self.params.osc2_params.velocity_sensitivity,
                                    &self.params.osc2_params.keyscaling,
                                ))
                                .push(self.osc_params_3.content(
                                    &self.params.osc3_params.amp,
                                    &self.params.osc3_params.coarse,
                                    &self.params.osc3_params.fine,
                                    &self.params.osc3_params.freq_mult,
                                    &self.params.osc3_params.freq_div,
                                    &self.params.osc3_params.attack,
                                    &self.params.osc3_params.decay,
                                    &self.params.osc3_params.sustain,
                                    &self.params.osc3_params.release,
                                    &self.params.osc3_params.feedback,
                                    &self.params.osc3_params.velocity_sensitivity,
                                    &self.params.osc3_params.keyscaling,
                                ))
                                .push(self.osc_params_4.content(
                                    &self.params.osc4_params.amp,
                                    &self.params.osc4_params.coarse,
                                    &self.params.osc4_params.fine,
                                    &self.params.osc4_params.freq_mult,
                                    &self.params.osc4_params.freq_div,
                                    &self.params.osc4_params.attack,
                                    &self.params.osc4_params.decay,
                                    &self.params.osc4_params.sustain,
                                    &self.params.osc4_params.release,
                                    &self.params.osc4_params.feedback,
                                    &self.params.osc4_params.velocity_sensitivity,
                                    &self.params.osc4_params.keyscaling,
                                )),
                        )
                        .push(
                            Row::new()
                                .padding(Padding::from(5))
                                .spacing(20)
                                .push(self.osc_params_5.content(
                                    &self.params.osc5_params.amp,
                                    &self.params.osc5_params.coarse,
                                    &self.params.osc5_params.fine,
                                    &self.params.osc5_params.freq_mult,
                                    &self.params.osc5_params.freq_div,
                                    &self.params.osc5_params.attack,
                                    &self.params.osc5_params.decay,
                                    &self.params.osc5_params.sustain,
                                    &self.params.osc5_params.release,
                                    &self.params.osc5_params.feedback,
                                    &self.params.osc5_params.velocity_sensitivity,
                                    &self.params.osc5_params.keyscaling,
                                ))
                                .push(self.osc_params_6.content(
                                    &self.params.osc6_params.amp,
                                    &self.params.osc6_params.coarse,
                                    &self.params.osc6_params.fine,
                                    &self.params.osc6_params.freq_mult,
                                    &self.params.osc6_params.freq_div,
                                    &self.params.osc6_params.attack,
                                    &self.params.osc6_params.decay,
                                    &self.params.osc6_params.sustain,
                                    &self.params.osc6_params.release,
                                    &self.params.osc6_params.feedback,
                                    &self.params.osc6_params.velocity_sensitivity,
                                    &self.params.osc6_params.keyscaling,
                                ))
                                .push(self.osc_params_7.content(
                                    &self.params.osc7_params.amp,
                                    &self.params.osc7_params.coarse,
                                    &self.params.osc7_params.fine,
                                    &self.params.osc7_params.freq_mult,
                                    &self.params.osc7_params.freq_div,
                                    &self.params.osc7_params.attack,
                                    &self.params.osc7_params.decay,
                                    &self.params.osc7_params.sustain,
                                    &self.params.osc7_params.release,
                                    &self.params.osc7_params.feedback,
                                    &self.params.osc7_params.velocity_sensitivity,
                                    &self.params.osc7_params.keyscaling,
                                ))
                                .push(self.osc_params_8.content(
                                    &self.params.osc8_params.amp,
                                    &self.params.osc8_params.coarse,
                                    &self.params.osc8_params.fine,
                                    &self.params.osc8_params.freq_mult,
                                    &self.params.osc8_params.freq_div,
                                    &self.params.osc8_params.attack,
                                    &self.params.osc8_params.decay,
                                    &self.params.osc8_params.sustain,
                                    &self.params.osc8_params.release,
                                    &self.params.osc8_params.feedback,
                                    &self.params.osc8_params.velocity_sensitivity,
                                    &self.params.osc8_params.keyscaling,
                                )),
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
            // .push(Row::new().padding(5)
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
        let slider_font_size = 16;
        let slider_width = 80;
        let slider_height = 16;
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

fn title_bar<'a>() -> Column<'a, Message> {
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
            Text::new("WORK IN PROGRESS GUI.")
                .font(assets::NOTO_SANS_BOLD)
                .size(12)
                .color(Color::from_rgb8(255, 80, 80))
                .horizontal_alignment(alignment::Horizontal::Center)
                .vertical_alignment(alignment::Vertical::Center),
        )
}

#[derive(Debug, Default)]
struct MatrixWidget {
    _1_2: param_slider::State,
    _1_3: param_slider::State,
    _1_4: param_slider::State,
    _1_5: param_slider::State,
    _1_6: param_slider::State,
    _1_7: param_slider::State,
    _1_8: param_slider::State,

    _2_1: param_slider::State,
    _2_3: param_slider::State,
    _2_4: param_slider::State,
    _2_5: param_slider::State,
    _2_6: param_slider::State,
    _2_7: param_slider::State,
    _2_8: param_slider::State,

    _3_1: param_slider::State,
    _3_2: param_slider::State,
    _3_4: param_slider::State,
    _3_5: param_slider::State,
    _3_6: param_slider::State,
    _3_7: param_slider::State,
    _3_8: param_slider::State,

    _4_1: param_slider::State,
    _4_2: param_slider::State,
    _4_3: param_slider::State,
    _4_5: param_slider::State,
    _4_6: param_slider::State,
    _4_7: param_slider::State,
    _4_8: param_slider::State,

    _5_1: param_slider::State,
    _5_2: param_slider::State,
    _5_3: param_slider::State,
    _5_4: param_slider::State,
    _5_6: param_slider::State,
    _5_7: param_slider::State,
    _5_8: param_slider::State,

    _6_1: param_slider::State,
    _6_2: param_slider::State,
    _6_3: param_slider::State,
    _6_4: param_slider::State,
    _6_5: param_slider::State,
    _6_7: param_slider::State,
    _6_8: param_slider::State,

    _7_1: param_slider::State,
    _7_2: param_slider::State,
    _7_3: param_slider::State,
    _7_4: param_slider::State,
    _7_5: param_slider::State,
    _7_6: param_slider::State,
    _7_8: param_slider::State,

    _8_1: param_slider::State,
    _8_2: param_slider::State,
    _8_3: param_slider::State,
    _8_4: param_slider::State,
    _8_5: param_slider::State,
    _8_6: param_slider::State,
    _8_7: param_slider::State,
}
impl MatrixWidget {
    fn ui_matrix<'a>(&'a mut self, params: &'a SynthPluginParams) -> Column<'a, Message> {
        let slider_width = 40;
        let slider_height = 16;
        let slider_font_size = 14;
        let spacing = 4;
        Column::new()
            .spacing(spacing)
            .push(
                Text::new("FM Matrix")
                    .font(assets::NOTO_SANS_BOLD)
                    .size(18)
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .vertical_alignment(alignment::Vertical::Center),
            )
            .push(
                Text::new("From")
                    .font(assets::NOTO_SANS_REGULAR)
                    .size(16)
                    .width((slider_width * 9 + spacing * 9 * 2).into())
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .vertical_alignment(alignment::Vertical::Center),
            )
            .push({
                let mut row = Row::new()
                    .spacing(spacing)
                    .push(Space::new(slider_width.into(), slider_height.into()));
                for i in 1..=8 {
                    row = row.push(
                        Text::new(i.to_string())
                            .size(14)
                            .width(slider_width.into())
                            .height(slider_height.into())
                            .horizontal_alignment(alignment::Horizontal::Center)
                            .vertical_alignment(alignment::Vertical::Center),
                    );
                }
                row
            })
            .push(
                Row::new()
                    .spacing(spacing)
                    .push(
                        Text::new("To 1".to_string())
                            .size(14)
                            .width(slider_width.into())
                            .height(slider_height.into())
                            .horizontal_alignment(alignment::Horizontal::Center)
                            .vertical_alignment(alignment::Vertical::Center),
                    )
                    .push(Space::new(slider_width.into(), slider_height.into()))
                    .push(
                        ParamSlider::new(&mut self._1_2, &params.mod_osc1_by_osc2)
                            .width(slider_width.into())
                            .height(slider_height.into())
                            .text_size(slider_font_size)
                            .map(Message::ParamUpdate),
                    )
                    .push(
                        ParamSlider::new(&mut self._1_3, &params.mod_osc1_by_osc3)
                            .width(slider_width.into())
                            .height(slider_height.into())
                            .text_size(slider_font_size)
                            .map(Message::ParamUpdate),
                    )
                    .push(
                        ParamSlider::new(&mut self._1_4, &params.mod_osc1_by_osc4)
                            .width(slider_width.into())
                            .height(slider_height.into())
                            .text_size(slider_font_size)
                            .map(Message::ParamUpdate),
                    )
                    .push(
                        ParamSlider::new(&mut self._1_5, &params.mod_osc1_by_osc5)
                            .width(slider_width.into())
                            .height(slider_height.into())
                            .text_size(slider_font_size)
                            .map(Message::ParamUpdate),
                    )
                    .push(
                        ParamSlider::new(&mut self._1_6, &params.mod_osc1_by_osc6)
                            .width(slider_width.into())
                            .height(slider_height.into())
                            .text_size(slider_font_size)
                            .map(Message::ParamUpdate),
                    )
                    .push(
                        ParamSlider::new(&mut self._1_7, &params.mod_osc1_by_osc7)
                            .width(slider_width.into())
                            .height(slider_height.into())
                            .text_size(slider_font_size)
                            .map(Message::ParamUpdate),
                    )
                    .push(
                        ParamSlider::new(&mut self._1_8, &params.mod_osc1_by_osc8)
                            .width(slider_width.into())
                            .height(slider_height.into())
                            .text_size(slider_font_size)
                            .map(Message::ParamUpdate),
                    ),
            )
            .push(
                Row::new()
                    .spacing(spacing)
                    .push(
                        Text::new("To 2".to_string())
                            .size(14)
                            .width(slider_width.into())
                            .height(slider_height.into())
                            .horizontal_alignment(alignment::Horizontal::Center)
                            .vertical_alignment(alignment::Vertical::Center),
                    )
                    .push(
                        ParamSlider::new(&mut self._2_1, &params.mod_osc2_by_osc1)
                            .width(slider_width.into())
                            .height(slider_height.into())
                            .text_size(slider_font_size)
                            .map(Message::ParamUpdate),
                    )
                    .push(Space::new(slider_width.into(), slider_height.into()))
                    .push(
                        ParamSlider::new(&mut self._2_3, &params.mod_osc2_by_osc3)
                            .width(slider_width.into())
                            .height(slider_height.into())
                            .text_size(slider_font_size)
                            .map(Message::ParamUpdate),
                    )
                    .push(
                        ParamSlider::new(&mut self._2_4, &params.mod_osc2_by_osc4)
                            .width(slider_width.into())
                            .height(slider_height.into())
                            .text_size(slider_font_size)
                            .map(Message::ParamUpdate),
                    )
                    .push(
                        ParamSlider::new(&mut self._2_5, &params.mod_osc2_by_osc5)
                            .width(slider_width.into())
                            .height(slider_height.into())
                            .text_size(slider_font_size)
                            .map(Message::ParamUpdate),
                    )
                    .push(
                        ParamSlider::new(&mut self._2_6, &params.mod_osc2_by_osc6)
                            .width(slider_width.into())
                            .height(slider_height.into())
                            .text_size(slider_font_size)
                            .map(Message::ParamUpdate),
                    )
                    .push(
                        ParamSlider::new(&mut self._2_7, &params.mod_osc2_by_osc7)
                            .width(slider_width.into())
                            .height(slider_height.into())
                            .text_size(slider_font_size)
                            .map(Message::ParamUpdate),
                    )
                    .push(
                        ParamSlider::new(&mut self._2_8, &params.mod_osc2_by_osc8)
                            .width(slider_width.into())
                            .height(slider_height.into())
                            .text_size(slider_font_size)
                            .map(Message::ParamUpdate),
                    ),
            )
            .push(
                Row::new()
                    .spacing(spacing)
                    .push(
                        Text::new("To 3".to_string())
                            .size(14)
                            .width(slider_width.into())
                            .height(slider_height.into())
                            .horizontal_alignment(alignment::Horizontal::Center)
                            .vertical_alignment(alignment::Vertical::Center),
                    )
                    .push(
                        ParamSlider::new(&mut self._3_1, &params.mod_osc3_by_osc1)
                            .width(slider_width.into())
                            .height(slider_height.into())
                            .text_size(slider_font_size)
                            .map(Message::ParamUpdate),
                    )
                    .push(
                        ParamSlider::new(&mut self._3_2, &params.mod_osc3_by_osc2)
                            .width(slider_width.into())
                            .height(slider_height.into())
                            .text_size(slider_font_size)
                            .map(Message::ParamUpdate),
                    )
                    .push(Space::new(slider_width.into(), slider_height.into()))
                    .push(
                        ParamSlider::new(&mut self._3_4, &params.mod_osc3_by_osc4)
                            .width(slider_width.into())
                            .height(slider_height.into())
                            .text_size(slider_font_size)
                            .map(Message::ParamUpdate),
                    )
                    .push(
                        ParamSlider::new(&mut self._3_5, &params.mod_osc3_by_osc5)
                            .width(slider_width.into())
                            .height(slider_height.into())
                            .text_size(slider_font_size)
                            .map(Message::ParamUpdate),
                    )
                    .push(
                        ParamSlider::new(&mut self._3_6, &params.mod_osc3_by_osc6)
                            .width(slider_width.into())
                            .height(slider_height.into())
                            .text_size(slider_font_size)
                            .map(Message::ParamUpdate),
                    )
                    .push(
                        ParamSlider::new(&mut self._3_7, &params.mod_osc3_by_osc7)
                            .width(slider_width.into())
                            .height(slider_height.into())
                            .text_size(slider_font_size)
                            .map(Message::ParamUpdate),
                    )
                    .push(
                        ParamSlider::new(&mut self._3_8, &params.mod_osc3_by_osc8)
                            .width(slider_width.into())
                            .height(slider_height.into())
                            .text_size(slider_font_size)
                            .map(Message::ParamUpdate),
                    ),
            )
            .push(
                Row::new()
                    .spacing(spacing)
                    .push(
                        Text::new("To 4".to_string())
                            .size(14)
                            .width(slider_width.into())
                            .height(slider_height.into())
                            .horizontal_alignment(alignment::Horizontal::Center)
                            .vertical_alignment(alignment::Vertical::Center),
                    )
                    .push(
                        ParamSlider::new(&mut self._4_1, &params.mod_osc4_by_osc1)
                            .width(slider_width.into())
                            .height(slider_height.into())
                            .text_size(slider_font_size)
                            .map(Message::ParamUpdate),
                    )
                    .push(
                        ParamSlider::new(&mut self._4_2, &params.mod_osc4_by_osc2)
                            .width(slider_width.into())
                            .height(slider_height.into())
                            .text_size(slider_font_size)
                            .map(Message::ParamUpdate),
                    )
                    .push(
                        ParamSlider::new(&mut self._4_3, &params.mod_osc4_by_osc3)
                            .width(slider_width.into())
                            .height(slider_height.into())
                            .text_size(slider_font_size)
                            .map(Message::ParamUpdate),
                    )
                    .push(Space::new(slider_width.into(), slider_height.into()))
                    .push(
                        ParamSlider::new(&mut self._4_5, &params.mod_osc4_by_osc5)
                            .width(slider_width.into())
                            .height(slider_height.into())
                            .text_size(slider_font_size)
                            .map(Message::ParamUpdate),
                    )
                    .push(
                        ParamSlider::new(&mut self._4_6, &params.mod_osc4_by_osc6)
                            .width(slider_width.into())
                            .height(slider_height.into())
                            .text_size(slider_font_size)
                            .map(Message::ParamUpdate),
                    )
                    .push(
                        ParamSlider::new(&mut self._4_7, &params.mod_osc4_by_osc7)
                            .width(slider_width.into())
                            .height(slider_height.into())
                            .text_size(slider_font_size)
                            .map(Message::ParamUpdate),
                    )
                    .push(
                        ParamSlider::new(&mut self._4_8, &params.mod_osc4_by_osc8)
                            .width(slider_width.into())
                            .height(slider_height.into())
                            .text_size(slider_font_size)
                            .map(Message::ParamUpdate),
                    ),
            )
            .push(
                Row::new()
                    .spacing(spacing)
                    .push(
                        Text::new("To 5".to_string())
                            .size(14)
                            .width(slider_width.into())
                            .height(slider_height.into())
                            .horizontal_alignment(alignment::Horizontal::Center)
                            .vertical_alignment(alignment::Vertical::Center),
                    )
                    .push(
                        ParamSlider::new(&mut self._5_1, &params.mod_osc5_by_osc1)
                            .width(slider_width.into())
                            .height(slider_height.into())
                            .text_size(slider_font_size)
                            .map(Message::ParamUpdate),
                    )
                    .push(
                        ParamSlider::new(&mut self._5_2, &params.mod_osc5_by_osc2)
                            .width(slider_width.into())
                            .height(slider_height.into())
                            .text_size(slider_font_size)
                            .map(Message::ParamUpdate),
                    )
                    .push(
                        ParamSlider::new(&mut self._5_3, &params.mod_osc5_by_osc3)
                            .width(slider_width.into())
                            .height(slider_height.into())
                            .text_size(slider_font_size)
                            .map(Message::ParamUpdate),
                    )
                    .push(
                        ParamSlider::new(&mut self._5_4, &params.mod_osc5_by_osc4)
                            .width(slider_width.into())
                            .height(slider_height.into())
                            .text_size(slider_font_size)
                            .map(Message::ParamUpdate),
                    )
                    .push(Space::new(slider_width.into(), slider_height.into()))
                    .push(
                        ParamSlider::new(&mut self._5_6, &params.mod_osc5_by_osc6)
                            .width(slider_width.into())
                            .height(slider_height.into())
                            .text_size(slider_font_size)
                            .map(Message::ParamUpdate),
                    )
                    .push(
                        ParamSlider::new(&mut self._5_7, &params.mod_osc5_by_osc7)
                            .width(slider_width.into())
                            .height(slider_height.into())
                            .text_size(slider_font_size)
                            .map(Message::ParamUpdate),
                    )
                    .push(
                        ParamSlider::new(&mut self._5_8, &params.mod_osc5_by_osc8)
                            .width(slider_width.into())
                            .height(slider_height.into())
                            .text_size(slider_font_size)
                            .map(Message::ParamUpdate),
                    ),
            )
            .push(
                Row::new()
                    .spacing(spacing)
                    .push(
                        Text::new("To 6".to_string())
                            .size(14)
                            .width(slider_width.into())
                            .height(slider_height.into())
                            .horizontal_alignment(alignment::Horizontal::Center)
                            .vertical_alignment(alignment::Vertical::Center),
                    )
                    .push(
                        ParamSlider::new(&mut self._6_1, &params.mod_osc6_by_osc1)
                            .width(slider_width.into())
                            .height(slider_height.into())
                            .text_size(slider_font_size)
                            .map(Message::ParamUpdate),
                    )
                    .push(
                        ParamSlider::new(&mut self._6_2, &params.mod_osc6_by_osc2)
                            .width(slider_width.into())
                            .height(slider_height.into())
                            .text_size(slider_font_size)
                            .map(Message::ParamUpdate),
                    )
                    .push(
                        ParamSlider::new(&mut self._6_3, &params.mod_osc6_by_osc3)
                            .width(slider_width.into())
                            .height(slider_height.into())
                            .text_size(slider_font_size)
                            .map(Message::ParamUpdate),
                    )
                    .push(
                        ParamSlider::new(&mut self._6_4, &params.mod_osc6_by_osc4)
                            .width(slider_width.into())
                            .height(slider_height.into())
                            .text_size(slider_font_size)
                            .map(Message::ParamUpdate),
                    )
                    .push(
                        ParamSlider::new(&mut self._6_5, &params.mod_osc6_by_osc5)
                            .width(slider_width.into())
                            .height(slider_height.into())
                            .text_size(slider_font_size)
                            .map(Message::ParamUpdate),
                    )
                    .push(Space::new(slider_width.into(), slider_height.into()))
                    .push(
                        ParamSlider::new(&mut self._6_7, &params.mod_osc6_by_osc7)
                            .width(slider_width.into())
                            .height(slider_height.into())
                            .text_size(slider_font_size)
                            .map(Message::ParamUpdate),
                    )
                    .push(
                        ParamSlider::new(&mut self._6_8, &params.mod_osc6_by_osc8)
                            .width(slider_width.into())
                            .height(slider_height.into())
                            .text_size(slider_font_size)
                            .map(Message::ParamUpdate),
                    ),
            )
            .push(
                Row::new()
                    .spacing(spacing)
                    .push(
                        Text::new("To 7".to_string())
                            .size(14)
                            .width(slider_width.into())
                            .height(slider_height.into())
                            .horizontal_alignment(alignment::Horizontal::Center)
                            .vertical_alignment(alignment::Vertical::Center),
                    )
                    .push(
                        ParamSlider::new(&mut self._7_1, &params.mod_osc7_by_osc1)
                            .width(slider_width.into())
                            .height(slider_height.into())
                            .text_size(slider_font_size)
                            .map(Message::ParamUpdate),
                    )
                    .push(
                        ParamSlider::new(&mut self._7_2, &params.mod_osc7_by_osc2)
                            .width(slider_width.into())
                            .height(slider_height.into())
                            .text_size(slider_font_size)
                            .map(Message::ParamUpdate),
                    )
                    .push(
                        ParamSlider::new(&mut self._7_3, &params.mod_osc7_by_osc3)
                            .width(slider_width.into())
                            .height(slider_height.into())
                            .text_size(slider_font_size)
                            .map(Message::ParamUpdate),
                    )
                    .push(
                        ParamSlider::new(&mut self._7_4, &params.mod_osc7_by_osc4)
                            .width(slider_width.into())
                            .height(slider_height.into())
                            .text_size(slider_font_size)
                            .map(Message::ParamUpdate),
                    )
                    .push(
                        ParamSlider::new(&mut self._7_5, &params.mod_osc7_by_osc5)
                            .width(slider_width.into())
                            .height(slider_height.into())
                            .text_size(slider_font_size)
                            .map(Message::ParamUpdate),
                    )
                    .push(
                        ParamSlider::new(&mut self._7_6, &params.mod_osc7_by_osc6)
                        .width(slider_width.into())
                        .height(slider_height.into())
                        .text_size(slider_font_size)
                        .map(Message::ParamUpdate),
                    )
                    .push(Space::new(slider_width.into(), slider_height.into()))
                    .push(
                        ParamSlider::new(&mut self._7_8, &params.mod_osc7_by_osc8)
                            .width(slider_width.into())
                            .height(slider_height.into())
                            .text_size(slider_font_size)
                            .map(Message::ParamUpdate),
                    ),
            )
            .push(
                Row::new()
                    .spacing(spacing)
                    .push(
                        Text::new("To 8".to_string())
                            .size(14)
                            .width(slider_width.into())
                            .height(slider_height.into())
                            .horizontal_alignment(alignment::Horizontal::Center)
                            .vertical_alignment(alignment::Vertical::Center),
                    )
                    .push(
                        ParamSlider::new(&mut self._8_1, &params.mod_osc8_by_osc1)
                            .width(slider_width.into())
                            .height(slider_height.into())
                            .text_size(slider_font_size)
                            .map(Message::ParamUpdate),
                    )
                    .push(
                        ParamSlider::new(&mut self._8_2, &params.mod_osc8_by_osc2)
                            .width(slider_width.into())
                            .height(slider_height.into())
                            .text_size(slider_font_size)
                            .map(Message::ParamUpdate),
                    )
                    .push(
                        ParamSlider::new(&mut self._8_3, &params.mod_osc8_by_osc3)
                            .width(slider_width.into())
                            .height(slider_height.into())
                            .text_size(slider_font_size)
                            .map(Message::ParamUpdate),
                    )
                    .push(
                        ParamSlider::new(&mut self._8_4, &params.mod_osc8_by_osc4)
                            .width(slider_width.into())
                            .height(slider_height.into())
                            .text_size(slider_font_size)
                            .map(Message::ParamUpdate),
                    )
                    .push(
                        ParamSlider::new(&mut self._8_5, &params.mod_osc8_by_osc5)
                            .width(slider_width.into())
                            .height(slider_height.into())
                            .text_size(slider_font_size)
                            .map(Message::ParamUpdate),
                    )
                    .push(
                        ParamSlider::new(&mut self._8_6, &params.mod_osc8_by_osc6)
                        .width(slider_width.into())
                        .height(slider_height.into())
                        .text_size(slider_font_size)
                        .map(Message::ParamUpdate),
                    )
                    .push(
                        ParamSlider::new(&mut self._8_7, &params.mod_osc8_by_osc7)
                            .width(slider_width.into())
                            .height(slider_height.into())
                            .text_size(slider_font_size)
                            .map(Message::ParamUpdate),
                    )
                    .push(Space::new(slider_width.into(), slider_height.into())),
            )
    }
}

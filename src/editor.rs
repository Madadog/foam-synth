use nih_plug::prelude::FloatParam;
use nih_plug::prelude::{util, Editor, GuiContext};
use nih_plug_iced::canvas::Cache;
use nih_plug_iced::renderer::Renderer;
use nih_plug_iced::widget::image;
use nih_plug_iced::widgets as nih_widgets;
use nih_plug_iced::IcedState;
use nih_plug_iced::*;
use std::default;
use std::sync::Arc;

use crate::parameters::{OscillatorParams, SynthPluginParams};

use self::param_slider::ParamSlider;

mod envelope;
mod param_slider;

pub(crate) fn default_state() -> Arc<IcedState> {
    IcedState::from_size(1000, 650)
}

pub(crate) fn create(
    params: Arc<SynthPluginParams>,
    editor_state: Arc<IcedState>,
) -> Option<Box<dyn Editor>> {
    create_iced_editor::<SynthPluginEditor>(editor_state, params)
}

#[derive(Debug, Clone)]
struct CanvasTest {
    size: f32,
    attack: f32,
    decay: f32,
    sustain: f32,
    release: f32,
}
impl CanvasTest {
    fn new(attack: f32, decay: f32, sustain: f32, release: f32) -> Self {
        Self {
            size: 5.0,
            attack,
            decay,
            sustain,
            release,
        }
    }
}
impl canvas::Program<Message> for CanvasTest {
    fn draw(&self, bounds: Rectangle, _cursor: canvas::Cursor) -> Vec<canvas::Geometry> {
        // We prepare a new `Frame`
        let mut frame = canvas::Frame::new(bounds.size());

        // We create a `Path` representing a simple circle
        let circle = canvas::Path::circle(frame.center(), self.size);

        let attack = canvas::Path::line(
            Point::new(0.0, -bounds.height),
            Point::new(self.attack * 10.0, 0.0),
        );
        let decay = canvas::Path::line(
            Point::new(self.attack * 10.0, bounds.height),
            Point::new(
                self.attack * 10.0 + self.decay * 10.0,
                self.sustain * bounds.height,
            ),
        );
        let sustain = canvas::Path::line(
            Point::new(self.attack * 10.0 + self.decay * 10.0, self.sustain * 10.0),
            Point::new(
                self.attack * 10.0 + self.decay * 10.0 + 10.0,
                self.sustain * bounds.height,
            ),
        );
        let release = canvas::Path::line(
            Point::new(
                self.attack * 10.0 + self.decay * 10.0 + 10.0,
                self.sustain * bounds.height,
            ),
            Point::new(
                self.attack * 10.0 + self.decay * 10.0 + self.release * 10.0 + 10.0,
                0.0,
            ),
        );

        for i in [attack, decay, sustain, release] {
            frame.stroke(&i, canvas::Stroke::default())
        }

        // And fill it with some color
        // frame.fill(&circle, Color::BLACK);

        // Finally, we produce the geometry
        vec![frame.into_geometry()]
    }
}

struct SynthPluginEditor {
    params: Arc<SynthPluginParams>,
    context: Arc<dyn GuiContext>,

    gain_slider_state: param_slider::State,
    octave_stretch_slider_state: param_slider::State,
    scrollable: widget::scrollable::State,

    filter_params: FilterWidget,
    global_envelope: GlobalEnvelopeWidget,

    osc_params_1: OscillatorWidget,
    osc_params_2: OscillatorWidget,
    osc_params_3: OscillatorWidget,
    osc_params_4: OscillatorWidget,
    osc_params_5: OscillatorWidget,
    osc_params_6: OscillatorWidget,
    osc_params_7: OscillatorWidget,
    osc_params_8: OscillatorWidget,

    // canvas: Canvas<Message, CanvasTest>,
    // canvas_cache: Arc<Cache>,
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

            filter_params: Default::default(),
            global_envelope: Default::default(),

            osc_params_1: OscillatorWidget::new("Osc 1"),
            osc_params_2: OscillatorWidget::new("Osc 2"),
            osc_params_3: OscillatorWidget::new("Osc 3"),
            osc_params_4: OscillatorWidget::new("Osc 4"),
            osc_params_5: OscillatorWidget::new("Osc 5"),
            osc_params_6: OscillatorWidget::new("Osc 6"),
            osc_params_7: OscillatorWidget::new("Osc 7"),
            osc_params_8: OscillatorWidget::new("Osc 8"),

            // canvas: Canvas::new(CanvasTest::new(1.0, 1.0, 0.5, 1.0)),
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
                    .push(self.matrix.ui_matrix(&self.params))
                    .push(self.filter_params.ui(&self.params))
                    .push(self.global_envelope.ui(&self.params))
                    .push(Canvas::new(CanvasTest::new(1.0, 1.0, 0.5, 1.0)))
                    .push(
                        title_bar()
                            .push(Space::with_height(10.into()))
                            .push(
                                Text::new("Output Gain")
                                    .size(16)
                                    .width(100.into())
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
                                    .width(100.into())
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
                                .push(self.osc_params_1.content(&self.params.osc1_params))
                                .push(self.osc_params_2.content(&self.params.osc2_params))
                                .push(self.osc_params_3.content(&self.params.osc3_params))
                                .push(self.osc_params_4.content(&self.params.osc4_params)),
                        )
                        .push(
                            Row::new()
                                .padding(Padding::from(5))
                                .spacing(20)
                                .push(self.osc_params_5.content(&self.params.osc5_params))
                                .push(self.osc_params_6.content(&self.params.osc6_params))
                                .push(self.osc_params_7.content(&self.params.osc7_params))
                                .push(self.osc_params_8.content(&self.params.osc8_params)),
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
    pub hz_detune: param_slider::State,
    pub phase_offset: param_slider::State,
    pub phase_rand: param_slider::State,
    pub attack_level: param_slider::State,
    pub release_level: param_slider::State,
    pub delay: param_slider::State,
    pub attack: param_slider::State,
    pub hold: param_slider::State,
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
            hz_detune: Default::default(),
            phase_offset: Default::default(),
            phase_rand: Default::default(),
            attack_level: Default::default(),
            release_level: Default::default(),
            delay: Default::default(),
            attack: Default::default(),
            hold: Default::default(),
            decay: Default::default(),
            sustain: Default::default(),
            release: Default::default(),
            feedback: Default::default(),
            velocity_sensitivity: Default::default(),
            keyscaling: Default::default(),
        }
    }
    fn content<'a>(&'a mut self, osc_params: &'a OscillatorParams) -> Column<Message> {
        let param_font_size = 14;
        let slider_font_size = 14;
        let slider_width = 60;
        let slider_height = 14;
        Column::new()
            .push(
                Text::new(self.name)
                    .size(18)
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .font(assets::NOTO_SANS_BOLD),
            )
            .push(
                Row::new()
                    .push(
                        Column::new()
                            .push(Text::new("Feedback").size(param_font_size))
                            .push(
                                ParamSlider::new(&mut self.feedback, &osc_params.feedback)
                                    .width(slider_width.into())
                                    .height(slider_height.into())
                                    .text_size(slider_font_size)
                                    .map(Message::ParamUpdate),
                            )
                            .push(Text::new("Coarse Det.").size(param_font_size))
                            .push(
                                ParamSlider::new(&mut self.coarse, &osc_params.coarse)
                                    .width(slider_width.into())
                                    .height(slider_height.into())
                                    .text_size(slider_font_size)
                                    .map(Message::ParamUpdate),
                            )
                            .push(Text::new("Fine Detune").size(param_font_size))
                            .push(
                                ParamSlider::new(&mut self.fine, &osc_params.fine)
                                    .width(slider_width.into())
                                    .height(slider_height.into())
                                    .text_size(slider_font_size)
                                    .map(Message::ParamUpdate),
                            )
                            .push(Text::new("Freq. Mult.").size(param_font_size))
                            .push(
                                ParamSlider::new(&mut self.freq_mult, &osc_params.freq_mult)
                                    .width(slider_width.into())
                                    .height(slider_height.into())
                                    .text_size(slider_font_size)
                                    .map(Message::ParamUpdate),
                            )
                            .push(Text::new("Freq. Divide").size(param_font_size))
                            .push(
                                ParamSlider::new(&mut self.freq_div, &osc_params.freq_div)
                                    .width(slider_width.into())
                                    .height(slider_height.into())
                                    .text_size(slider_font_size)
                                    .map(Message::ParamUpdate),
                            )
                            .push(Text::new("Hz Detune").size(param_font_size))
                            .push(
                                ParamSlider::new(&mut self.hz_detune, &osc_params.hz_detune)
                                    .width(slider_width.into())
                                    .height(slider_height.into())
                                    .text_size(slider_font_size)
                                    .map(Message::ParamUpdate),
                            ),
                    )
                    .push(Space::with_width(8.into()))
                    .push(
                        Column::new()
                            .push(Text::new("Delay").size(param_font_size))
                            .push(
                                ParamSlider::new(&mut self.delay, &osc_params.delay)
                                    .width(slider_width.into())
                                    .height(slider_height.into())
                                    .text_size(slider_font_size)
                                    .map(Message::ParamUpdate),
                            )
                            .push(Text::new("Attack").size(param_font_size))
                            .push(
                                ParamSlider::new(&mut self.attack, &osc_params.attack)
                                    .width(slider_width.into())
                                    .height(slider_height.into())
                                    .text_size(slider_font_size)
                                    .map(Message::ParamUpdate),
                            )
                            .push(Text::new("Hold").size(param_font_size))
                            .push(
                                ParamSlider::new(&mut self.hold, &osc_params.hold)
                                    .width(slider_width.into())
                                    .height(slider_height.into())
                                    .text_size(slider_font_size)
                                    .map(Message::ParamUpdate),
                            )
                            .push(Text::new("Decay").size(param_font_size))
                            .push(
                                ParamSlider::new(&mut self.decay, &osc_params.decay)
                                    .width(slider_width.into())
                                    .height(slider_height.into())
                                    .text_size(slider_font_size)
                                    .map(Message::ParamUpdate),
                            )
                            .push(Text::new("Sustain").size(param_font_size))
                            .push(
                                ParamSlider::new(&mut self.sustain, &osc_params.sustain)
                                    .width(slider_width.into())
                                    .height(slider_height.into())
                                    .text_size(slider_font_size)
                                    .map(Message::ParamUpdate),
                            )
                            .push(Text::new("Release").size(param_font_size))
                            .push(
                                ParamSlider::new(&mut self.release, &osc_params.release)
                                    .width(slider_width.into())
                                    .height(slider_height.into())
                                    .text_size(slider_font_size)
                                    .map(Message::ParamUpdate),
                            ),
                    )
                    .push(Space::with_width(8.into()))
                    .push(
                        Column::new()
                            .push(Text::new("Amplitude").size(param_font_size))
                            .push(
                                ParamSlider::new(&mut self.amp, &osc_params.amp)
                                    .width(slider_width.into())
                                    .height(slider_height.into())
                                    .text_size(slider_font_size)
                                    .map(Message::ParamUpdate),
                            )
                            .push(Text::new("Atk. Level").size(param_font_size))
                            .push(
                                ParamSlider::new(&mut self.attack_level, &osc_params.attack_level)
                                    .width(slider_width.into())
                                    .height(slider_height.into())
                                    .text_size(slider_font_size)
                                    .map(Message::ParamUpdate),
                            )
                            .push(Text::new("Phase").size(param_font_size))
                            .push(
                                ParamSlider::new(&mut self.phase_offset, &osc_params.phase_offset)
                                    .width(slider_width.into())
                                    .height(slider_height.into())
                                    .text_size(slider_font_size)
                                    .map(Message::ParamUpdate),
                            )
                            .push(Text::new("Velo. Sens.").size(param_font_size))
                            .push(
                                ParamSlider::new(
                                    &mut self.velocity_sensitivity,
                                    &osc_params.velocity_sensitivity,
                                )
                                .width(slider_width.into())
                                .height(slider_height.into())
                                .text_size(slider_font_size)
                                .map(Message::ParamUpdate),
                            )
                            .push(Text::new("Keyscaling").size(param_font_size))
                            .push(
                                ParamSlider::new(&mut self.keyscaling, &osc_params.keyscaling)
                                    .width(slider_width.into())
                                    .height(slider_height.into())
                                    .text_size(slider_font_size)
                                    .map(Message::ParamUpdate),
                            )
                            .push(Text::new("Rls. Level").size(param_font_size))
                            .push(
                                ParamSlider::new(
                                    &mut self.release_level,
                                    &osc_params.release_level,
                                )
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
        .align_items(Alignment::Start)
        .max_width(150)
        .push(
            Text::new("Foam Synth GUI")
                .font(assets::NOTO_SANS_LIGHT)
                .size(20)
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
    _1_1: param_slider::State,
    _1_2: param_slider::State,
    _1_3: param_slider::State,
    _1_4: param_slider::State,
    _1_5: param_slider::State,
    _1_6: param_slider::State,
    _1_7: param_slider::State,
    _1_8: param_slider::State,

    _2_1: param_slider::State,
    _2_2: param_slider::State,
    _2_3: param_slider::State,
    _2_4: param_slider::State,
    _2_5: param_slider::State,
    _2_6: param_slider::State,
    _2_7: param_slider::State,
    _2_8: param_slider::State,

    _3_1: param_slider::State,
    _3_2: param_slider::State,
    _3_3: param_slider::State,
    _3_4: param_slider::State,
    _3_5: param_slider::State,
    _3_6: param_slider::State,
    _3_7: param_slider::State,
    _3_8: param_slider::State,

    _4_1: param_slider::State,
    _4_2: param_slider::State,
    _4_3: param_slider::State,
    _4_4: param_slider::State,
    _4_5: param_slider::State,
    _4_6: param_slider::State,
    _4_7: param_slider::State,
    _4_8: param_slider::State,

    _5_1: param_slider::State,
    _5_2: param_slider::State,
    _5_3: param_slider::State,
    _5_4: param_slider::State,
    _5_5: param_slider::State,
    _5_6: param_slider::State,
    _5_7: param_slider::State,
    _5_8: param_slider::State,

    _6_1: param_slider::State,
    _6_2: param_slider::State,
    _6_3: param_slider::State,
    _6_4: param_slider::State,
    _6_5: param_slider::State,
    _6_6: param_slider::State,
    _6_7: param_slider::State,
    _6_8: param_slider::State,

    _7_1: param_slider::State,
    _7_2: param_slider::State,
    _7_3: param_slider::State,
    _7_4: param_slider::State,
    _7_5: param_slider::State,
    _7_6: param_slider::State,
    _7_7: param_slider::State,
    _7_8: param_slider::State,

    _8_1: param_slider::State,
    _8_2: param_slider::State,
    _8_3: param_slider::State,
    _8_4: param_slider::State,
    _8_5: param_slider::State,
    _8_6: param_slider::State,
    _8_7: param_slider::State,
    _8_8: param_slider::State,

    osc1_amp: param_slider::State,
    osc2_amp: param_slider::State,
    osc3_amp: param_slider::State,
    osc4_amp: param_slider::State,
    osc5_amp: param_slider::State,
    osc6_amp: param_slider::State,
    osc7_amp: param_slider::State,
    osc8_amp: param_slider::State,
}
impl MatrixWidget {
    fn ui_matrix<'a>(&'a mut self, params: &'a SynthPluginParams) -> Column<'a, Message> {
        let slider_width = 30;
        let slider_height = 14;
        let slider_font_size = 12;
        let spacing = 4;
        Column::new()
            .spacing(spacing)
            .push(
                Row::new()
                    .push(
                        Text::new("FM Matrix")
                            .font(assets::NOTO_SANS_BOLD)
                            .size(18)
                            .horizontal_alignment(alignment::Horizontal::Left)
                            .vertical_alignment(alignment::Vertical::Center),
                    )
                    .push(
                        Text::new("From")
                            .font(assets::NOTO_SANS_REGULAR)
                            .size(16)
                            .width((slider_width * 6 + spacing * 6 * 2).into())
                            .horizontal_alignment(alignment::Horizontal::Center)
                            .vertical_alignment(alignment::Vertical::Center),
                    ),
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
                    .push(
                        ParamSlider::new(&mut self._1_1, &params.mod_osc1_by_osc1)
                            .width(slider_width.into())
                            .height(slider_height.into())
                            .text_size(slider_font_size)
                            .map(Message::ParamUpdate),
                    )
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
                    .push(
                        ParamSlider::new(&mut self._2_2, &params.mod_osc2_by_osc2)
                            .width(slider_width.into())
                            .height(slider_height.into())
                            .text_size(slider_font_size)
                            .map(Message::ParamUpdate),
                    )
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
                    .push(
                        ParamSlider::new(&mut self._3_3, &params.mod_osc3_by_osc3)
                            .width(slider_width.into())
                            .height(slider_height.into())
                            .text_size(slider_font_size)
                            .map(Message::ParamUpdate),
                    )
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
                    .push(
                        ParamSlider::new(&mut self._4_4, &params.mod_osc4_by_osc4)
                            .width(slider_width.into())
                            .height(slider_height.into())
                            .text_size(slider_font_size)
                            .map(Message::ParamUpdate),
                    )
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
                    .push(
                        ParamSlider::new(&mut self._5_5, &params.mod_osc5_by_osc5)
                            .width(slider_width.into())
                            .height(slider_height.into())
                            .text_size(slider_font_size)
                            .map(Message::ParamUpdate),
                    )
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
                    .push(
                        ParamSlider::new(&mut self._6_6, &params.mod_osc6_by_osc6)
                            .width(slider_width.into())
                            .height(slider_height.into())
                            .text_size(slider_font_size)
                            .map(Message::ParamUpdate),
                    )
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
                    .push(
                        ParamSlider::new(&mut self._7_7, &params.mod_osc7_by_osc7)
                            .width(slider_width.into())
                            .height(slider_height.into())
                            .text_size(slider_font_size)
                            .map(Message::ParamUpdate),
                    )
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
                    .push(
                        ParamSlider::new(&mut self._8_8, &params.mod_osc8_by_osc8)
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
                        Text::new("Out".to_string())
                            .size(14)
                            .width(slider_width.into())
                            .height(slider_height.into())
                            .horizontal_alignment(alignment::Horizontal::Center)
                            .vertical_alignment(alignment::Vertical::Center),
                    )
                    .push(
                        ParamSlider::new(&mut self.osc1_amp, &params.osc1_params.amp)
                            .width(slider_width.into())
                            .height(slider_height.into())
                            .text_size(slider_font_size)
                            .map(Message::ParamUpdate),
                    )
                    .push(
                        ParamSlider::new(&mut self.osc2_amp, &params.osc2_params.amp)
                            .width(slider_width.into())
                            .height(slider_height.into())
                            .text_size(slider_font_size)
                            .map(Message::ParamUpdate),
                    )
                    .push(
                        ParamSlider::new(&mut self.osc3_amp, &params.osc3_params.amp)
                            .width(slider_width.into())
                            .height(slider_height.into())
                            .text_size(slider_font_size)
                            .map(Message::ParamUpdate),
                    )
                    .push(
                        ParamSlider::new(&mut self.osc4_amp, &params.osc4_params.amp)
                            .width(slider_width.into())
                            .height(slider_height.into())
                            .text_size(slider_font_size)
                            .map(Message::ParamUpdate),
                    )
                    .push(
                        ParamSlider::new(&mut self.osc5_amp, &params.osc5_params.amp)
                            .width(slider_width.into())
                            .height(slider_height.into())
                            .text_size(slider_font_size)
                            .map(Message::ParamUpdate),
                    )
                    .push(
                        ParamSlider::new(&mut self.osc6_amp, &params.osc6_params.amp)
                            .width(slider_width.into())
                            .height(slider_height.into())
                            .text_size(slider_font_size)
                            .map(Message::ParamUpdate),
                    )
                    .push(
                        ParamSlider::new(&mut self.osc7_amp, &params.osc7_params.amp)
                            .width(slider_width.into())
                            .height(slider_height.into())
                            .text_size(slider_font_size)
                            .map(Message::ParamUpdate),
                    )
                    .push(
                        ParamSlider::new(&mut self.osc8_amp, &params.osc8_params.amp)
                            .width(slider_width.into())
                            .height(slider_height.into())
                            .text_size(slider_font_size)
                            .map(Message::ParamUpdate),
                    ),
            )
    }
}

#[derive(Default)]
struct FilterWidget {
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
}
impl FilterWidget {
    fn ui<'a>(&'a mut self, params: &'a SynthPluginParams) -> Column<'a, Message> {
        let slider_height: Length = 14.into();
        let slider_width: Length = 60.into();
        let slider_font_size = 14;
        let font_size = 14;
        Column::new()
            .max_width(200)
            // .push(Space::with_height(20.into()))
            .push(
                Text::new("Filter")
                    .size(18)
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .font(assets::NOTO_SANS_BOLD),
            )
            .push(
                Row::new()
                    // .spacing(10)
                    .push(
                        Column::new()
                            .max_width(90)
                            .push(Text::new("Enabled").size(font_size))
                            .push(
                                ParamSlider::new(
                                    &mut self.filter_enabled_slider_state,
                                    &params.filter_enabled,
                                )
                                .height(slider_height)
                                .width(slider_width)
                                .text_size(slider_font_size)
                                .map(Message::ParamUpdate),
                            )
                            .push(Text::new("Type").size(font_size))
                            .push(
                                ParamSlider::new(
                                    &mut self.filter_type_slider_state,
                                    &params.filter_type,
                                )
                                .height(slider_height)
                                .width(slider_width)
                                .text_size(slider_font_size)
                                .map(Message::ParamUpdate),
                            )
                            .push(Text::new("Cutoff").size(font_size))
                            .push(
                                ParamSlider::new(
                                    &mut self.filter_cutoff_slider_state,
                                    &params.filter_cutoff,
                                )
                                .height(slider_height)
                                .width(slider_width)
                                .text_size(slider_font_size)
                                .map(Message::ParamUpdate),
                            )
                            .push(Text::new("Resonance").size(font_size))
                            .push(
                                ParamSlider::new(
                                    &mut self.filter_resonance_slider_state,
                                    &params.filter_resonance,
                                )
                                .height(slider_height)
                                .width(slider_width)
                                .text_size(slider_font_size)
                                .map(Message::ParamUpdate),
                            )
                            .push(Text::new("Keytrack").size(font_size))
                            .push(
                                ParamSlider::new(
                                    &mut self.filter_keytrack_slider_state,
                                    &params.filter_keytrack,
                                )
                                .height(slider_height)
                                .width(slider_width)
                                .text_size(slider_font_size)
                                .map(Message::ParamUpdate),
                            ),
                    )
                    .push(Space::with_width(8.into()))
                    .push(
                        Column::new()
                            .push(Text::new("Env. Amt.").size(font_size))
                            .push(
                                ParamSlider::new(
                                    &mut self.filter_envelope_amount_slider_state,
                                    &params.filter_envelope_amount,
                                )
                                .height(slider_height)
                                .width(slider_width)
                                .text_size(slider_font_size)
                                .map(Message::ParamUpdate),
                            )
                            .push(Text::new("Fil. Attack").size(font_size))
                            .push(
                                ParamSlider::new(
                                    &mut self.filter_envelope_attack_slider_state,
                                    &params.filter_envelope_attack,
                                )
                                .height(slider_height)
                                .width(slider_width)
                                .text_size(slider_font_size)
                                .map(Message::ParamUpdate),
                            )
                            .push(Text::new("Fil. Decay").size(font_size))
                            .push(
                                ParamSlider::new(
                                    &mut self.filter_envelope_decay_slider_state,
                                    &params.filter_envelope_decay,
                                )
                                .height(slider_height)
                                .width(slider_width)
                                .text_size(slider_font_size)
                                .map(Message::ParamUpdate),
                            )
                            .push(Text::new("Fil. Sustain").size(font_size))
                            .push(
                                ParamSlider::new(
                                    &mut self.filter_envelope_sustain_slider_state,
                                    &params.filter_envelope_sustain,
                                )
                                .height(slider_height)
                                .width(slider_width)
                                .text_size(slider_font_size)
                                .map(Message::ParamUpdate),
                            )
                            .push(Text::new("Fil. Release").size(font_size))
                            .push(
                                ParamSlider::new(
                                    &mut self.filter_envelope_release_slider_state,
                                    &params.filter_envelope_release,
                                )
                                .height(slider_height)
                                .width(slider_width)
                                .text_size(slider_font_size)
                                .map(Message::ParamUpdate),
                            ),
                    ),
            )
    }
}

#[derive(Default)]
struct GlobalEnvelopeWidget {
    attack: param_slider::State,
    decay: param_slider::State,
    sustain: param_slider::State,
    release: param_slider::State,
}
impl GlobalEnvelopeWidget {
    fn ui<'a>(&'a mut self, params: &'a SynthPluginParams) -> Column<'a, Message> {
        let slider_height: Length = 14.into();
        let slider_width: Length = 60.into();
        let slider_font_size = 14;
        let font_size = 14;
        Column::new()
            .max_width(200)
            // .push(Space::with_height(20.into()))
            .push(
                Text::new("Amp Env.")
                    .size(18)
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .font(assets::NOTO_SANS_BOLD),
            )
            .push(
                Row::new()
                    // .spacing(10)
                    .push(
                        Column::new()
                            .max_width(90)
                            .push(Text::new("Attack").size(font_size))
                            .push(
                                ParamSlider::new(&mut self.attack, &params.global_attack)
                                    .height(slider_height)
                                    .width(slider_width)
                                    .text_size(slider_font_size)
                                    .map(Message::ParamUpdate),
                            )
                            .push(Text::new("Decay").size(font_size))
                            .push(
                                ParamSlider::new(&mut self.decay, &params.global_decay)
                                    .height(slider_height)
                                    .width(slider_width)
                                    .text_size(slider_font_size)
                                    .map(Message::ParamUpdate),
                            )
                            .push(Text::new("Sustain").size(font_size))
                            .push(
                                ParamSlider::new(&mut self.sustain, &params.global_sustain)
                                    .height(slider_height)
                                    .width(slider_width)
                                    .text_size(slider_font_size)
                                    .map(Message::ParamUpdate),
                            )
                            .push(Text::new("Release").size(font_size))
                            .push(
                                ParamSlider::new(&mut self.release, &params.global_release)
                                    .height(slider_height)
                                    .width(slider_width)
                                    .text_size(slider_font_size)
                                    .map(Message::ParamUpdate),
                            ),
                    ),
            )
    }
}

use nih_plug::prelude::FloatParam;
use nih_plug::prelude::{util, Editor, GuiContext};
use nih_plug_iced::canvas::{Cache, Fill};
use nih_plug_iced::renderer::Renderer;
use nih_plug_iced::widget::image;
use nih_plug_iced::widgets as nih_widgets;
use nih_plug_iced::IcedState;
use nih_plug_iced::*;
use std::default;
use std::sync::Arc;
use wide::f32x8;

use crate::parameters::{OscMod, OscillatorParams, SynthPluginParams};
use crate::voice::{OscParams, OscParamsBatch, OscillatorBatch};

use self::param_slider::ParamSlider;

mod envelope;
mod param_slider;

pub(crate) fn default_state() -> Arc<IcedState> {
    IcedState::from_size(1150, 650)
}
pub(crate) fn editor_state_with_scale(scale: f32) -> Arc<IcedState> {
    IcedState::from_size((1150 as f32 * scale) as u32, (650 as f32 * scale) as u32)
}

pub(crate) fn create(
    params: Arc<SynthPluginParams>,
    editor_state: Arc<IcedState>,
) -> Option<Box<dyn Editor>> {
    create_iced_editor::<SynthPluginEditor>(editor_state, params)
}

#[derive(Debug, Clone)]
struct EnvelopeWidget {
    size: f32,
    delay: f32,
    attack: f32,
    attack_height: f32,
    hold: f32,
    decay: f32,
    sustain: f32,
    release: f32,
    release_height: f32,
}
impl EnvelopeWidget {
    fn new(
        delay: f32,
        attack: f32,
        attack_height: f32,
        hold: f32,
        decay: f32,
        sustain: f32,
        release: f32,
        release_height: f32,
    ) -> Self {
        Self {
            size: 5.0,
            delay,
            attack,
            hold,
            attack_height,
            decay,
            sustain,
            release,
            release_height,
        }
    }
}
impl canvas::Program<Message> for EnvelopeWidget {
    fn draw(&self, bounds: Rectangle, _cursor: canvas::Cursor) -> Vec<canvas::Geometry> {
        // We prepare a new `Frame`
        let mut frame = canvas::Frame::new(bounds.size());
        let y_offset = 2.0;
        let y_margin = 0.8;
        let x_offset = 2.0;
        let x_margin = 0.9;

        let sustain_length = 1.0;
        let total_length =
            self.delay + self.attack + self.hold + self.decay + self.release + sustain_length;
        let (
            delay_length,
            attack_length,
            hold_length,
            decay_length,
            release_length,
            sustain_length,
        ) = (
            self.delay / total_length,
            self.attack / total_length,
            self.hold / total_length,
            self.decay / total_length,
            self.release / total_length,
            sustain_length / total_length,
        );
        let (attack_height, sustain, release_height) = (
            self.attack_height * y_margin,
            self.sustain * y_margin,
            self.release_height * y_margin,
        );

        let bottom = y_offset + bounds.height * y_margin;
        let top = y_offset;
        let points = vec![
            Point::new(
                x_offset,
                y_offset + bounds.height * (y_margin - attack_height),
            ),
            Point::new(
                x_offset + bounds.width * delay_length * x_margin,
                y_offset + bounds.height * (y_margin - attack_height),
            ),
            Point::new(
                x_offset + bounds.width * (delay_length + attack_length) * x_margin,
                top,
            ),
            Point::new(
                x_offset + bounds.width * (delay_length + attack_length + hold_length) * x_margin,
                top,
            ),
            Point::new(
                x_offset
                    + bounds.width
                        * (delay_length + attack_length + hold_length + decay_length)
                        * x_margin,
                y_offset + bounds.height * (y_margin - sustain),
            ),
            Point::new(
                x_offset
                    + bounds.width
                        * (delay_length
                            + attack_length
                            + hold_length
                            + decay_length
                            + sustain_length)
                        * x_margin,
                y_offset + bounds.height * (y_margin - sustain),
            ),
            Point::new(
                x_offset
                    + bounds.width
                        * (delay_length
                            + attack_length
                            + hold_length
                            + decay_length
                            + sustain_length
                            + release_length)
                        * x_margin,
                y_offset + bounds.height * (y_margin - release_height),
            ),
        ];
        let mut time_marks = Vec::new();
        for i in 0..=((total_length / 0.5) as usize) {
            time_marks.push(x_offset + bounds.width * (i as f32 / (total_length / 0.5)) * x_margin);
        }
        let time_path = canvas::Path::new(|p| {
            for i in time_marks.iter() {
                p.move_to(Point::new(*i, top));
                p.line_to(Point::new(*i, bottom));
            }
        });
        let bg_path = canvas::Path::new(|p| {
            p.move_to(Point::new(
                points[4].x,
                y_offset + bounds.height * (y_margin - sustain),
            ));
            p.line_to(Point::new(points[4].x, bottom));
            p.move_to(Point::new(
                points[5].x,
                y_offset + bounds.height * (y_margin - sustain),
            ));
            p.line_to(Point::new(points[5].x, bottom));
        });
        let line_path = canvas::Path::new(|p| {
            // for i in points.windows(2) {
            //     p.move_to(i[0]);
            //     p.line_to(i[1]);
            // }
            p.move_to(points[0]);
            for i in points.iter() {
                p.line_to(i.clone());
            }
        });
        let fill_path = canvas::Path::new(|p| {
            // for i in points.windows(2) {
            //     p.move_to(i[0]);
            //     p.line_to(i[1]);
            // }
            p.move_to(points[0]);
            for i in points.iter() {
                p.line_to(i.clone());
            }
            p.line_to(Point::new(
                x_offset
                    + bounds.width
                        * (delay_length
                            + attack_length
                            + hold_length
                            + decay_length
                            + sustain_length
                            + release_length)
                        * x_margin,
                bottom,
            ));
            p.line_to(Point::new(x_offset, bottom));
            p.close();
        });

        frame.stroke(
            &time_path,
            canvas::Stroke::default().with_color(Color::from_rgba8(0, 0, 0, 0.3)),
        );
        frame.fill(
            &fill_path,
            Fill {
                color: Color::from_rgb8(230, 230, 230),
                rule: canvas::FillRule::NonZero,
            },
        );
        frame.stroke(
            &bg_path,
            canvas::Stroke::default().with_color(Color::from_rgb8(150, 150, 150)),
        );
        frame.stroke(&line_path, canvas::Stroke::default());

        for i in 0..7 {
            let radius = 2.0;
            let point = Point::new(points[i].x - radius, points[i].y - radius);
            frame.fill_rectangle(point, Size::new(radius * 2.0, radius * 2.0), Color::BLACK);
        }

        // Finally, we produce the geometry
        vec![frame.into_geometry()]
    }
}

#[derive(Debug, Clone)]
struct OscilloscopeWidget {
    oscillator: OscillatorBatch,
    osc_params: OscParamsBatch,
    osc_index: usize,
    border: bool,
    margin: f32,
}
impl OscilloscopeWidget {
    fn new(freq: f32, mut osc_params: OscParamsBatch, sample_rate: f32, osc_index: usize) -> Self {
        osc_params.sample_rate = f32x8::splat(sample_rate);
        let mut oscillator = OscillatorBatch::new(0, &osc_params, 1.0);
        oscillator.frequency = f32x8::splat(freq);
        oscillator.target_frequency = f32x8::splat(freq);
        oscillator.gain = f32x8::splat(1.0);
        Self {
            oscillator,
            osc_params,
            osc_index,
            border: false,
            margin: 0.1,
        }
    }
}
impl canvas::Program<Message> for OscilloscopeWidget {
    fn draw(&self, bounds: Rectangle, _cursor: canvas::Cursor) -> Vec<canvas::Geometry> {
        // We prepare a new `Frame`
        let mut frame = canvas::Frame::new(bounds.size());

        let mut oscillator = self.oscillator.clone();
        let mut points: Vec<Point> = (0..100)
            .map(|i| {
                let x = i as f32 / 100.0;
                let x = x * bounds.width;
                let y = (oscillator
                    .step(&self.osc_params, f32x8::splat(0.0))
                    .as_array_ref()[self.osc_index]
                    - 1.0)
                    .abs()
                    / 2.0;
                let y = y * (1.0 - self.margin) + self.margin * 0.5;
                let y = y * bounds.height;
                Point::new(x, y)
            })
            .collect();
        points.push(Point::new(100.0, points[0].y));
        let path = canvas::Path::new(|p| {
            p.move_to(points[0]);
            for i in points.iter() {
                p.line_to(i.clone());
            }
        });

        frame.stroke(&path, canvas::Stroke::default());
        if self.border {
            frame.stroke(
                &canvas::Path::rectangle(
                    Point::new(0.0, 0.0),
                    Size::new(bounds.width, bounds.height),
                ),
                canvas::Stroke::default(),
            );
        }

        // Finally, we produce the geometry
        vec![frame.into_geometry()]
    }
}

struct SynthPluginEditor {
    params: Arc<SynthPluginParams>,
    context: Arc<dyn GuiContext>,

    scrollable: widget::scrollable::State,

    filter_params: FilterWidget,
    global_envelope: GlobalEnvelopeWidget,
    global_params: GlobalParamWidget,

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

            scrollable: Default::default(),

            filter_params: Default::default(),
            global_envelope: Default::default(),
            global_params: Default::default(),

            osc_params_1: OscillatorWidget::new(0),
            osc_params_2: OscillatorWidget::new(1),
            osc_params_3: OscillatorWidget::new(2),
            osc_params_4: OscillatorWidget::new(3),
            osc_params_5: OscillatorWidget::new(4),
            osc_params_6: OscillatorWidget::new(5),
            osc_params_7: OscillatorWidget::new(6),
            osc_params_8: OscillatorWidget::new(7),

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
            .push(
                Row::new()
                    .padding(Padding::from(10))
                    .spacing(26)
                    .push(self.matrix.fm_matrix(&self.params))
                    .push(self.filter_params.ui(&self.params))
                    .push(self.global_envelope.ui(&self.params))
                    .push(self.global_params.ui(&self.params)),
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
    pub index: usize,
    pub name: String,
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
    pub waveshaper: param_slider::State,
    pub waveshaper_amount: param_slider::State,
    pub phaseshaper: param_slider::State,
    pub phaseshaper_amount: param_slider::State,
}

impl OscillatorWidget {
    fn new(index: usize) -> Self {
        Self {
            index,
            name: format!("Osc {}", index + 1),
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
            waveshaper: Default::default(),
            waveshaper_amount: Default::default(),
            phaseshaper: Default::default(),
            phaseshaper_amount: Default::default(),
        }
    }
    fn content<'a>(&'a mut self, osc_params: &'a OscillatorParams) -> Column<Message> {
        let param_font_size = 14;
        let slider_font_size = 14;
        let slider_width = 60;
        let slider_height = 14;
        let osc_env_spacing = 8;
        Column::new()
            .push(
                Row::new()
                    .push(
                        Text::new(&self.name)
                            .size(18)
                            .width(slider_width.into())
                            .font(assets::NOTO_SANS_BOLD),
                    )
                    .push(Space::with_width(osc_env_spacing.into()))
                    .push({
                        Canvas::new(EnvelopeWidget::new(
                            osc_params.delay.value(),
                            osc_params.attack.value(),
                            osc_params.attack_level.value(),
                            osc_params.hold.value(),
                            osc_params.decay.value(),
                            osc_params.sustain.value(),
                            osc_params.release.value(),
                            osc_params.release_level.value(),
                        ))
                        .height(18.into())
                        .width(slider_width.into())
                    })
                    .push(Space::with_width(osc_env_spacing.into()))
                    .push({
                        let mut params = [OscParams::default(); 8];
                        params[self.index] = osc_params.to_osc_params(100.0, 1.0, 0.0, 0);
                        Canvas::new(OscilloscopeWidget::new(
                            1.0,
                            params.into(),
                            100.0,
                            self.index,
                        ))
                        .height(18.into())
                        .width(slider_width.into())
                    }),
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
                    )
                    .push(Space::with_width(8.into()))
                    .push(
                        Column::new()
                            .push(Text::new("Waveshaper").size(param_font_size))
                            .push(
                                ParamSlider::new(&mut self.waveshaper, &osc_params.waveshaper)
                                    .width(slider_width.into())
                                    .height(slider_height.into())
                                    .text_size(slider_font_size)
                                    .map(Message::ParamUpdate),
                            )
                            .push(Text::new("Wshp. Amt.").size(param_font_size))
                            .push(
                                ParamSlider::new(
                                    &mut self.waveshaper_amount,
                                    &osc_params.waveshaper_amount,
                                )
                                .width(slider_width.into())
                                .height(slider_height.into())
                                .text_size(slider_font_size)
                                .map(Message::ParamUpdate),
                            )
                            .push(Text::new("Phaseshaper").size(param_font_size))
                            .push(
                                ParamSlider::new(&mut self.phaseshaper, &osc_params.phaseshaper)
                                    .width(slider_width.into())
                                    .height(slider_height.into())
                                    .text_size(slider_font_size)
                                    .map(Message::ParamUpdate),
                            )
                            .push(Text::new("Pshp. Amt.").size(param_font_size))
                            .push(
                                ParamSlider::new(
                                    &mut self.phaseshaper_amount,
                                    &osc_params.phaseshaper_amount,
                                )
                                .width(slider_width.into())
                                .height(slider_height.into())
                                .text_size(slider_font_size)
                                .map(Message::ParamUpdate),
                            ), // .push(Text::new("Keyscaling").size(param_font_size))
                               // .push(
                               //     ParamSlider::new(&mut self.keyscaling, &osc_params.keyscaling)
                               //         .width(slider_width.into())
                               //         .height(slider_height.into())
                               //         .text_size(slider_font_size)
                               //         .map(Message::ParamUpdate),
                               // )
                               // .push(Text::new("Rls. Level").size(param_font_size))
                               // .push(
                               //     ParamSlider::new(
                               //         &mut self.release_level,
                               //         &osc_params.release_level,
                               //     )
                               //     .width(slider_width.into())
                               //     .height(slider_height.into())
                               //     .text_size(slider_font_size)
                               //     .map(Message::ParamUpdate),
                               // ),
                    ),
            )
    }
}

fn title_bar<'a>() -> Column<'a, Message> {
    Column::new()
        .align_items(Alignment::Start)
        .push(
            Text::new("Foam FM Synth")
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
struct MatrixRow(
    param_slider::State,
    param_slider::State,
    param_slider::State,
    param_slider::State,
    param_slider::State,
    param_slider::State,
    param_slider::State,
    param_slider::State,
);
impl MatrixRow {
    pub fn to_ui<'a>(
        &'a mut self,
        label: String,
        spacing: u16,
        slider_width: u16,
        slider_height: u16,
        slider_font_size: u16,
        params: &'a OscMod,
    ) -> Row<Message> {
        Row::new()
            .spacing(spacing)
            .push(
                Text::new(label)
                    .size(14)
                    .width(slider_width.into())
                    .height(slider_height.into())
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .vertical_alignment(alignment::Vertical::Center),
            )
            .push(
                ParamSlider::new(&mut self.0, &params.by_osc1)
                    .width(slider_width.into())
                    .height(slider_height.into())
                    .text_size(slider_font_size)
                    .map(Message::ParamUpdate),
            )
            .push(
                ParamSlider::new(&mut self.1, &params.by_osc2)
                    .width(slider_width.into())
                    .height(slider_height.into())
                    .text_size(slider_font_size)
                    .map(Message::ParamUpdate),
            )
            .push(
                ParamSlider::new(&mut self.2, &params.by_osc3)
                    .width(slider_width.into())
                    .height(slider_height.into())
                    .text_size(slider_font_size)
                    .map(Message::ParamUpdate),
            )
            .push(
                ParamSlider::new(&mut self.3, &params.by_osc4)
                    .width(slider_width.into())
                    .height(slider_height.into())
                    .text_size(slider_font_size)
                    .map(Message::ParamUpdate),
            )
            .push(
                ParamSlider::new(&mut self.4, &params.by_osc5)
                    .width(slider_width.into())
                    .height(slider_height.into())
                    .text_size(slider_font_size)
                    .map(Message::ParamUpdate),
            )
            .push(
                ParamSlider::new(&mut self.5, &params.by_osc6)
                    .width(slider_width.into())
                    .height(slider_height.into())
                    .text_size(slider_font_size)
                    .map(Message::ParamUpdate),
            )
            .push(
                ParamSlider::new(&mut self.6, &params.by_osc7)
                    .width(slider_width.into())
                    .height(slider_height.into())
                    .text_size(slider_font_size)
                    .map(Message::ParamUpdate),
            )
            .push(
                ParamSlider::new(&mut self.7, &params.by_osc8)
                    .width(slider_width.into())
                    .height(slider_height.into())
                    .text_size(slider_font_size)
                    .map(Message::ParamUpdate),
            )
    }
}

#[derive(Debug, Default)]
struct MatrixWidget {
    _1: MatrixRow,
    _2: MatrixRow,
    _3: MatrixRow,
    _4: MatrixRow,
    _5: MatrixRow,
    _6: MatrixRow,
    _7: MatrixRow,
    _8: MatrixRow,

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
    fn fm_matrix<'a>(&'a mut self, params: &'a SynthPluginParams) -> Column<'a, Message> {
        let slider_width = 30;
        let slider_height = 14;
        let slider_font_size = 12;
        let spacing = 2;
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
            .push(self._1.to_ui(
                "To 1".to_string(),
                spacing,
                slider_width,
                slider_height,
                slider_font_size,
                &params.osc1_fm_mod,
            ))
            .push(self._2.to_ui(
                "To 2".to_string(),
                spacing,
                slider_width,
                slider_height,
                slider_font_size,
                &params.osc2_fm_mod,
            ))
            .push(self._3.to_ui(
                "To 3".to_string(),
                spacing,
                slider_width,
                slider_height,
                slider_font_size,
                &params.osc3_fm_mod,
            ))
            .push(self._4.to_ui(
                "To 4".to_string(),
                spacing,
                slider_width,
                slider_height,
                slider_font_size,
                &params.osc4_fm_mod,
            ))
            .push(self._5.to_ui(
                "To 5".to_string(),
                spacing,
                slider_width,
                slider_height,
                slider_font_size,
                &params.osc5_fm_mod,
            ))
            .push(self._6.to_ui(
                "To 6".to_string(),
                spacing,
                slider_width,
                slider_height,
                slider_font_size,
                &params.osc6_fm_mod,
            ))
            .push(self._7.to_ui(
                "To 7".to_string(),
                spacing,
                slider_width,
                slider_height,
                slider_font_size,
                &params.osc7_fm_mod,
            ))
            .push(self._8.to_ui(
                "To 8".to_string(),
                spacing,
                slider_width,
                slider_height,
                slider_font_size,
                &params.osc8_fm_mod,
            ))
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
            .push(
                Text::new("Filter")
                    .size(18)
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .font(assets::NOTO_SANS_BOLD),
            )
            .push(
                Row::new()
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
                            .push({
                                Canvas::new(EnvelopeWidget::new(
                                    0.0,
                                    params.global_attack.value(),
                                    0.0,
                                    0.0,
                                    params.global_decay.value(),
                                    params.global_sustain.value(),
                                    params.global_release.value(),
                                    0.0,
                                ))
                                .height(28.into())
                                .width(slider_width.into())
                            })
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

#[derive(Default)]
struct GlobalParamWidget {
    gain_slider: param_slider::State,
    global_coarse_slider: param_slider::State,
    octave_stretch_slider: param_slider::State,
    bend_range_slider: param_slider::State,
    voice_count_slider: param_slider::State,
    unison_slider: param_slider::State,
    unison_detune_slider: param_slider::State,
    legato_slider: param_slider::State,
    portamento_slider: param_slider::State,
}
impl GlobalParamWidget {
    fn ui<'a>(&'a mut self, params: &'a SynthPluginParams) -> Column<'a, Message> {
        let slider_height: Length = 14.into();
        let slider_width: Length = 60.into();
        let slider_font_size = 14;
        let font_size = 14;
        title_bar().push(Space::with_height(14.into())).push(
            Row::new()
                .spacing(8)
                .push(
                    Column::new()
                        .width(slider_width)
                        .push(Text::new("Output Gain").size(font_size))
                        .push(
                            ParamSlider::new(&mut self.gain_slider, &params.gain)
                                .height(slider_height)
                                .width(slider_width)
                                .text_size(slider_font_size)
                                .map(Message::ParamUpdate),
                        )
                        .push(Text::new("Glob. Crs.").size(font_size))
                        .push(
                            ParamSlider::new(&mut self.global_coarse_slider, &params.global_coarse)
                                .height(slider_height)
                                .width(slider_width)
                                .text_size(slider_font_size)
                                .map(Message::ParamUpdate),
                        )
                        .push(Text::new("Octave Size").size(font_size))
                        .push(
                            ParamSlider::new(
                                &mut self.octave_stretch_slider,
                                &params.octave_stretch,
                            )
                            .height(slider_height)
                            .width(slider_width)
                            .text_size(slider_font_size)
                            .map(Message::ParamUpdate),
                        )
                        .push(Text::new("Bend Range").size(font_size))
                        .push(
                            ParamSlider::new(&mut self.bend_range_slider, &params.bend_range)
                                .height(slider_height)
                                .width(slider_width)
                                .text_size(slider_font_size)
                                .map(Message::ParamUpdate),
                        ),
                )
                .push(
                    Column::new()
                        .align_items(Alignment::Start)
                        .width(slider_width)
                        .push(Text::new("Max Voices").size(font_size))
                        .push(
                            ParamSlider::new(&mut self.voice_count_slider, &params.voice_count)
                                .height(slider_height)
                                .width(slider_width)
                                .text_size(slider_font_size)
                                .map(Message::ParamUpdate),
                        )
                        .push(Text::new("Unison").size(font_size))
                        .push(
                            ParamSlider::new(&mut self.unison_slider, &params.unison_count)
                                .height(slider_height)
                                .width(slider_width)
                                .text_size(slider_font_size)
                                .map(Message::ParamUpdate),
                        )
                        .push(Text::new("Uni. Detune").size(font_size))
                        .push(
                            ParamSlider::new(&mut self.unison_detune_slider, &params.unison_detune)
                                .height(slider_height)
                                .width(slider_width)
                                .text_size(slider_font_size)
                                .map(Message::ParamUpdate),
                        )
                        .push(Text::new("Legato").size(font_size))
                        .push(
                            ParamSlider::new(&mut self.legato_slider, &params.legato)
                                .height(slider_height)
                                .width(slider_width)
                                .text_size(slider_font_size)
                                .map(Message::ParamUpdate),
                        ),
                )
                .push(
                    Column::new()
                        .align_items(Alignment::Start)
                        .width(slider_width)
                        .push(Text::new("Portamento").size(font_size))
                        .push(
                            ParamSlider::new(&mut self.portamento_slider, &params.portamento)
                                .height(slider_height)
                                .width(slider_width)
                                .text_size(slider_font_size)
                                .map(Message::ParamUpdate),
                        ),
                ),
        )
    }
}

pub mod interpolation {
    pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
        a.mul_add(1.0 - t, b * t)
    }

    pub fn catmull_rom(p: (f32, f32, f32, f32), t: f32) -> f32 {
        let a1 = -t * p.0 + (t + 1.0) * p.1;
        let a2 = (1.0 - t) * p.1 + t * p.2;
        let a3 = (2.0 - t) * p.2 + (t - 1.0) * p.3;
        let b1 = ((1.0 - t) * a1 + (t + 1.0) * a2) * 0.5;
        let b2 = ((2.0 - t) * a2 + t * a3) * 0.5;
        lerp(b1, b2, t)
    }

    // pub fn lanczos()

    mod test {
        use super::*;

        #[test]
        fn test_lerp() {
            assert_eq!(lerp(0.0, 1.0, 0.5), 0.5);
            assert_eq!(lerp(9.0, 1.0, 0.0), 9.0);
            assert_eq!(lerp(2.0, 4.0, 1.0), 4.0);
        }

        #[test]
        fn test_catmull_rom() {
            assert_eq!(catmull_rom((0.0, 1.0, 2.0, 3.0), 0.0), 1.0);
            assert_eq!(catmull_rom((0.0, 1.0, 2.0, 3.0), 0.5), 1.5);
            assert_eq!(catmull_rom((0.0, 1.0, 2.0, 3.0), 1.0), 2.0);
        }
    }
}

pub mod oscillators {
    use std::f32::consts::TAU;

    use enum_dispatch::enum_dispatch;

    use super::interpolation::{catmull_rom, lerp};

    #[enum_dispatch]
    pub trait Oscillate {
        fn render(&mut self, delta: f32, output: &mut [f32]);
    }

    #[derive(Default, Clone)]
    pub struct SineOscillator {
        phase: f32,
    }
    impl Oscillate for SineOscillator {
        #[no_mangle]
        fn render(&mut self, delta: f32, output: &mut [f32]) {
            for sample in output {
                self.phase += delta;
                if self.phase >= 1.0 {
                    self.phase -= 1.0;
                }
                *sample += (self.phase * TAU).sin();
            }
        }
    }

    #[derive(Default, Clone)]
    pub struct SineOscillator2 {
        phase: f32,
    }
    impl Oscillate for SineOscillator2 {
        fn render(&mut self, delta: f32, output: &mut [f32]) {
            output.iter_mut().for_each(|sample| {
                self.phase += delta;
                if self.phase >= 1.0 {
                    self.phase -= 1.0;
                }
                *sample += (self.phase * TAU).sin();
            })
        }
    }

    #[derive(Default, Clone)]
    pub struct SineOscillator3 {
        phase: f32,
    }
    impl Oscillate for SineOscillator3 {
        fn render(&mut self, delta: f32, output: &mut [f32]) {
            for sample in output {
                self.phase = (self.phase + delta) % 1.0;
                *sample += (self.phase * TAU).sin();
            }
        }
    }

    #[derive(Default, Clone)]
    pub struct SquareOscillator {
        phase: f32,
    }
    impl Oscillate for SquareOscillator {
        fn render(&mut self, delta: f32, output: &mut [f32]) {
            for sample in output {
                self.phase += delta;
                if self.phase >= 1.0 {
                    self.phase -= 1.0;
                }
                *sample += if self.phase > 0.5 { -1.0 } else { 1.0 };
            }
        }
    }

    #[derive(Default, Clone)]
    pub struct GbOscillator {
        phase: f32,
        samples: [f32; 16],
    }
    impl Oscillate for GbOscillator {
        fn render(&mut self, delta: f32, output: &mut [f32]) {
            for sample in output {
                self.phase += delta;
                if self.phase >= 1.0 {
                    self.phase -= 1.0;
                }
                *sample += self.samples[(self.phase * self.samples.len() as f32) as usize];
            }
        }
    }
    #[derive(Default, Clone)]
    pub struct GbOscillatorModulo {
        phase: f32,
        samples: [f32; 16],
    }
    impl Oscillate for GbOscillatorModulo {
        fn render(&mut self, delta: f32, output: &mut [f32]) {
            for sample in output {
                self.phase += delta;
                if self.phase >= 1.0 {
                    self.phase -= 1.0;
                }
                *sample += self.samples
                    [(self.phase * self.samples.len() as f32) as usize % self.samples.len()];
            }
        }
    }

    #[derive(Clone)]
    pub struct NaiveWaveOscillator {
        phase: f32,
        samples: Vec<f32>,
    }
    impl Oscillate for NaiveWaveOscillator {
        fn render(&mut self, delta: f32, output: &mut [f32]) {
            for sample in output {
                self.phase += delta;
                if self.phase >= 1.0 {
                    self.phase -= 1.0;
                }
                *sample += self.samples
                    [(self.phase * self.samples.len() as f32) as usize % self.samples.len()];
            }
        }
    }
    impl Default for NaiveWaveOscillator {
        fn default() -> Self {
            Self {
                phase: Default::default(),
                samples: vec![0.0; 2048],
            }
        }
    }

    #[derive(Clone)]
    pub struct LerpWaveOscillator {
        phase: f32,
        samples: Vec<f32>,
    }
    impl Oscillate for LerpWaveOscillator {
        fn render(&mut self, delta: f32, output: &mut [f32]) {
            for sample in output {
                self.phase += delta;
                if self.phase >= 1.0 {
                    self.phase -= 1.0;
                }
                let start = self.samples[(self.phase * self.samples.len() as f32) as usize];
                let end = self.samples
                    [((self.phase * self.samples.len() as f32) as usize + 1) % self.samples.len()];
                *sample += lerp(start, end, self.phase);
            }
        }
    }
    impl Default for LerpWaveOscillator {
        fn default() -> Self {
            Self {
                phase: Default::default(),
                samples: vec![0.0; 2048],
            }
        }
    }

    #[derive(Clone)]
    pub struct CatmullRomOscillator {
        phase: f32,
        samples: Vec<f32>,
    }
    impl Oscillate for CatmullRomOscillator {
        fn render(&mut self, delta: f32, output: &mut [f32]) {
            let length = self.samples.len();
            for sample in output {
                self.phase += delta;
                if self.phase >= 1.0 {
                    self.phase -= 1.0;
                }
                let p1 = self.samples[(self.phase * length as f32) as usize % length];
                let p2 = self.samples[((self.phase * length as f32) as usize + 1) % length];
                let p3 = self.samples[((self.phase * length as f32) as usize + 2) % length];
                let p0 =
                    self.samples[((self.phase * length as f32) as usize + (length - 1)) % length];
                *sample += catmull_rom((p0, p1, p2, p3), self.phase);
            }
        }
    }
    impl Default for CatmullRomOscillator {
        fn default() -> Self {
            Self {
                phase: Default::default(),
                samples: vec![0.0; 2048],
            }
        }
    }

    #[derive(Clone)]
    pub struct CatmullRomOscillator2 {
        phase: f32,
        samples: Vec<f32>,
    }
    impl Oscillate for CatmullRomOscillator2 {
        fn render(&mut self, delta: f32, output: &mut [f32]) {
            let length = 256;
            for sample in output {
                self.phase += delta;
                if self.phase >= 1.0 {
                    self.phase -= 1.0;
                }
                let p1 = self.samples[(self.phase * length as f32) as usize % length];
                let p2 = self.samples[((self.phase * length as f32) as usize + 1) % length];
                let p3 = self.samples[((self.phase * length as f32) as usize + 2) % length];
                let p0 =
                    self.samples[((self.phase * length as f32) as usize + (length - 1)) % length];
                *sample += catmull_rom((p0, p1, p2, p3), self.phase);
            }
        }
    }
    impl Default for CatmullRomOscillator2 {
        fn default() -> Self {
            Self {
                phase: Default::default(),
                samples: vec![0.0; 256],
            }
        }
    }

    #[derive(Clone)]
    pub struct CatmullRomOscillator3 {
        phase: f32,
        samples: Vec<f32>,
    }
    impl Oscillate for CatmullRomOscillator3 {
        fn render(&mut self, delta: f32, output: &mut [f32]) {
            let length = self.samples.len();
            assert!(length >= 1);
            for sample in output {
                self.phase += delta;
                if self.phase >= 1.0 {
                    self.phase -= 1.0;
                }
                let p1 = *self
                    .samples
                    .iter()
                    .cycle()
                    .nth((self.phase * length as f32) as usize)
                    .unwrap_or_else(|| unreachable!());
                let p2 = *self
                    .samples
                    .iter()
                    .cycle()
                    .nth((self.phase * length as f32) as usize + 1)
                    .unwrap_or_else(|| unreachable!());
                let p3 = *self
                    .samples
                    .iter()
                    .cycle()
                    .nth((self.phase * length as f32) as usize + 2)
                    .unwrap_or_else(|| unreachable!());
                let p0 = *self
                    .samples
                    .iter()
                    .cycle()
                    .nth((self.phase * length as f32) as usize + (length - 1))
                    .unwrap_or_else(|| unreachable!());
                *sample += catmull_rom((p0, p1, p2, p3), self.phase);
            }
        }
    }
    impl Default for CatmullRomOscillator3 {
        fn default() -> Self {
            Self {
                phase: Default::default(),
                samples: vec![0.0; 2048],
            }
        }
    }

    #[derive(Clone)]
    pub struct ParamSmoother {
        current: f32,
        target: f32,
        rate: f32,
    }
    impl Oscillate for ParamSmoother {
        fn render(&mut self, _delta: f32, output: &mut [f32]) {
            for sample in output {
                *sample = self.current;
                self.current = lerp(self.current, self.target, self.rate)
            }
            self.target *= 2.0;
        }
    }
    impl Default for ParamSmoother {
        fn default() -> Self {
            Self {
                current: Default::default(),
                target: 1.0,
                rate: 0.15 * 44100.0 / 44100.0,
            }
        }
    }

    #[derive(Clone)]
    pub struct SurgeQuadrOsc {
        r: f32,
        i: f32,
        dr: f32,
        di: f32,
    }
    impl Oscillate for SurgeQuadrOsc {
        fn render(&mut self, delta: f32, output: &mut [f32]) {
            self.set_rate(delta);
            for sample in output {
                let (lr, li) = (self.r, self.i);
                self.r = self.dr * lr - self.di * li;
                self.i = self.dr * li - self.di * lr;
                *sample = self.r;
            }
        }
    }
    impl SurgeQuadrOsc {
        fn set_rate(&mut self, w: f32) {
            self.dr = w.cos();
            self.di = w.sin();
            let n = 1.0 / (self.r.powi(2) + self.i.powi(2)).sqrt();
            self.r *= n;
            self.i *= n;
        }
    }
    impl Default for SurgeQuadrOsc {
        fn default() -> Self {
            Self {
                r: 0.0,
                i: -1.0,
                dr: 0.0,
                di: 0.0,
            }
        }
    }

    #[enum_dispatch(Oscillate)]
    #[derive(Clone)]
    pub enum MetaOscillator {
        Sine(SineOscillator2),
        Square(SquareOscillator),
        NaiveWave(NaiveWaveOscillator),
        LerpWave(LerpWaveOscillator),
        CatmullRom(CatmullRomOscillator2),
    }

    mod test {
        use std::time::Instant;

        use super::*;
        fn benchmark(mut osc: impl Oscillate) -> f32 {
            let mut buffer = [0.0f32; 44100 * 5];
            let pitches: Vec<f32> = (0..(32 * 8))
                .map(|x| ((x as f32) * 40.0 + 100.0) / 44100.0)
                .collect();
            let now = Instant::now();
            for pitch in pitches {
                osc.render(pitch, &mut buffer);
            }
            let time = now.elapsed().as_secs_f32();
            // println!("took {time} to render {} samples", buffer.len());
            time
        }
        fn benchmark_samples(mut osc: impl Oscillate) -> f32 {
            let mut buffer = [0.0f32; 44100 * 5];
            let block_size = 32;
            let pitches: Vec<f32> = (0..(32 * 8))
                .map(|x| ((x as f32) * 40.0 + 100.0) / 44100.0)
                .collect();
            let now = Instant::now();
            for pitch in pitches {
                for samples in buffer.chunks_exact_mut(block_size) {
                    osc.render(pitch, samples);
                }
            }
            let time = now.elapsed().as_secs_f32();
            // println!("took {time} to render {} samples", buffer.len());
            time
        }
        fn multi_benchmark(osc: impl Oscillate + Clone) -> f32 {
            let mut samples: Vec<f32> = Vec::with_capacity(10);
            for _ in 0..5 {
                samples.push(benchmark_samples(osc.clone()));
            }
            let len = samples.len() as f32;
            samples.into_iter().sum::<f32>() / len
        }
        #[test]
        fn benchmark_all() {
            // let osc1 = SineOscillator::default();
            // println!("SineOscillator took {} to render", multi_benchmark(osc1));
            let osc2 = SineOscillator2::default();
            println!("SineOscillator2 took {} to render", multi_benchmark(osc2));
            // let osc3 = SineOscillator3::default();
            // println!("SineOscillator3 took {} to render", multi_benchmark(osc3));
            // let osc = SquareOscillator::default();
            // println!("SquareOscillator took {} to render", multi_benchmark(osc));
            // let osc = GbOscillator::default();
            // println!("GbOscillator took {} to render", multi_benchmark(osc));
            // let osc = GbOscillatorModulo::default();
            // println!("GbOscillatorModulo took {} to render", multi_benchmark(osc));
            let osc = NaiveWaveOscillator::default();
            println!(
                "NaiveWaveOscillator took {} to render",
                multi_benchmark(osc)
            );
            let osc = LerpWaveOscillator::default();
            println!("LerpWaveOscillator took {} to render", multi_benchmark(osc));
            let osc = CatmullRomOscillator::default();
            println!(
                "CatmullRomOscillator took {} to render",
                multi_benchmark(osc)
            );
            let osc = CatmullRomOscillator2::default();
            println!(
                "CatmullRomOscillator2 took {} to render",
                multi_benchmark(osc)
            );
            let osc = CatmullRomOscillator3::default();
            println!(
                "CatmullRomOscillator3 took {} to render",
                multi_benchmark(osc)
            );
            let osc = ParamSmoother::default();
            println!("ParamSmoother took {} to render", multi_benchmark(osc));
            let osc = SurgeQuadrOsc::default();
            println!("SurgeQuadrOsc took {} to render", multi_benchmark(osc));
            let mut osc = MetaOscillator::CatmullRom(CatmullRomOscillator2::default());
            println!("MetaOscillator::CatmullRom took {} to render", multi_benchmark(osc));
            osc = MetaOscillator::Square(SquareOscillator::default());
            println!("MetaOscillator::Square took {} to render", multi_benchmark(osc));
        }
    }
}

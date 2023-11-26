pub mod interpolation {
    use wide::f32x8;

    pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
        a.mul_add(1.0 - t, b * t)
    }

    pub fn lerpx8(a: f32x8, b: f32x8, t: f32x8) -> f32x8 {
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

    pub fn hermite(p: (f32, f32, f32, f32), t: f32) -> f32 {
        let slope = (
            (p.2 - p.0) * 0.5,
            (p.3 - p.1) * 0.5,
        );
        let v = p.1 - p.2;
        let w = slope.0 + v;
        let a = w + v + slope.1;
        let b_neg = w + a;
        let stage1 = a * t - b_neg;
        let stage2 = stage1 * t + slope.0;
        stage2 * t + p.1
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
    use itertools::izip;

    use super::interpolation::{catmull_rom, lerp};

    #[enum_dispatch]
    pub trait Oscillate {
        fn render(&mut self, delta: f32, output: &mut [f32]);
    }

    pub trait ParamOscillate {
        fn render(&mut self, params: [&[f32]; 16], output: &mut [f32]);
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
    pub struct SineOscillator4 {
        phase: u32,
    }
    impl Oscillate for SineOscillator4 {
        fn render(&mut self, delta: f32, output: &mut [f32]) {
            for sample in output {
                let bit_depth = 24;
                let phase_inc = (delta * (2 << (bit_depth)) as f32) as u32;
                self.phase = self.phase.wrapping_add(phase_inc) % (2 << bit_depth);
                let index = ((self.phase >> (bit_depth - 10)) % 2048) as f32;
                *sample += ((index / 2048.0) * TAU).sin();
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
    pub struct NaiveWaveOscillator2 {
        phase: u32,
        samples: Vec<f32>,
    }
    impl Oscillate for NaiveWaveOscillator2 {
        fn render(&mut self, delta: f32, output: &mut [f32]) {
            let bit_depth = 24;
            for sample in output {
                let phase_inc = (delta * (2 << (bit_depth)) as f32) as u32;
                self.phase = self.phase.wrapping_add(phase_inc) % (2 << bit_depth);
                let index = ((self.phase >> (bit_depth - 10)) % 2048) as usize;
                *sample += self.samples
                    [index];
            }
        }
    }
    impl Default for NaiveWaveOscillator2 {
        fn default() -> Self {
            let samples = (0..2048).into_iter().map(|i| {
                let x = (i as f32) / 2048.0;
                (x * TAU).sin()
            }).collect();
            Self {
                phase: Default::default(),
                samples,
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
        NaiveWave(NaiveWaveOscillator2),
        LerpWave(LerpWaveOscillator),
        CatmullRom(CatmullRomOscillator2),
    }

    #[derive(Clone)]
    pub struct PitchGen;
    impl ParamOscillate for PitchGen {
        fn render(&mut self, params: [&[f32]; 16], output: &mut [f32]) {
            for (sample, params) in output.into_iter().zip(izip!(params[0], params[1], params[2], params[3])) {
                *sample += 2.0f32.powf(
                    (params.0 + params.1 + params.2 / 100.0 - 69.0)
                        / (12.0 / params.3),
                ) * 440.0;
            }
        }
    }

    #[derive(Default, Clone)]
    pub struct SineOscillatorParams {
        phase: f32,
    }
    impl ParamOscillate for SineOscillatorParams {
        fn render(&mut self, params: [&[f32]; 16], output: &mut [f32]) {
            for (sample, params) in output.into_iter().zip(izip!(params[0], params[1], params[2], params[3])) {
                let delta = 2.0f32.powf(
                    (params.0 + params.1 + params.2 / 100.0 - 69.0)
                        / (12.0 / params.3),
                ) * 440.0;
                self.phase += delta;
                if self.phase >= 1.0 {
                    self.phase -= 1.0;
                }
                *sample += (self.phase * TAU).sin();
            }
        }
    }

    #[derive(Default, Clone)]
    pub struct SineOscillatorParams2 {
        phase: f32,
    }
    impl ParamOscillate for SineOscillatorParams2 {
        fn render(&mut self, params: [&[f32]; 16], output: &mut [f32]) {
            let delta = 2.0f32.powf(
                (params[0][0] + params[0][1] + params[1][2] / 100.0 - 69.0)
                    / (12.0 / params[1][3]),
            ) * 440.0;
            for sample in output.into_iter() {
                self.phase += delta;
                if self.phase >= 1.0 {
                    self.phase -= 1.0;
                }
                *sample += (self.phase * TAU).sin();
            }
        }
    }

    #[derive(Clone)]
    pub struct Pow2Table {
        data: Vec<f32>
    }
    impl Pow2Table {
        pub fn new() -> Self {
            let len = 2usize.pow(16);
            let mut data = Vec::with_capacity(len);
            for i in 0..len {
                data.push(lerp(-1.0, 1.0, i as f32 / len as f32));
            }
            Self {
                data
            }
        }
        pub fn look_up(&self, x: f32) -> f32 {
            0.0
        }
    }
    impl Default for Pow2Table {
        fn default() -> Self {
        Self::new()
    }
    }

    #[derive(Default, Clone)]
    pub struct SineOscillatorParams3 {
        phase: f32,
        table: Pow2Table,
    }
    impl ParamOscillate for SineOscillatorParams3 {
        fn render(&mut self, params: [&[f32]; 16], output: &mut [f32]) {
            let delta = 2.0f32.powf(
                (params[0][0] + params[0][1] + params[1][2] / 100.0 - 69.0)
                    / (12.0 / params[1][3]),
            ) * 440.0;
            for sample in output.into_iter() {
                self.phase += delta;
                if self.phase >= 1.0 {
                    self.phase -= 1.0;
                }
                *sample += (self.phase * TAU).sin();
            }
        }
    }

    #[derive(Default, Clone)]
    pub struct PmOscillatorParams {
        phase: f32,
        pm_depth: f32,
    }
    impl ParamOscillate for PmOscillatorParams {
        fn render(&mut self, params: [&[f32]; 16], output: &mut [f32]) {
            let delta = 2.0f32.powf(
                (params[0][0] + params[0][1] + params[1][2] / 100.0 - 69.0)
                    / (12.0 / params[1][3]),
            ) * 440.0;
            for (sample, pm) in output.into_iter().zip(params[1]) {
                self.phase += delta;
                if self.phase >= 1.0 {
                    self.phase -= 1.0;
                }
                *sample += (self.phase * TAU + pm * self.pm_depth).sin();
            }
        }
    }

    mod test {
        use std::time::Instant;
        fn ramp_up(mult: f32) -> Vec<f32> {
            (0..44100*5).map(|x| x as f32 / 44100.0 * mult).collect()
        }
        fn ramp_down(mult: f32) -> Vec<f32> {
            (0..44100*5).map(|x| x as f32 / 44100.0 * mult).collect()
        }

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
        fn benchmark_samples(mut osc: impl Oscillate) -> (f32, Vec<f32>) {
            let mut buffer = [0.0f32; 44100 * 5];
            let block_size = 32;
            // let pitches: Vec<f32> = (0..(32 * 8))
            //     .map(|x| ((x as f32) * 10.0 + 40.0) / 44100.0)
            //     .collect();
            let pitches: Vec<f32> = vec![200.0 / 44100.0, (200.0 * 5.0/2.0) / 44100.0, 300.0 / 44100.0];
            let now = Instant::now();
            for pitch in pitches {
                for samples in buffer.chunks_exact_mut(block_size) {
                    osc.render(pitch, samples);
                }
            }
            for sample in buffer.iter_mut() {
                *sample *= (1.0 / (32.0 * 8.0));
            }
            let time = now.elapsed().as_secs_f32();
            // println!("took {time} to render {} samples", buffer.len());
            (time, Vec::from(buffer))
        }
        fn multi_benchmark(osc: impl Oscillate + Clone) -> (f32, Vec<f32>) {
            let mut samples: Vec<f32> = Vec::with_capacity(10);
            let mut audio: Vec<Vec<f32>> = Vec::with_capacity(10);
            for _ in 0..5 {
                let (time, sample) = benchmark_samples(osc.clone());
                samples.push(time);
                audio.push(sample);
            }
            let len = samples.len() as f32;
            (samples.into_iter().sum::<f32>() / len, audio.into_iter().flatten().collect())
        }
        fn benchmark_samples_params(mut osc: impl ParamOscillate) -> f32 {
            let mut buffer = [0.0f32; 44100 * 5];
            let vectors = [
                ramp_up(8.0),
                ramp_down(4.0),
                ramp_up(8.0),
                ramp_down(4.0),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
            ];
            let params: [&[f32]; 16] = [
                &vectors[0], &vectors[1], &vectors[2], &vectors[3], &vectors[4], &vectors[5], &vectors[6], &vectors[7], &vectors[8], &vectors[9], &vectors[10], &vectors[11], &vectors[12], &vectors[13], &vectors[14], &vectors[15]
            ];
            let now = Instant::now();
            for _ in 0..(32 * 8) {
                osc.render(params.clone(), &mut buffer);
            }
            let time = now.elapsed().as_secs_f32();
            // let vec2 = Vec::from(buffer);
            // println!("took {time} to render {} samples", vec2.len());
            time
        }
        fn multi_benchmark_params(osc: impl ParamOscillate + Clone) -> f32 {
            let mut samples: Vec<f32> = Vec::with_capacity(10);
            for _ in 0..5 {
                samples.push(benchmark_samples_params(osc.clone()));
            }
            let len = samples.len() as f32;
            samples.into_iter().sum::<f32>() / len
        }
        #[test]
        fn benchmark_all() {
            fn multi_benchmark_and_file(osc: impl Oscillate + Clone, name: &str) -> Result<f32, ()> {
                use hound;
    
                let spec = hound::WavSpec {
                    channels: 2,
                    sample_rate: 44100,
                    bits_per_sample: 32,
                    sample_format: hound::SampleFormat::Float,
                };
                let (time, audio) = multi_benchmark(osc);
                let mut writer = hound::WavWriter::create(format!("target/{name}.wav"), spec).unwrap();
                for x in audio {
                    if let Err(_) = writer.write_sample(x) {
                        return Err(())
                    }
                    if let Err(_) = writer.write_sample(x) {
                        return Err(())
                    }
                }
                Ok(time)
            }
            // let osc1 = SineOscillator::default();
            // println!("SineOscillator took {} to render", multi_benchmark(osc1));
            let osc2 = SineOscillator2::default();
            let name = "SineOscillator2";
            println!("{name} took {} to render", multi_benchmark_and_file(osc2, name).unwrap());
            // let osc3 = SineOscillator3::default();
            // println!("SineOscillator3 took {} to render", multi_benchmark(osc3));
            // let osc = SquareOscillator::default();
            let osc = SineOscillator4::default();
            let name = "SineOscillator4";
            println!("{name} took {} to render", multi_benchmark_and_file(osc, name).unwrap());
            // println!("SquareOscillator took {} to render", multi_benchmark(osc));
            // let osc = GbOscillator::default();
            // println!("GbOscillator took {} to render", multi_benchmark(osc));
            // let osc = GbOscillatorModulo::default();
            // println!("GbOscillatorModulo took {} to render", multi_benchmark(osc));
            let osc = NaiveWaveOscillator::default();
            let name = "NaiveWaveOscillator";
            println!("{name} took {} to render", multi_benchmark_and_file(osc, name).unwrap());

            let osc = NaiveWaveOscillator2::default();
            let name = "NaiveWaveOscillator2";
            println!("{name} took {} to render", multi_benchmark_and_file(osc, name).unwrap());

            let osc = LerpWaveOscillator::default();
            let name = "LerpWaveOscillator";
            println!("{name} took {} to render", multi_benchmark_and_file(osc, name).unwrap());

            let osc = CatmullRomOscillator::default();
            let name = "CatmullRomOscillator";
            println!("{name} took {} to render", multi_benchmark_and_file(osc, name).unwrap());

            let osc = CatmullRomOscillator2::default();
            let name = "CatmullRomOscillator2";
            println!("{name} took {} to render", multi_benchmark_and_file(osc, name).unwrap());

            let osc = CatmullRomOscillator3::default();
            let name = "CatmullRomOscillator2";
            println!("{name} took {} to render", multi_benchmark_and_file(osc, name).unwrap());

            let osc = ParamSmoother::default();
            println!("ParamSmoother took {} to render", multi_benchmark(osc).0);
            let osc = SurgeQuadrOsc::default();
            let name = "SurgeQuadrOsc";
            println!("{name} took {} to render", multi_benchmark_and_file(osc, name).unwrap());
            let mut osc = MetaOscillator::CatmullRom(CatmullRomOscillator2::default());
            let name = "MetaOscillator::CatmullRom";
            println!("{name} took {} to render", multi_benchmark_and_file(osc, name).unwrap());
            osc = MetaOscillator::Square(SquareOscillator::default());
            let name = "MetaOscillator::Square";
            println!("{name} took {} to render", multi_benchmark_and_file(osc, name).unwrap());
        }
        #[test]
        fn benchmark_params() {
            let ramp = ramp_up(8.0);
            let ramp_down = ramp_down(8.0);
            let osc1 = SineOscillator::default();
            println!("SineOscillator took {} to render", multi_benchmark(osc1).0);
            let osc2 = SineOscillator2::default();
            println!("SineOscillator2 took {} to render", multi_benchmark(osc2).0);
            let osc = PitchGen;
            println!("PitchGen took {} to render", multi_benchmark_params(osc));
            let osc = SineOscillatorParams::default();
            println!("SineOscillatorParams took {} to render", multi_benchmark_params(osc));
            let osc = SineOscillatorParams2::default();
            println!("SineOscillatorParams2 took {} to render", multi_benchmark_params(osc));
            let osc = PmOscillatorParams::default();
            println!("PmOscillatorParams took {} to render", multi_benchmark_params(osc));
        }
    }
}

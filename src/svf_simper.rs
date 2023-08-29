// Original from: https://github.com/wrl/baseplug, under
// the MIT license:

// Permission is hereby granted, free of charge, to any
// person obtaining a copy of this software and associated
// documentation files (the "Software"), to deal in the
// Software without restriction, including without
// limitation the rights to use, copy, modify, merge,
// publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software
// is furnished to do so, subject to the following
// conditions:

// The above copyright notice and this permission notice
// shall be included in all copies or substantial portions
// of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
// ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
// TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
// PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
// SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
// CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
// IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

// Modifications from the original:
// Removed simd acceleration.
// 

// implemented from https://cytomic.com/files/dsp/SvfLinearTrapOptimised2.pdf
// thanks, andy!

use std::f32::consts;

use nih_plug::prelude::Enum;

#[derive(Debug, Clone, Copy, Enum, PartialEq)]
pub enum FilterType {
    Lowpass,
    Bandpass,
    Highpass,
}

#[derive(Debug, Clone, Copy)]
pub struct SvfSimper {
    pub a1: f32,
    pub a2: f32,
    pub a3: f32,

    pub ic1eq: f32,
    pub ic2eq: f32,

    k: f32,

    pub filter_type: FilterType,
}

impl SvfSimper {
    pub fn new(cutoff: f32, resonance: f32, sample_rate: f32) -> Self {
        let g = (consts::PI * (cutoff / sample_rate)).tan();
        let k = 2f32 - (1.9f32 * resonance.clamp(0.0, 1.0));

        let a1 = 1.0 / (1.0 + (g * (g + k)));
        let a2 = g * a1;
        let a3 = g * a2;

        SvfSimper {
            a1,
            a2,
            a3,

            ic1eq: 0.0,
            ic2eq: 0.0,

            k,

            filter_type: FilterType::Lowpass,
        }
    }

    pub fn set(&mut self, cutoff: f32, resonance: f32, sample_rate: f32) {
        let new = Self::new(cutoff, resonance, sample_rate);

        self.a1 = new.a1;
        self.a2 = new.a2;
        self.a3 = new.a3;
    }
    #[inline]
    pub fn process(&mut self, input: f32) -> f32 {
        let v3 = input - self.ic2eq;
        let v1 = (self.a1 * self.ic1eq) + (self.a2 * v3);
        let v2 = self.ic2eq + (self.a2 * self.ic1eq) + (self.a3 * v3);

        self.ic1eq = (2.0 * v1) - self.ic1eq;
        self.ic2eq = (2.0 * v2) - self.ic2eq;

        match self.filter_type {
            FilterType::Lowpass => v2,
            FilterType::Bandpass => v1,
            FilterType::Highpass => input - self.k * v1 - v2,
            // notch: v0 - self.k * v1
            // peak: 2.0 * v2 - v0 + self.k * v1
            // all: v0 - 2.0 * self.k * v1
        }
    }

    pub fn set_params(&mut self, sample_rate: f32, cutoff: f32, resonance: f32) {
        self.set(cutoff, resonance / 10.0, sample_rate)
    }

    pub fn set_filter_type(&mut self, filter_type: FilterType) {
        self.filter_type = filter_type;
    }
}
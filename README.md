# Foam
![Screenshot of UI](nogui.png)

6 operator FM synth, available in the VST3 or CLAP plugin formats.

Open source under GPLv3.

Currently has no GUI, relies on the host's default controls for VST/CLAP plugins. In development, control ranges and such may change between versions, potentially breaking your presets/saved projects on update.

## What

There are 6 feedback-capable sine oscillators, each with independent amplitude envelopes. Each of the 6 oscillators has the following controls:
* **Amp:** Direct output volume (after modulation, doesn't affect modulation)
* **Feedback:** Oscillator tone/self-PM. Positive values produce a saw wave, negative values produce a square wave.
* **Frequency Controls:**
    * **Coarse:** Frequency shift in semitones
    * **Fine:** Frequency shift in cents
    * **Frequency Multiply:** Multiplies frequency by an integer value from 1-64
    * **Frequency Divide:** Divides frequency by an integer value from 1-64
* **Envelope controls: (affects modulation)**
    * **Attack:** Envelope attack time. How long it takes to reach maximum volume after note start.
    * **Decay:** Envelope decay time. How long it takes for the volume to descend to the sustain level after attack time.
    * **Sustain:** Envelope sustain level. Sustained volume level after decay stage but before note release.
    * **Release:** Envelope release time. Time note continues playing after note release.
    * Note: The decay and release stages are exponential, not linear. 
* **Misc. Controls**
    * **Velocity Sensitivity:** How much MIDI velocity affects oscillator volume.
    * **Keyscaling:** How oscillator volume decreases/increases as pitch rises/falls.

The oscillators modulate each other via a 6x5 matrix, where every oscillator is connected to every other one (excluding itself, since feedback is a separate control with a greater range). It is possible to create cross-oscillator feedback loops (e.g. Osc1 and Osc2 both modulate each other) but they don't typically sound that good (not that I'm stopping you). The matrix is implemented by enforcing a 1-sample delay between oscillators.

There is also a multimode filter (Simper SVF) on each voice, which can be controlled via an ADSR envelope.

Technically this is a PM synth, but the terms PM/FM are often used interchangeably. If you want a true FM synth, try setting it up in [Cardinal](https://github.com/DISTRHO/Cardinal).

## Building

After installing [Rust](https://rustup.rs/), you can compile Foam as follows:

```shell
cargo xtask bundle foam --release
```

## Credits
Thanks to [Surge XT](https://github.com/surge-synthesizer/surge) (and its developers) for the +/- FM feedback algorithm.

Thanks to [NIH-plug](https://github.com/robbert-vdh/nih-plug) for being a great plugin framework.

Thanks to [Codeium](https://codeium.com/) for filling out a lot of the boilerplate which would have been a pain to do manually (mostly plugin parameters).

## TODO:

* Add GUI
* Add panning
* Add envelope Delay and Hold
* Envelope slope controls?
* Precalculate table for tuning approximation?
* Add pitch bend control
* Add phase control
* Add LFO and pitch envelope
* Make filter envelope independent of host block size
* Add global volume envelope
* Make filter keytracking actually work
* Allow use of the filter in FM modulation
* Scala support?
* Optimize
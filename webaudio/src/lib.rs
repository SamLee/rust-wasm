use wasm_bindgen::prelude::*;
use web_sys::{AudioContext, OscillatorType};

pub fn midi_to_frequency(midi_note: u8) -> f32 {
    2.0_f32.powf((midi_note as f32 - 69.0) / 12.0) * 440.0
}

#[wasm_bindgen]
pub struct Synth {
    context: AudioContext,
    osc: web_sys::OscillatorNode,
    gain: web_sys::GainNode,
    fm_osc: web_sys::OscillatorNode,
    fm_gain: web_sys::GainNode,
    fm_frequency_ratio: f32,
    fm_gain_ratio: f32,
}

impl Drop for Synth {
    fn drop(&mut self) {
        let _ = self.context.close();
    }
}

#[wasm_bindgen]
impl Synth {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<Synth, JsValue> {
        let context = web_sys::AudioContext::new()?;
        let osc = context.create_oscillator()?;
        let gain = context.create_gain()?;
        let fm_osc = context.create_oscillator()?;
        let fm_gain = context.create_gain()?;

        osc.set_type(OscillatorType::Sine);
        osc.frequency().set_value(440.0);
        gain.gain().set_value(0.0);
        fm_osc.set_type(OscillatorType::Sine);
        fm_osc.frequency().set_value(0.0);
        fm_gain.gain().set_value(0.0);

        osc.connect_with_audio_node(&gain)?;
        gain.connect_with_audio_node(&context.destination())?;
        fm_osc.connect_with_audio_node(&fm_gain)?;
        fm_gain.connect_with_audio_param(&osc.frequency())?;

        osc.start()?;
        fm_osc.start()?;

        return Ok(Synth { 
            context,
            osc,
            gain,
            fm_osc,
            fm_gain,
            fm_frequency_ratio: 0.0 ,
            fm_gain_ratio: 0.0,
        });
    }

    #[wasm_bindgen]
    pub fn set_frequency(&self, frequency: f32) {
        self.osc.frequency().set_value(frequency);
        self.fm_osc.frequency().set_value(self.fm_frequency_ratio * frequency);
        self.fm_gain.gain().set_value(self.fm_gain_ratio * frequency);
    }

    #[wasm_bindgen]
    pub fn set_gain(&self, mut gain: f32) {
        if gain > 1.0 { gain = 1.0 }
        if gain < 0.0 { gain = 0.0 }
        self.gain.gain().set_value(gain);
    }

    #[wasm_bindgen]
    pub fn set_note(&self, note: u8) {
        let frequency = midi_to_frequency(note);
        self.set_frequency(frequency);
    }

    #[wasm_bindgen]
    pub fn set_fm_frequency(&mut self, ratio: f32) {
        self.fm_frequency_ratio = ratio;
        self.fm_osc.frequency().set_value(self.fm_frequency_ratio * self.osc.frequency().value());
    }

    #[wasm_bindgen]
    pub fn set_fm_gain(&mut self, gain: f32) {
        self.fm_gain_ratio = gain;
        self.fm_gain.gain().set_value(self.fm_gain_ratio * self.osc.frequency().value());
    }

    #[wasm_bindgen]
    pub fn set_waveform(&self, wave: &str) {
        let waveform = match wave {
            "sine" => OscillatorType::Sine,
            "square" => OscillatorType::Square,
            "sawtooth" => OscillatorType::Sawtooth,
            "triangle" => OscillatorType::Triangle,
            _ => OscillatorType::Sine,
        };
        self.osc.set_type(waveform);
    }

    #[wasm_bindgen]
    pub fn set_fm_waveform(&self, wave: &str) {
        let waveform = match wave {
            "sine" => OscillatorType::Sine,
            "square" => OscillatorType::Square,
            "sawtooth" => OscillatorType::Sawtooth,
            "triangle" => OscillatorType::Triangle,
            _ => OscillatorType::Sine,
        };
        self.fm_osc.set_type(waveform);
    }

}

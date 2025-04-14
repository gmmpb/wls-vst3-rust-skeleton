use atomic_float::AtomicF32;
use nih_plug::prelude::*;
use std::sync::Arc;

use crate::dsp::parameters::{GainParams, PEAK_METER_DECAY_MS};

/// Handles the audio processing logic
pub fn process_samples(
    buffer: &mut Buffer,
    params: &Arc<GainParams>,
    peak_meter: &Arc<AtomicF32>,
    peak_meter_decay_weight: f32,
) -> ProcessStatus {
    for channel_samples in buffer.iter_samples() {
        let mut amplitude = 0.0;
        let num_samples = channel_samples.len();

        let gain = params.gain.smoothed.next();
        for sample in channel_samples {
            *sample *= gain;
            amplitude += *sample;
        }

        // To save resources, only perform peak meter calculations when GUI is open
        if params.editor_state.is_open() {
            amplitude = (amplitude / num_samples as f32).abs();
            let current_peak_meter = peak_meter.load(std::sync::atomic::Ordering::Relaxed);
            let new_peak_meter = if amplitude > current_peak_meter {
                amplitude
            } else {
                current_peak_meter * peak_meter_decay_weight
                    + amplitude * (1.0 - peak_meter_decay_weight)
            };

            peak_meter
                .store(new_peak_meter, std::sync::atomic::Ordering::Relaxed)
        }
    }

    ProcessStatus::Normal
}

/// Calculate the peak meter decay weight based on sample rate
pub fn calculate_peak_meter_decay_weight(sample_rate: f32) -> f32 {
    0.25f64
        .powf((sample_rate as f64 * PEAK_METER_DECAY_MS / 1000.0).recip())
        as f32
}
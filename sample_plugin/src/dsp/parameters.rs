use nih_plug::prelude::*;
use nih_plug_iced::IcedState;
use std::sync::Arc;

use crate::gui::editor;

/// The time it takes for the peak meter to decay by 12 dB after switching to complete silence.
pub const PEAK_METER_DECAY_MS: f64 = 150.0;

/// Plugin parameter structure
#[derive(Params)]
pub struct GainParams {
    /// The editor state, saved together with the parameter state so the custom scaling can be
    /// restored.
    #[persist = "editor-state"]
    pub editor_state: Arc<IcedState>,

    #[id = "gain"]
    pub gain: FloatParam,
}

impl Default for GainParams {
    fn default() -> Self {
        Self {
            editor_state: editor::default_state(),

            // See the main gain example for more details
            gain: FloatParam::new(
                "Gain",
                util::db_to_gain(0.0),
                FloatRange::Skewed {
                    min: util::db_to_gain(-30.0),
                    max: util::db_to_gain(30.0),
                    factor: FloatRange::gain_skew_factor(-30.0, 30.0),
                },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit(" dB")
            .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
            .with_string_to_value(formatters::s2v_f32_gain_to_db()),
        }
    }
}
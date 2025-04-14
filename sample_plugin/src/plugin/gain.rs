use atomic_float::AtomicF32;
use nih_plug::prelude::*;
use std::sync::Arc;

use crate::dsp::parameters::GainParams;
use crate::dsp::processor;
use crate::gui::editor;
use crate::plugin::metadata::PluginMetadata;

/// This plugin implements a simple gain control with GUI.
pub struct Gain {
    params: Arc<GainParams>,
    peak_meter_decay_weight: f32,
    peak_meter: Arc<AtomicF32>,
}

impl Default for Gain {
    fn default() -> Self {
        Self {
            params: Arc::new(GainParams::default()),
            peak_meter_decay_weight: 1.0,
            peak_meter: Arc::new(AtomicF32::new(util::MINUS_INFINITY_DB)),
        }
    }
}

impl Plugin for Gain {
    const NAME: &'static str = PluginMetadata::NAME;
    const VENDOR: &'static str = PluginMetadata::VENDOR;
    const URL: &'static str = PluginMetadata::URL;
    const EMAIL: &'static str = PluginMetadata::EMAIL;
    const VERSION: &'static str = PluginMetadata::VERSION;

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = PluginMetadata::AUDIO_IO_LAYOUTS;
    const SAMPLE_ACCURATE_AUTOMATION: bool = PluginMetadata::SAMPLE_ACCURATE_AUTOMATION;

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        editor::create(
            self.params.clone(),
            self.peak_meter.clone(),
            self.params.editor_state.clone(),
        )
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        // Calculate the peak meter decay weight based on the sample rate
        self.peak_meter_decay_weight = processor::calculate_peak_meter_decay_weight(
            buffer_config.sample_rate as f32
        );

        true
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        processor::process_samples(
            buffer,
            &self.params,
            &self.peak_meter,
            self.peak_meter_decay_weight,
        )
    }
}

impl ClapPlugin for Gain {
    const CLAP_ID: &'static str = PluginMetadata::CLAP_ID;
    const CLAP_DESCRIPTION: Option<&'static str> = PluginMetadata::CLAP_DESCRIPTION;
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = PluginMetadata::CLAP_FEATURES;
}

impl Vst3Plugin for Gain {
    const VST3_CLASS_ID: [u8; 16] = PluginMetadata::VST3_CLASS_ID;
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = PluginMetadata::VST3_SUBCATEGORIES;
}

// Register plugin format exports
// nih_export_clap!(Gain);
// nih_export_vst3!(Gain);
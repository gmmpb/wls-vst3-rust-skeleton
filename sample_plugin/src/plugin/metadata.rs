use nih_plug::prelude::*;
use std::num::NonZeroU32;

/// Plugin metadata and descriptors
pub struct PluginMetadata;

impl PluginMetadata {
    pub const NAME: &'static str = "Sample Plugin - WLS Mark";
    pub const VENDOR: &'static str = "weblabstudio.hu";
    pub const URL: &'static str = "https://weblabstudio.hu";
    pub const EMAIL: &'static str = "hello@weblabstudio.hu";
    pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    pub const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(2),
            main_output_channels: NonZeroU32::new(2),
            ..AudioIOLayout::const_default()
        },
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(1),
            main_output_channels: NonZeroU32::new(1),
            ..AudioIOLayout::const_default()
        },
    ];

    pub const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    // CLAP specific metadata
    pub const CLAP_ID: &'static str = "com.moist-plugins-gmbh.gain-gui-iced";
    pub const CLAP_DESCRIPTION: Option<&'static str> = Some("A smoothed gain parameter example plugin");
    pub const CLAP_FEATURES: &'static [ClapFeature] = &[
        ClapFeature::AudioEffect,
        ClapFeature::Stereo,
        ClapFeature::Mono,
        ClapFeature::Utility,
    ];

    // VST3 specific metadata
    pub const VST3_CLASS_ID: [u8; 16] = *b"GainGuiIcedAaAAa";
    pub const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Fx, Vst3SubCategory::Tools];
}
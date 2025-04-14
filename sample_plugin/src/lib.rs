use nih_plug::prelude::*;

// Define the module structure
pub mod dsp;
pub mod gui;
pub mod plugin;

// Re-export the plugin struct and necessary types
pub use plugin::gain::Gain;

// Register plugin format exports
nih_export_clap!(Gain);
nih_export_vst3!(Gain);
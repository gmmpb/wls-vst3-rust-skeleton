use nih_plug::prelude::*;

use sample_plugin::Gain;

fn main() {
    nih_export_standalone::<Gain>();
}
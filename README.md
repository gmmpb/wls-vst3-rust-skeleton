# WLS VST3 Rust Skeleton

A minimal, modular audio plugin skeleton project using Rust and NIH-plug.

## Project Overview

This is a lightweight skeleton for creating VST3 and CLAP audio plugins with Rust. It provides a simplified but well-structured foundation that you can build upon.

## Project Structure

The project follows a modular architecture:

- **plugin_core**: The main plugin implementation that orchestrates everything
- **plugin_gui**: User interface implementation using NIH-plug VIZIA
- **features**: Common audio effects and processing algorithms
- **xtask**: Build utilities for bundling the plugin

## Getting Started

### Prerequisites

- Rust and Cargo (latest stable version recommended)
- A C++ compiler (for VST3 SDK)
- CMake (for building dependencies)

### Building

1. Build the plugin in development mode:

   ```
   cargo build
   ```

2. Bundle the plugin for distribution:
   ```
   cargo run --package xtask -- bundle plugin_core
   ```

The bundled plugin will be available in `target/bundled/` with both VST3 and CLAP versions.

## Architecture

This skeleton uses a modular approach that separates concerns:

- **Core Plugin**: Handles plugin lifecycle, parameter management, and audio processing
- **GUI**: Manages the user interface separately from the core functionality
- **Features**: Contains audio processing algorithms that can be reused and tested independently
- **XTask**: Provides build utilities specifically for audio plugin deployment

## Customizing

To extend this skeleton for your own plugin:

1. Update the plugin name, vendor, and identifiers in `plugin_core/src/plugin.rs`
2. Add your own audio processing in the `features` crate
3. Customize the GUI in `plugin_gui`
4. Add additional parameters as needed in `MyPluginParams`

## License

[MIT License](LICENSE)

## Acknowledgements

- [NIH-plug](https://github.com/robbert-vdh/nih-plug) - Rust audio plugin framework
- [VIZIA](https://github.com/vizia/vizia) - UI framework used by NIH-plug

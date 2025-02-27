#![doc(html_logo_url = "https://raw.githubusercontent.com/vulkano-rs/vulkano/master/logo.png")]
#![allow(clippy::missing_safety_doc)]
#![warn(rust_2018_idioms, rust_2021_compatibility)]

/// Create a surface either using winit or a RawWindowHandle
/// Its possible to disable either one using features
///
/// RawWindowHandle feature is called 'raw-window-handle_'
/// winit featur is called 'winit_'

#[cfg(feature = "raw-window-handle_")]
mod raw_window_handle;
#[cfg(feature = "raw-window-handle_")]
pub use crate::raw_window_handle::*;

#[cfg(feature = "winit_")]
mod winit;
#[cfg(feature = "winit_")]
pub use crate::winit::*;

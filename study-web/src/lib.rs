mod ui;
mod database;
mod graphics_3d;
mod camera_controll;
mod bevy_main;

use bevy_egui::EguiPlugin;
use bevy_main::bevy_run;
use bevy_mod_picking::{DefaultPickingPlugins, prelude::{RaycastPickCamera}};
use camera_controll::CameraControllPlugin;
use database::DataBasePlugin;
use graphics_3d::Graphics3dPlugins;
use ui::{MainUiPlugin, ui_elements::WindowBoxPlugin};
use wasm_bindgen::prelude::*;
use web_sys::console;

use std::path::Path;
use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::{PresentMode, WindowLevel, WindowTheme, PrimaryWindow}, core_pipeline::clear_color::ClearColorConfig,
};
use bevy_persistent::prelude::*;
use serde::*;
use dirs;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    // Your code goes here!
    bevy_run();

    Ok(())
}

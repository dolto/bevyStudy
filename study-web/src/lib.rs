mod ui;
mod database;

use bevy_egui::EguiPlugin;
use database::DataBasePlugin;
use ui::MainUiPlugin;
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

pub fn camera_spawn(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>
){
    let window = window_query.get_single().unwrap();
    commands.spawn(
        Camera2dBundle{
            transform: Transform::from_xyz(
                window.width()/2.0
                , window.height()/2.0
                , 0.0
            ),
            ..default()
        }
    );
}

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    // Your code goes here!
    console::log_1(&JsValue::from_str("Hello world!"));

    App::new()
    .add_plugins(
        (
        DefaultPlugins.set(
        WindowPlugin {
            primary_window: Some(Window{
                title: "game".into(),
                resolution: (393., 851.).into(),
                present_mode: PresentMode::AutoVsync,
                fit_canvas_to_parent: false,
                prevent_default_event_handling: false,
                window_theme: Some(WindowTheme::Dark),
                ..default()
            }),
            ..default()
        }
        ),
        EguiPlugin
        )
    ) //기반 플러그인
    .add_plugins(
        (
            DataBasePlugin,
            MainUiPlugin,
        )
    ) //개발 플러그인
    .add_systems(Startup, 
        (
            camera_spawn,
        )
    )
    .run();

    Ok(())
}

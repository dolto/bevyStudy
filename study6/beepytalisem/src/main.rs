// #[allow(unused_imports)]
// #[cfg(debug_assertions)] // new
// use bevy_dylib;

mod node;
use node::*;

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::{PresentMode, WindowLevel, WindowTheme, PrimaryWindow}, core_pipeline::clear_color::ClearColorConfig,
};

fn main() {
    App::new()
    .add_plugins(DefaultPlugins.set(
    WindowPlugin {
        primary_window: Some(Window{
            title: "game".into(),
            resolution: (393., 851.).into(),
            present_mode: PresentMode::AutoVsync,
            fit_canvas_to_parent: true,
            prevent_default_event_handling: false,
            window_theme: Some(WindowTheme::Dark),
            ..default()
        }),
        ..default()
    }   
    ))
    .add_systems(
        Startup, 
    (
        camera_spawn,
    ))
    .add_plugins(
        (
            MenuPlugin,
        )
    )
    .run();
}

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
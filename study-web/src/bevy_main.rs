use bevy::{prelude::*, window::{PresentMode, WindowTheme}};
use bevy_egui::EguiPlugin;
use bevy_mod_picking::DefaultPickingPlugins;

use crate::{camera_controll::CameraControllPlugin, database::DataBasePlugin, ui::{MainUiPlugin, ui_elements::WindowBoxPlugin}, graphics_3d::Graphics3dPlugins};

pub fn bevy_run(){
    println!("Hello bevy!");
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
        EguiPlugin,
        DefaultPickingPlugins.build()
        // .disable::<DebugPickingPlugin>()
        // .disable::<DefaultHighlightingPlugin>()
        // .disable::<SelectionPlugin>()
        )
    ) //기반 플러그인
    .add_plugins(
        (
            CameraControllPlugin,
            DataBasePlugin,
            MainUiPlugin,
            WindowBoxPlugin,
            Graphics3dPlugins
        )
    ) //개발 플러그인
    // .add_systems(Startup, 
    //     (
    //         camera_spawn,
    //     )
    // )
    .run();
}
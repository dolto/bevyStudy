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

#[derive(Resource, Serialize, Deserialize)]
struct SaveTheMessage {
    message: String
}

// fn setup(mut commands: Commands){
//     let test_my_message = dirs::config_dir().unwrap().join("my_message");
//     commands.insert_resource(
//         Persistent::<SaveTheMessage>::builder()
//         .name("message")
//         .format(StorageFormat::Ini)
//         .path(test_my_message.join("message.ini"))
//         .default(SaveTheMessage { message: "아직 메세지 입력 안됨".to_string() })
//         .build()
//         .expect("failed to initialize your message")
//     )
// }

fn setup_web(mut commands: Commands){
    let config_dir = dirs::config_dir()
    .map(|native_config_dir| native_config_dir.join("my_message"))
    .unwrap_or(Path::new("local").join("configuration"));

    commands.insert_resource(
        Persistent::<SaveTheMessage>::builder()
        .name("my_message")
        .format(StorageFormat::Json)
        .path(config_dir.join("message.json"))
        .default(SaveTheMessage { message: "아직 메세지 입력 안됨".to_string() })
        .build()
        .expect("failed to initialize your message")
    );
}

fn load_my_message(
    message: Res<Persistent<SaveTheMessage>>,
    check: Res<Input<KeyCode>>
){
    if check.just_pressed(KeyCode::A){
        console::log_1(&JsValue::from_str(&message.message));
    }
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
    .add_plugins(DefaultPlugins.set(
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
        ))
    .add_systems(Startup, 
    (
        setup_web,
        camera_spawn
    ))
    .add_systems(Update, 
    (
        load_my_message,
    ))
    .run();

    Ok(())
}

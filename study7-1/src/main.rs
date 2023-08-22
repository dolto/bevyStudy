use std::path::Path;
use bevy::prelude::*;
use bevy_persistent::prelude::*;
use serde::*;
use dirs;

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
        println!("your message: {:?}", message.message);
    }
}

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, 
    (
        setup_web,
    ))
    .add_systems(Update, 
    (
        load_my_message,
    ))
    .run();
}

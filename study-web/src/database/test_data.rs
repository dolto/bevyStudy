use std::path::Path;
use bevy::prelude::*;
use bevy_persistent::*;
use serde::*;
use dirs;
use wasm_bindgen::JsValue;
use web_sys::console;


#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum TestDataState{
    #[default]
    Unload,
    Reload
}

#[derive(Resource, Serialize, Deserialize)]
pub struct SaveTheMessage {
    pub message: String
}

pub fn setup_web(mut commands: Commands){
    let config_dir = dirs::config_dir()
    .map(|native_config_dir| native_config_dir.join("my_message"))
    .unwrap_or(Path::new("local").join("configuration"));

    commands.insert_resource(
        Persistent::<SaveTheMessage>::builder()
        .name("my_message")
        .format(StorageFormat::Json)
        .path(config_dir.join("message.json"))
        .default(SaveTheMessage { message: "아직 메세지 입력 안됨".to_string() })
        .unloaded(true) //콘솔을 열었을 때만 데이터 로드할 수 있도록 최초는 로드하지않음
        .revertible(true) //데이터를 기본값으로 되돌리기 가능하게끔 지정
        .build()
        .expect("failed to initialize your message")
    );
}

pub fn load_my_message(
    message: Res<Persistent<SaveTheMessage>>,
    check: Res<Input<KeyCode>>
){
    if check.just_pressed(KeyCode::A){
        console::log_1(&JsValue::from_str(&message.message));
    }
}

pub fn unload_my_message(
    mut message: ResMut<Persistent<SaveTheMessage>>
){
    message.unload().expect("내 메세지를 언로드 하는데 실패함!");
}

pub fn reload_my_message(
    mut message: ResMut<Persistent<SaveTheMessage>>
){
    message.reload().expect("내 메세지를 리로드 하는데 실패함!");
}

// fn modify_key_bindings(mut key_bindings: ResMut<Persistent<KeyBindings>>) {
//     key_bindings.update(|key_bindings| {
//         key_bindings.crouch = KeyCode::ControlLeft;
//     }).expect("failed to update key bindings"); //데이터를 수정함과 동시에 저장 파일에 반영
// }

// fn modify_key_bindings(mut key_bindings: ResMut<Persistent<KeyBindings>>) {
//     key_bindings.crouch = KeyCode::ControlLeft; //데이터를 그냥 수정만하고 파일엔 반영 x
// }

// fn persist_key_bindings(key_bindings: Res<Persistent<KeyBindings>>) {
//     key_bindings.persist().expect("failed to save new key bindings"); //수정된 데이터를 저장하는 방법
// }

// fn revert_key_bindings_to_default(key_bindings: Res<Persistent<KeyBindings>>) {
//     key_bindings.revert_to_default().expect("failed to revert key bindings to default"); //데이터를 기본값으로 돌리는 방법 (옵션 필요)
// }
mod resources;
mod systems;
mod components;

use bevy::prelude::*;
use wasm_bindgen::prelude::*;
//use wasm_bindgen::JsCast;
use web_sys::{window};
use crate::steat::AppState;

use self::resources::Score;
use systems::*;

pub struct ScorePlugin;
impl Plugin for ScorePlugin{
    fn build(&self, app: &mut App) {
        app
        .add_startup_system(score_borad_spawn)
        .init_resource::<Score>()
        .add_systems(
            (
                score_up,
                score_borad_update
            )
            .in_set(OnUpdate(AppState::Game))
        )
        .add_system(save_score.in_schedule(OnExit(AppState::Game)));
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = localStorage)]
    fn set_item(key: &str, value: &str);
    
    #[wasm_bindgen(js_namespace = localStorage)]
    fn get_item(key: &str) -> Option<String>;
}

pub fn set_item_to_web_storage(key: &str, value: &str) {
    if let Some(local_storage) = window().and_then(|w| w.local_storage().ok()) {
        if let Some(storage) = local_storage{
            storage.set_item(key, value).expect("스토리지에 점수 저장 완료");
        } else {
            panic!("스토리지에서 값 저장 실패");
        }
    } else {
        panic!("스토리지 불러오기 실패");
    }
}

pub fn get_item_from_web_storage(key: &str) -> Option<String> {
    if let Some(local_storage) = window().and_then(|w| w.local_storage().ok()) {
        if let Some(storage) = local_storage{
            storage.get_item(key).unwrap()
        } else {
            //panic!("스토리지에서 값 불러오기 실패");
            None
        }
    } else {
        panic!("스토리지 불러오기 실패");
    }
}
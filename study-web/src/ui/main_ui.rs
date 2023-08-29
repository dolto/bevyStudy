use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::{PresentMode, WindowLevel, WindowTheme, PrimaryWindow}, core_pipeline::clear_color::ClearColorConfig,
};
use bevy_egui::{*, egui::FontDefinitions};
use bevy_persistent::*;
use crate::database::test_data::{TestDataState, SaveTheMessage};

#[derive(Resource)]
pub struct ConsoleData {
    pub input_line: String,
    pub log: Vec<String>
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum TitleState{
    #[default]
    Titel,
    Console
}

pub fn ui_print_main_console(
    mut contexts: EguiContexts,
    mut console_data: ResMut<ConsoleData>
){
    egui::Window::new("Console").
    show(contexts.ctx_mut(), |ui| {
        ui.text_edit_singleline(&mut console_data.input_line);
        for l in console_data.log.iter(){
            ui.label(l.clone());
        }
    });
}

pub fn ui_open_console(
    mut title_state: ResMut<NextState<TitleState>>,
    input_key: Res<Input<KeyCode>>
){
    if input_key.just_pressed(KeyCode::Grave){
        title_state.set(TitleState::Console);
    }
}

pub fn ui_close_console(
    mut title_state: ResMut<NextState<TitleState>>,
    input_key: Res<Input<KeyCode>>
){
    if input_key.just_pressed(KeyCode::Grave){
        title_state.set(TitleState::Titel);
    }
}

pub fn ui_open_console_data(
    mut test_data_state: ResMut<NextState<TestDataState>>
){
    test_data_state.set(TestDataState::Reload);
}

pub fn ui_cloese_console_data(
    mut test_data: ResMut<NextState<TestDataState>>
){
    test_data.set(TestDataState::Unload);
}

pub fn ui_input_command(
    mut console_data: ResMut<ConsoleData>,
    input_key: Res<Input<KeyCode>>,
    mut test_data: ResMut<Persistent<SaveTheMessage>>
){
    if input_key.just_pressed(KeyCode::Return){
        let mut input_line = console_data.input_line.clone();
        input_line.pop();
        let input_line_vec = input_line.split(" ").collect::<Vec<&str>>();
        let mut result = String::new();

        console_data.input_line = "".to_string();
        if input_line_vec.len() < 2{
            result = "format is \"command arge\"".to_string();
        }
        else if input_line_vec[0] == "print"{
            if input_line_vec[1] == "testdata"{
                result = test_data.message.clone();
            }
            else {
                result = "this data dosn't exsit".to_string();
            }
        }
        else if input_line_vec[0] == "input"{
            if input_line_vec[1] == "testdata"{
                test_data.update(|test_data|{
                    test_data.message = input_line_vec[2].to_string();
                }).expect("데이터 갱신 실패!");
                result = test_data.message.clone();
            }
            else {
                result = "this data dosn't exsit".to_string();
            }
        }
        else {
            result = "command is can't read".to_string();
        }

        console_data.log.push(format!("{} \n\tresult: {}", input_line, result));
    }
}


pub mod main_ui;
pub mod ui_elements;
use main_ui::*;
use bevy::prelude::*;


pub struct MainUiPlugin;
impl Plugin for MainUiPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_state::<TitleState>()
        .insert_resource(ConsoleData { input_line: "".to_string(), log: Vec::with_capacity(100) })
        .add_systems(
            OnEnter(TitleState::Titel), 
            ui_cloese_console_data
        )
        .add_systems(
            OnEnter(TitleState::Console), 
            ui_open_console_data
        )
        .add_systems(
            Update
            , 
            (
                ui_close_console,
                ui_print_main_console,
                ui_input_command
            ).run_if(state_exists_and_equals(TitleState::Console))
        )
        .add_systems(
            Update
            , 
            (
                ui_open_console
            ).run_if(state_exists_and_equals(TitleState::Titel))
        );
        
    }
}
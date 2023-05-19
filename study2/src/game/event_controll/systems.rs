/**
 * 버비 이벤트
 * 이벤트는 리소스와 같이 그저 구조체일 뿐이며 데이터를 포함할 수도 있음
 * 리더를 사용하여 이벤트를 받고, 롸이터를 사용해서 이벤드를 보낼 수 있음
 * 기존에 있는 이벤트 말고도, 이벤트를 직접 만들어서 쓸 수 있음
 * 예제에서는 GameOver이벤트를 만들어서 사용함
 * 
 * 메인의 App::new() 에서 .add_event::<이벤트이름>() 으로 넣으면 이벤트로 등록이됨
 */
use bevy::{prelude::*, app::AppExit};
use crate::{game::{player::resources::HighScores, SimulationState}, app_state::{AppState}};
use super::events::GameOver;

pub fn exit_game(
    keyboard: Res<Input<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>
){
    if keyboard.just_pressed(KeyCode::Escape){
        app_exit_event_writer.send(AppExit);
    }
}

pub fn handle_game_over(
    mut game_over_event_reader: EventReader<GameOver>,
    mut commands: Commands
){
    for event in game_over_event_reader.iter(){
        println!("Your final score is : {}", event.score);
        commands.insert_resource(NextState(Some(AppState::GameOver)));
    }
    //이벤트는 동시에 여러번 일어날 수 있으므로 반복자를 고수하는게 좋음
}

pub fn update_high_scores(
    mut game_over_event_reader: EventReader<GameOver>,
    mut high_scores: ResMut<HighScores>
){
    for event in game_over_event_reader.iter(){
        high_scores.scores.push(("Player".to_string(), event.score));
        //일단 모든 유저를 Player로 저장
        high_scores.save();
    }
}

pub fn translation_to_game_state(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>
){
    if keyboard_input.just_pressed(KeyCode::G){
        if app_state.0 != AppState::Game {
            commands.insert_resource(NextState(Some(AppState::Game)));
            println!("Entered AppState::Game");
        }
    }
}

pub fn translation_to_main_menu_state(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>
){
    if keyboard_input.just_pressed(KeyCode::M){
        if app_state.0 != AppState::MainMenu {
            commands.insert_resource(NextState(Some(AppState::MainMenu)));
            commands.insert_resource(NextState(Some(SimulationState::Paused)));
            println!("Entered AppState::MainMenu");
        }
    }
}
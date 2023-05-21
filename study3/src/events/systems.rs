use bevy::prelude::*;

use crate::{steat::AppState, bird::{components::Bird, systems::BRIDSIZE}, pipe::components::Pip};

use super::events::GameOver;

pub fn game_over_evnet(
    mut game_over_event_reader: EventReader<GameOver>,
    mut next_menu: ResMut<NextState<AppState>>
){
    for _ in game_over_event_reader.iter(){
        println!("게임오버!");
        next_menu.set(AppState::Menu);
    }
}

pub fn bird_hit_pip(
    mut game_over_event_witer: EventWriter<GameOver>,
    brid_query: Query<&Transform, With<Bird>>,
    pipe_qurey: Query<(&Transform, &Pip)>
){
    if let Ok(brid_transform) = brid_query.get_single(){
        for (pip_transform, pip_self) in pipe_qurey.iter(){
            if pip_self.hit_trigger(pip_transform.translation, 
                brid_transform.translation, 
                BRIDSIZE/2.0, 
            BRIDSIZE/2.0){
                println!("파이프에 부딪힘!");
                game_over_event_witer.send(GameOver {  });
            }
        }
    }
}
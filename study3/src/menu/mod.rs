mod components;
mod systems;

use bevy::prelude::*;

use crate::steat::AppState;

use self::systems::{spawn_main_menu, despawn_main_menu, interaction_with_play_button};

pub struct MenuPlugin;
impl Plugin for MenuPlugin{
    fn build(&self, app: &mut App) {
        app
        .add_system(spawn_main_menu.in_schedule(OnEnter(AppState::Menu)))
        .add_system(despawn_main_menu.in_schedule(OnExit(AppState::Menu)))
        .add_system(interaction_with_play_button.run_if(in_state(AppState::Menu)));
    }
}
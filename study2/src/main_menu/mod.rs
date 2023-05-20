mod components;
mod systems;
mod style;

use bevy::prelude::*;
use systems::layout::*;
use systems::interaction::*;

use crate::app_state::AppState;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin{
    fn build(&self, app: &mut App) {
        app
        .add_system(spawn_main_menu.in_schedule(OnEnter(AppState::MainMenu)))
        .add_systems((
            interaction_with_play_button,
            interaction_with_quit_button
        ).in_set(OnUpdate(AppState::MainMenu))
    )
        .add_system(despawn_main_menu.in_schedule(OnExit(AppState::MainMenu)));
    }
}
pub mod components;
pub mod systems;

use bevy::prelude::*;

use crate::steat::AppState;

use self::systems::*;

pub struct BirdPlugin;
impl Plugin for BirdPlugin{
    fn build(&self, app: &mut App) {
        app
        .add_system(bird_spawn.in_schedule(OnEnter(AppState::Game)))
        .add_systems((
            bird_move,
            bird_jump,
            bird_out_screen
        )
        .in_set(OnUpdate(AppState::Game))
        )
        .add_system(bird_despawn.in_schedule(OnExit(AppState::Game)));
    }
}
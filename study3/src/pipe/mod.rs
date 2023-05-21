pub mod components;
mod systems;
mod resources;

use bevy::prelude::*;
use systems::*;
use resources::*;

use crate::steat::AppState;

pub struct PipPlugin;
impl Plugin for PipPlugin{
    fn build(&self, app: &mut App) {
        app
        .init_resource::<PipSpawnTimer>()
        .init_resource::<PipMoveSpeed>()
        .add_systems(
            (
                tick_pip_spawn_timer,
                pip_spawn_timer_finished,
                pip_out_screen,
                pip_move
            )
            .in_set(OnUpdate(AppState::Game))
        )
        .add_system(despawn_pip.in_schedule(OnExit(AppState::Game)));
    }
}
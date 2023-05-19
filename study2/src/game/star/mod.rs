use bevy::prelude::*;
use self::{resources::*, systems::*};
use crate::app_state::AppState;
use super::SimulationState;

pub mod components;
mod systems;
pub mod resources;

pub const STAR_SPAWN_TIME: f32 = 1.0;
pub const NUMBER_OF_STAR: usize = 10;
pub const STAR_SIZE:f32 = 30.0;

pub struct StarPlugin;
impl Plugin for StarPlugin{
    fn build(&self, app: &mut App) {
        app
        .init_resource::<StarSpawnTimer>()
        .add_system(sqawn_stars.in_schedule(OnEnter(AppState::Game)))
        .add_system(despawn_stars.in_schedule(OnExit(AppState::Game)))
        // .add_system(tick_star_spawn_timer)
        // .add_system(spawn_stars_over_time)
        .add_systems(
            (
                tick_star_spawn_timer,
                spawn_stars_over_time
            ).in_set(OnUpdate(AppState::Game))
            .in_set(OnUpdate(SimulationState::Running))
        );
    }
}
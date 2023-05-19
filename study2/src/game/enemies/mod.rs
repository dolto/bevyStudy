use bevy::prelude::*;
use crate::app_state::AppState;

use self::{resources::*, systems::*};

use super::SimulationState;

pub mod components;
mod systems;
pub mod resources;

pub const NUMBER_OF_ENEMIES: usize = 7;
pub const ENEMY_SPEED:f32 = 200.0;
pub const ENEMY_SIZE:f32 = 64.0;
pub const ENEMY_SPAWN_TIME:f32 = 5.0;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin{
    fn build(&self, app: &mut App) {
        app
        .init_resource::<EnemySpawnTimer>()
        .add_system(spawn_enemes.in_schedule(OnEnter(AppState::Game)))
        //스텟이 매개변수로 변경되고 최초 한번만 수행한다는 의미
        .add_system(despawn_enemies.in_schedule(OnExit(AppState::Game)))
        //스텟이 매개변수였다가 다른 스텟으로 변할 때 한번만 수행
        // .add_system(enemy_movement)
        // .add_system(update_enemy_direction)
        // .add_system(confine_enemy_movement)
        // .add_system(tick_enemy_spawn_timer)
        // .add_system(spawn_enemy_over_time)
        .add_systems((
            enemy_movement,
            update_enemy_direction,
            confine_enemy_movement,
            tick_enemy_spawn_timer,
            spawn_enemy_over_time
        ).in_set(OnUpdate(AppState::Game))
        .in_set(OnUpdate(SimulationState::Running))
        );
    }
}
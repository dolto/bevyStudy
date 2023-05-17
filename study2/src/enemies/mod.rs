use bevy::prelude::*;
use self::{resources::*, systems::*};

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
        .add_startup_system(spawn_enemes)
        .add_system(enemy_movement)
        .add_system(update_enemy_direction)
        .add_system(confine_enemy_movement)
        .add_system(tick_enemy_spawn_timer)
        .add_system(spawn_enemy_over_time);
    }
}
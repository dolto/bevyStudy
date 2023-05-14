use bevy::prelude::*;
use study2::player::*;
use study2::camera::*;
use study2::enemies::*;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_startup_system(sqawn_player)
    .add_startup_system(spawn_camera)
    .add_startup_system(spawn_enemes)
    .add_system(player_movement)
    .add_system(confine_player_movement)
    .add_system(enemy_movement)
    .add_system(update_enemy_direction)
    .add_system(confine_enemy_movement)
    .add_system(enemy_hit_player)
    .run();
}

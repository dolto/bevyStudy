use bevy::prelude::*;
use study2::player::*;
use study2::camera::*;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_startup_system(sqawn_player)
    .add_startup_system(spawn_camera)
    .run();
}

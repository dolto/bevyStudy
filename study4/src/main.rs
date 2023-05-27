mod components;
mod systems;
mod eventes;
mod resources;

use bevy::{prelude::*, window::PrimaryWindow, core_pipeline::clear_color::ClearColorConfig};
use bevy_rapier2d::{prelude::{RapierPhysicsPlugin, NoUserData}, render::RapierDebugRenderPlugin};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use systems::block::*;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(WorldInspectorPlugin::new())
    .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
    .add_plugin(RapierDebugRenderPlugin::default())
    .add_startup_system(camera_spawn)
    .add_startup_system(spawn_i_mino)
    //.add_startup_system(setup_physics)
    .run();
}

pub fn camera_spawn(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>
){
    let window = window_query.get_single().unwrap();
    commands.spawn(
        Camera2dBundle{
            transform: Transform::from_xyz(
                window.width()/2.0
                , window.height()/2.0
                , 0.0
            ),
            camera_2d: Camera2d{
                clear_color: ClearColorConfig::Custom(Color::rgb(0.3,0.3,0.65))
            },
            ..default()
        }
    );
}
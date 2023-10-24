mod honeycomb;
mod rim;

use bevy::prelude::*;

use self::honeycomb::{HuneycombRes, spawn_huneycomb, HuneycombSpawnState, set_huneycomb_node};

fn spawn_light(
    mut commands: Commands
){
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}

fn camera_spin(
    mut query_camera_transform: Query<&mut Transform, With<Camera>>,
    time: Res<Time>
){
    let mut camera_transform 
    = query_camera_transform.get_single_mut().unwrap();
    camera_transform.rotate_around(Vec3::ZERO, Quat::from_rotation_y(time.delta_seconds()));
}
pub struct Graphics3dPlugins;
impl Plugin for Graphics3dPlugins{
    fn build(&self, app: &mut App) {
        app
        .init_resource::<HuneycombRes>()
        .add_state::<HuneycombSpawnState>()
        .add_systems(Startup, (
            spawn_light,
        ))
        .add_systems(Update, (
            camera_spin,
        ))
        .add_systems(Update, (
            spawn_huneycomb,
        ).after(set_huneycomb_node).run_if(state_exists_and_equals(HuneycombSpawnState::IsSpawn)))
        .add_systems(Update, (
            set_huneycomb_node
        ).run_if(state_exists_and_equals(HuneycombSpawnState::IsSpawn)));
    }
}

use std::f32::consts::PI;

use bevy::{prelude::*, input::mouse::MouseButtonInput};
use bevy_mod_picking::prelude::RaycastPickCamera;

pub enum CameraMode {
    Camera2d,
    Camera3d
}

#[derive(Resource)]
pub struct CameraInfo{
    cameramod: CameraMode,
    location: Vec3,
    rotation: Quat,
    camera_entity: Entity,
    fov: f32,
    is_changed: bool
}

#[derive(Component)]
struct MainCamera;

pub fn camera_spawn(
    mut commands: Commands,
){
    let location = Transform::from_xyz(
        0.,5.,15.
    ).looking_at(Vec3::ZERO, Vec3::Y);
    let camera = commands.spawn((
        Camera3dBundle{
            transform: location.clone(),
            ..default()
        },
        RaycastPickCamera::default(),
        MainCamera,
    )).id();
    commands.insert_resource(CameraInfo{
        cameramod: CameraMode::Camera3d,
        location: location.translation,
        rotation: location.rotation.into(),
        camera_entity: camera,
        fov: PI/4.,
        is_changed: false
    });
}

pub fn camera_mode_switch(
    mut commands: Commands,
    time: Res<Time>,
    mut res_camerainfo: ResMut<CameraInfo>,
){
    if res_camerainfo.is_changed{
        match res_camerainfo.cameramod {
            CameraMode::Camera2d => {
                if res_camerainfo.fov <= PI/4.{
                    res_camerainfo.cameramod = CameraMode::Camera3d;
                    res_camerainfo.is_changed = false;
                }else{
                    let delay = time.delta_seconds();
                    res_camerainfo.fov -= delay;
                }
                commands.entity(res_camerainfo.camera_entity).insert(
                    Projection::Perspective(PerspectiveProjection { fov: res_camerainfo.fov, ..Default::default()})
                );
            },
            CameraMode::Camera3d => {
                if res_camerainfo.fov >= PI{
                    res_camerainfo.cameramod = CameraMode::Camera2d;
                    res_camerainfo.is_changed = false;
                    commands.entity(res_camerainfo.camera_entity).insert(
                        Projection::Orthographic(OrthographicProjection::default())
                    );
                    return
                }
                let delay = time.delta_seconds();
                res_camerainfo.fov += delay;
                commands.entity(res_camerainfo.camera_entity).insert(
                    Projection::Perspective(PerspectiveProjection { fov: res_camerainfo.fov, ..Default::default()})
                );
            },
        }
    }
}

pub fn camera_switch_test(
    mut res_camerainfo: ResMut<CameraInfo>,
    mut events: EventReader<MouseButtonInput>
){
    for ev in events.iter(){
        if ev.button == MouseButton::Middle{
            res_camerainfo.is_changed = true;
        }
    }
}

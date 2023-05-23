use std::time::Duration;

use bevy::{prelude::*, window::PrimaryWindow};
use rand::random;

use super::components::Pip;
use super::resources::*;

fn pip_spawn(
    commands: &mut Commands,
    window_query: &Query<&Window ,With<PrimaryWindow>>,
    asset_server: &Res<AssetServer>
){
    let window = window_query.get_single().unwrap();
    let mut bottum_size = random::<f32>() * window.height();
    let mut top_size = window.height() - bottum_size;

    if bottum_size > top_size{
        bottum_size -= 250.0;
    }
    else{
        top_size -= 250.0;
    }

    //println!("파이프 생성!");
    commands.spawn((
        Pip {
            width: 64.0,
            height: bottum_size - 150.0
        },
        SpriteBundle {
            texture: asset_server.load("sprites/dangerPlant.png"),
            sprite: Sprite { 
                custom_size: Some(Vec2 { x: 64.0, y: bottum_size }),
                ..default()
            },
            transform: Transform::from_xyz(window.width(), 0.0 + bottum_size /2.0, 0.0),
            ..default()
        }
    ));

    commands.spawn((
        Pip {
            width: 64.0,
            height: top_size - 150.0
        },
        SpriteBundle {
            texture: asset_server.load("sprites/dangerPlant.png"),
            sprite: Sprite { 
                custom_size: Some(Vec2 { x: 64.0, y: top_size }),
                flip_y: true,
                ..default()
            },
            transform: Transform::from_xyz(window.width(), window.height() - top_size / 2.0, 0.0),
            ..default()
        }
    ));
}

pub fn despawn_pip(
    mut commands:Commands,
    pip_query: Query<Entity, With<Pip>>
){
    for pip_entity in pip_query.iter(){
        commands.entity(pip_entity).despawn();
    }
}

pub fn tick_pip_spawn_timer(
    mut pip_spawn_timer: ResMut<PipSpawnTimer>,
    pip_move_speed: Res<PipMoveSpeed>,
    time:Res<Time>
){
    let add_time = time.delta_seconds() * STARTPIPSPEED / pip_move_speed.x_speed / 1.5;
    pip_spawn_timer.timer.tick(Duration::from_secs_f32(add_time) );
}

pub fn pip_spawn_timer_finished(
    mut commands: Commands,
    window_query: Query<&Window ,With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    pip_spawn_timer: Res<PipSpawnTimer>,
    time:Res<Time>,
    mut pip_move_speed: ResMut<PipMoveSpeed>
){
    if pip_spawn_timer.timer.finished() {
        pip_move_speed.x_speed += time.delta_seconds() * 2.0;
        pip_spawn(&mut commands, &window_query, &asset_server);
    }
}

pub fn pip_out_screen(
    mut commands: Commands,
    pip_query: Query<(Entity, &Transform) ,With<Pip>>
){
    for (pip_entity, pip_transform) in pip_query.iter(){
        if pip_transform.translation.x < 0.0{
           // println!("파이프 제거!");
            commands.entity(pip_entity).despawn();
        }
    }
}

pub fn pip_move(
    mut pip_query: Query<&mut Transform, With<Pip>>,
    pip_speed: Res<PipMoveSpeed>,
    time:Res<Time>
){
    for mut pip_transform in pip_query.iter_mut(){
        let mut translation = pip_transform.translation;
        //println!("이동전 {}", translation.x);
        translation.x -= pip_speed.x_speed * time.delta_seconds();
        //println!("이동후 {}", translation.x);
        pip_transform.translation = translation;
    }
}
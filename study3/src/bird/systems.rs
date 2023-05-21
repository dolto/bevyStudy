use bevy::{prelude::*, window::PrimaryWindow};

use super::components::Bird;
use crate::events::events::GameOver;

const GRAVITY:f32 = 23.0;
const MAXSPEED:f32 = 800.0;
const JUMPOWER:f32 = 8.0;
pub const BRIDSIZE:f32 = 64.0;

pub fn bird_spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_qurey: Query<&Window, With<PrimaryWindow>>
){
    let main_window = window_qurey.get_single().unwrap();
    commands.spawn((
        Bird {y_speed: 0.0},
        SpriteBundle {
            sprite: Sprite { 
                custom_size: Some(Vec2::new(BRIDSIZE, BRIDSIZE)),
                ..default()
            },
            texture: asset_server.load("sprites/greenFish.png"),
            transform: Transform::from_xyz(
                main_window.width() / 2.0, main_window.height() / 2.0, 0.0),
            ..default()
        }
    ));
}

pub fn bird_despawn(
    mut commands: Commands,
    bird: Query<Entity, With<Bird>>
){
    if let Ok(bird_entity) = bird.get_single(){
        commands.entity(bird_entity).despawn();
    }
}

pub fn bird_move(
    mut bird_qurey: Query<&mut Transform, With<Bird>>,
    mut bird_yspeed: Query<&mut Bird>,
    time: Res<Time>
){
    let mut speed:f32 = 0.0;
    if let Ok(mut bird) = bird_yspeed.get_single_mut(){
        speed = bird.y_speed;
        bird.y_speed = (speed - GRAVITY * time.delta_seconds()).max(-MAXSPEED);
    }
    if let Ok(mut bird_transform) = bird_qurey.get_single_mut(){
        let mut translation = bird_transform.translation;
        let mut rotation = bird_transform.rotation;
        translation.y += speed;
        rotation.z = speed / MAXSPEED * 20.0;
        //println!("{}", rotation.z);
        bird_transform.rotation = rotation;
        bird_transform.translation = translation;
    }
}

pub fn bird_jump(
    mut bird_yspeed: Query<&mut Bird>,
    input_key: Res<Input<KeyCode>>,
    input_mouse: Res<Input<MouseButton>>
){
    if let Ok(mut bird) = bird_yspeed.get_single_mut(){
        if input_key.just_pressed(KeyCode::A) || input_mouse.just_pressed(MouseButton::Left){
            bird.y_speed = JUMPOWER;
        }
    }
}

pub fn bird_out_screen(
    mut bird_qurey: Query<&mut Transform, With<Bird>>,
    window_qurey: Query<&Window, With<PrimaryWindow>>,
    mut game_over_writer: EventWriter<GameOver>
){
    let window = window_qurey.get_single().unwrap();
    if let Ok(bird) = bird_qurey.get_single_mut(){
        let translation = bird.translation;
        if translation.y > window.height() || translation.y < 0.0{
            game_over_writer.send(GameOver {  });
        }
    }
}
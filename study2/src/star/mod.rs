use bevy::{prelude::*, window::PrimaryWindow};
use rand::random;

pub const NUMBER_OF_STAR: usize = 10;
pub const STAR_SIZE:f32 = 30.0;

#[derive(Component)]
pub struct Star {}

pub fn sqawn_stars(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
){
    let window = window_query.get_single().unwrap();

    for _ in 0..NUMBER_OF_STAR{
        spawn_star(&asset_server, &mut commands, window.width(), window.height());
    }
}

pub fn spawn_star(asset_server: &Res<AssetServer>,commands: &mut Commands,width: f32, height: f32){
    let random_x = random::<f32>() * width;
    let random_y = random::<f32>() * height;

    commands.spawn(
        (
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/star.png"),
                ..default()
            },
            Star {}
        )
    );
}
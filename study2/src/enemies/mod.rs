use bevy::{prelude::*, window::PrimaryWindow};
use rand::random;

#[derive(Component)]
pub struct Enemy{}

pub fn spawn_enemes(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
){
    let window = window_query.get_single().unwrap();

    for _ in 0..crate::player::NUMBER_OF_ENEMIES{
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn(
            (
                SpriteBundle{
                    transform: Transform::from_xyz(random_x, random_y, 0.0),
                    texture: asset_server.load("sprites/ball_red_large.png"),
                    ..default()
                },
                Enemy{}
            )
        );
    }
}
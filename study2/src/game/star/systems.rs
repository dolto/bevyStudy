use bevy::{prelude::*, window::PrimaryWindow};
use rand::random;
use super::{components::Star, resources::StarSpawnTimer, *};


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
pub fn despawn_stars(
    mut commands: Commands,
    star_query: Query<Entity, With<Star>>
){
    for star_entity in star_query.iter(){
        commands.entity(star_entity).despawn();
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

pub fn tick_star_spawn_timer(
    mut star_spawn_timer: ResMut<StarSpawnTimer>,
    time: Res<Time>
){
    star_spawn_timer.timer.tick(time.delta());
    //타이머에 시간을 더하는 함수인듯? 근데 f32 가 아니라 duration 으로 반환한다. (더 세밀한 단위인듯?)
}

pub fn spawn_stars_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    star_spawn_timer: Res<StarSpawnTimer>
){
    if star_spawn_timer.timer.finished() { //타이머가 0으로 갔을 경우 모드가 리피트임으로 초기화 됨
        let window = window_query.get_single().unwrap();
        spawn_star(&asset_server, &mut commands, window.width(), window.height());
    }
}
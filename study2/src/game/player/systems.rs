use bevy::{prelude::*, window::PrimaryWindow};
use crate::{game::event_controll::events::GameOver, game::star::{components::Star, STAR_SIZE}, game::enemies::{components::Enemy, *}};
use super::{components::Player, resources::{HighScores, Score}, *};



pub fn high_scores_updated(
    high_scores: Res<HighScores>
){
    if high_scores.is_changed() {
        println!("High Scores: {:?}", high_scores);
    }
}


pub fn sqawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
){
    let window = window_query.get_single().unwrap();
    commands.spawn(
        (
            SpriteBundle{
                transform: Transform::from_xyz(window.width() /2.0, window.height() /2.0, 0.0),
                texture: asset_server.load("sprites/ball_blue_large.png"),
                visibility:Visibility::Visible,
                ..default()
            },
            Player { },
        )
    );
}

pub fn despawn_player(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>
){
    if let Ok(player_entity) = player_query.get_single(){
        commands.entity(player_entity).despawn();
    }
}

//bevy에서 수행되는 메인 윈도우창은 PrimaryWindow를 포함하고 있기 때문에
//해당 윈도우를 참조하려면 저런식으로 해야한다
//또한 PNG, OGG파일을 로드하기 위해서 Res<AssetServer> 를 써서 에셋서버에 접근해야한다.
//즉 버비는(혹은 기본 플러그인은) 자동적으로 윈도우와 에셋을 저장하는 엔티티를 생성한다.


//번들에 대해서
/***
 * 미리 정의된 구성요소 집합
 * 그니까 구조체라고 보면 됨
 * 또한 러스트 기본 기능의 Default를 사용할 수 있는 트레잇 소유
 */

//리소스에 대해
/***
 * 고유하고 전역적으로 접근가능한 구조체 (쿼리와는 달리 인스턴트 식으로 쓰는게 아닌듯?)
 * 주로 데이터에 사용되며 자산서버의 개념을 이용해서 데이터에 접근 가능
 * Res<T> : 읽기 전용으로 접근
 * ResMut<T> : 가변 참조로 접근
 */



pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>
){
    if let Ok(mut transform) = player_query.get_single_mut(){
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A){
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D){
            direction += Vec3::new(1.0,0.0,0.0);
        }
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W){
            direction += Vec3::new(0.0,1.0,0.0);
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S){
            direction += Vec3::new(0.0,-1.0,0.0);
        }

        if direction.length() > 0.0{
            direction = direction.normalize();
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();

    }
}

pub fn confine_player_movement(
    mut player_query:Query<&mut Transform, With<Player>>,
    window_query:Query<&Window, With<PrimaryWindow>>
){
    if let Ok(mut player_transform) = player_query.get_single_mut(){
        let window = window_query.get_single().unwrap();

        let half_player_size = PLAYER_SIZE/2.0;
        let x_min = half_player_size;
        let x_max = window.width() - half_player_size;
        let y_min = half_player_size;
        let y_max = window.height() - half_player_size;

        let mut translation = player_transform.translation;

        if translation.x < x_min{
            translation.x = x_min;
        }else if translation.x > x_max{
            translation.x = x_max;
        }

        if translation.y < y_min{
            translation.y = y_min;
        }else if translation.y > y_max{
            translation.y = y_max;
        }

        player_transform.translation = translation;
    }
}

pub fn enemy_hit_player(
    mut commands: Commands,
    mut game_over_event_witer: EventWriter<GameOver>,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    score: Res<Score>
){
    if let Ok((player_entity, player_transform)) = player_query.get_single_mut() {
        for enemy_transform in enemy_query.iter() {
            let distance = player_transform
            .translation.distance(enemy_transform.translation);
        //플레이어와 적의 거리를 구함
            let player_radius = PLAYER_SIZE / 2.0;
            let enemy_radius = ENEMY_SIZE /2.0;
            if distance < player_radius + enemy_radius{
                println!("Enemy hit player! Game Over!");
                let sound_effect = asset_server.load("audio/explosionCrunch_000.ogg");
                audio.play(sound_effect);
                game_over_event_witer.send(GameOver { score: score.value });
                commands.entity(player_entity).despawn(); //플레이어의 엔티티 번호를 죽인다
                //유니티의 Destroy와 비슷한 개념인듯
            }
        }
    }
}

pub fn star_hit_player(
    mut commands: Commands,
    mut player_query: Query<&Transform, With<Player>>,
    star_query: Query<(&Transform, Entity), With<Star>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut score: ResMut<Score>
){
    if let Ok(player_transform) = player_query.get_single_mut(){
        for (star_transform, star_entity) in star_query.iter(){
            let distance = star_transform.
            translation.distance(player_transform.translation);

            let player_radius = PLAYER_SIZE / 2.0;
            let star_radius = STAR_SIZE / 2.0;

            if distance < player_radius + star_radius {
                score.value += 1;
                println!("Player get Star! {}", score.value.to_string());
                let sound_effect = asset_server.load("audio/impactGeneric_light_000.ogg");
                audio.play(sound_effect);
                commands.entity(star_entity).despawn();
            }
        }
    }
}

pub fn update_score(
    score: Res<Score>,
){
    if score.is_changed() {
        print!("Player Score is {}  ", score.value.to_string());
    }
}

pub fn insert_score(
    mut commands: Commands
){
    commands.insert_resource(Score::default())
}

pub fn remove_score(
    mut commands: Commands
){
    commands.remove_resource::<Score>();
}
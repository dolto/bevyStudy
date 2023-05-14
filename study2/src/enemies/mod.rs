use bevy::{prelude::*, window::PrimaryWindow};
use rand::random;

#[derive(Component)]
pub struct Enemy{
    pub directoion: Vec2,
}

pub const NUMBER_OF_ENEMIES: usize = 7;
pub const ENEMY_SPEED:f32 = 200.0;
pub const ENEMY_SIZE:f32 = 64.0;

pub fn spawn_enemes(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
){
    let window = window_query.get_single().unwrap();

    for _ in 0..NUMBER_OF_ENEMIES{
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn(
            (
                SpriteBundle{
                    transform: Transform::from_xyz(random_x, random_y, 0.0),
                    texture: asset_server.load("sprites/ball_red_large.png"),
                    ..default()
                },
                Enemy{
                    directoion: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
                }
            )
        );
    }
}

pub fn enemy_movement(
    mut enemy_query: Query<(&mut Transform, &Enemy)>,
    time: Res<Time>
){
    for (mut transform, enemy) in enemy_query.iter_mut(){
        let direction = Vec3::new(enemy.directoion.x, enemy.directoion.y, 0.0);
        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
    }
}

pub fn update_enemy_direction(
    mut enemy_query: Query<(&Transform, &mut Enemy, Entity)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    audio:Res<Audio>,
    asset_server: Res<AssetServer>
){
    let window = window_query.get_single().unwrap();

    let half_enemy_size = ENEMY_SIZE /2.0;
    let x_min = half_enemy_size;
    let x_max = window.width() - half_enemy_size;
    let y_min = half_enemy_size;
    let y_max = window.height() - half_enemy_size;

    let mut other_translation_list: Vec<(Vec3, Vec2, Entity)> = Vec::with_capacity(NUMBER_OF_ENEMIES);

    for (transform, mut enemy, entity) in enemy_query.iter_mut(){
        let translation = transform.translation;
        other_translation_list.push((translation, enemy.directoion, entity));
    }
    //비교할 데이터를 수집한 후

    for (transform, mut enemy, entity) in enemy_query.iter_mut(){
        let mut direction_changed = false;

        let translation = transform.translation;
        if translation.x < x_min || translation.x > x_max{
            enemy.directoion.x *= -1.0;
            direction_changed = true;
        }
        if translation.y < y_min || translation.y > y_max {
            enemy.directoion.y *= -1.0;
            direction_changed = true;
        }

        for (other_translation, other_direction, other_entity) in other_translation_list.iter(){
            if entity.eq(other_entity){
                continue;
            }
            let distance = transform.translation.distance(other_translation.clone());
            if distance < ENEMY_SIZE {
                enemy.directoion = other_direction.clone();
                direction_changed = true;
            }
        }
        //여기에서 체크함
        
        if direction_changed{
            let sound_effect_1 = asset_server.load("audio/pluck_001.ogg");
            let sound_effect_2 = asset_server.load("audio/pluck_002.ogg");

            let sound_effect = if random::<f32>() > 0.5 {
                sound_effect_1
            }else{
                sound_effect_2
            };
            audio.play(sound_effect);
        }
    }
}

pub fn confine_enemy_movement(
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    let window = window_query.get_single().unwrap();
    let half_enemy_size = ENEMY_SIZE / 2.0;
    let x_min = half_enemy_size;
    let x_max = window.width() - half_enemy_size;
    let y_min = half_enemy_size;
    let y_max = window.height() - half_enemy_size;

    for mut transform in enemy_query.iter_mut(){
        let mut translation = transform.translation;

        if translation.x < x_min {
            translation.x = x_min;
        }else if translation.x > x_max{
            translation.x = x_max;
        }
        if translation.y < y_min {
            translation.y = y_min;
        }else if translation.y > y_max{
            translation.y = y_max;
        }

        transform.translation = translation;
    }
}
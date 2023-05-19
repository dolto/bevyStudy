use bevy::{prelude::*, window::PrimaryWindow};

pub fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
){
    let window = window_query.get_single().unwrap();

    commands.spawn(
        Camera2dBundle{
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            ..default()
        }
    );
}

//카메라
/***
 * 카메라는 유니티랑 비슷한 개념으로 bevy 세상을 찍는 역할을 함
 * 아직 서브 카메라나 메인카메라가 존재하는지 잘 모르는 상태
 */
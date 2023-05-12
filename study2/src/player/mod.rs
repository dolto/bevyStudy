use bevy::{prelude::*, window::PrimaryWindow};

#[derive(Component)]
pub struct Player {

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
            Player {},
        )
    );
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
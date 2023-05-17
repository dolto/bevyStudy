//mod enemies;
//메인에 있어도 되고, 어느 파일에 있어도 상대경로로 작동한다.
/***
 * 또한 현재 되어있는 형태로 파일을(모듈) 나누는 것이 아니라
 * 컴포넌트, 이벤트, 리소스, 시스템으로 나누는 것이 좀 더 표준적이고
 * use낭비를 줄여준다.
 * 
 * 좀 더 세분화 하여 사용한다면, 지금처럼 각 개념을 중심으로
 * 컴포넌트, 리소스, 시스템을 분할하는 것이 좀 더 깔끔하다.
 */
use bevy::prelude::*;
use study2::camera::CameraPlugin;
use study2::enemies::EnemyPlugin;
use study2::player::PlayerPlugin;
use study2::event_controll::EventControllPlugin;
use study2::star::StarPlugin;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(EventControllPlugin) //플러그인들이 해당이벤트에 의존 하는경우 먼저 수행되어야 오류가 안남
    .add_plugin(CameraPlugin)
    .add_plugin(EnemyPlugin)
    .add_plugin(PlayerPlugin)
    .add_plugin(StarPlugin)
    .run();
}

/***
 * plugins: 수행하는 모든 리소스, 시스템등을 구현하고 집어넣어서 한번에 수행하는 패키지
 * resource: 게임 내부에서 사용되는 데이터
 * system: 게임 내부에 돌아가는 시스템
 * 
 * resource는 insert_resource 함수와 init_resource 함수가 존재하는데
 * 전자는 집어넣고 resource의 구조체를 생성하면 되고 .insert_resource(구조체 {요소:값, 요소:값...})
 * 후자는 집어넣고 impl에 미리 정의된 default 함수를 수행한다.
 * init_resource::<구조체>() //이런 형태를 터보 피쉬라고 하는데 ::<>() 이게 마치 물고기 모양같다고;;...
 * 예시
 * #[derive(Resource)]
    pub struct Score {
        pub value: u32,
    }
    impl Default for Score {
        fn default() -> Score {
            Score { value: 0 }
        }
    }
 */
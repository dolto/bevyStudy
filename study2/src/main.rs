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
use study2::app_state::AppState;
use study2::game::GamePlugin;
use study2::main_menu::MainMenuPlugin;

fn main() {
    App::new()
    //Bevy Plugins
    .add_plugins(DefaultPlugins)
    .add_state::<AppState>()
    //My Plugins
    .add_plugin(MainMenuPlugin)
    .add_plugin(GamePlugin)
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


/***
 * Order 시스템으로 Bevy의 스케줄링 알고리즘으로 프레임마다 수행되는 함수의 순서가 다를 수 있음을
 * 알려줌
 * 최적화를 위한 알고리즘이지만, 이를 해결하기 위해 System Ordering 이라는 개념을 사용함
 * 
 * 당연하지만 이런 Ordering은 애초에 병렬 수행이 어려운 함수들에게만 대상으로 하지 않으면
 * 성능에 영향을 끼침
 */

/***
 * States 기능
 * 유니티의 씬 기능과 같이, 특정 상태 혹은 특정 상태집합에 있는 동안 수행하는 시스템이나 기능들을
 * 설정할 수 있다.
 */
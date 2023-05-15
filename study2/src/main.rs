use bevy::prelude::*;
use study2::player::*;
use study2::camera::*;
use study2::enemies::*;
use study2::star::*;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .init_resource::<Score>() //리소스를 넣는 방법
    .init_resource::<StarSpawnTimer>()
    .add_startup_system(sqawn_player)
    .add_startup_system(spawn_camera)
    .add_startup_system(spawn_enemes)
    .add_startup_system(sqawn_stars)
    .add_system(player_movement)
    .add_system(confine_player_movement)
    .add_system(enemy_movement)
    .add_system(update_enemy_direction)
    .add_system(confine_enemy_movement)
    .add_system(enemy_hit_player)
    .add_system(star_hit_player)
    .add_system(update_score)
    .add_system(tick_star_spawn_timer)
    .add_system(spawn_stars_over_time)
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
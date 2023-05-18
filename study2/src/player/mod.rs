use bevy::prelude::*;
use self::{resources::*, systems::*};

pub mod components;
mod systems;
pub mod resources;

pub const PLAYER_SPEED:f32 = 500.0;
pub const PLAYER_SIZE:f32 = 64.0;

//SystemSet 이라는 개념
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct MovementSystemSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct ConfinementSystemSet;

//위의 두 개념과 동일한 형태로 생성 가능
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum PlayerSystemSet{
    Movement,
    Confinemet
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin{
    fn build(&self, app: &mut App) {
        app
        .init_resource::<Score>() //리소스를 넣는 방법
        .init_resource::<HighScores>()
        //.configure_set(MovementSystemSet.before(ConfinementSystemSet))
        .configure_set(PlayerSystemSet::Movement.before(PlayerSystemSet::Confinemet))
        .add_startup_system(sqawn_player)
        //.add_system(player_movement/*.before(confine_player_movement)*/)
        //.add_system(confine_player_movement.after(player_movement))
        // .add_systems(
        //     (
        //         player_movement,
        //         confine_player_movement/*.after(player_movement) */
        //     ).chain() //함수 수행의 스케줄을 명시함
        // )
        // .add_system(player_movement.in_set(MovementSystemSet))
        // .add_system(confine_player_movement.in_set(ConfinementSystemSet))
        .add_system(player_movement.in_set(PlayerSystemSet::Movement))
        .add_system(confine_player_movement.in_set(PlayerSystemSet::Confinemet))
        //위에 있는 configure_set으로 함수의 수행 순서를 정의함
        .add_system(enemy_hit_player)
        .add_system(star_hit_player)
        .add_system(update_score)
        //.add_system(update_high_scores)
        .add_system(high_scores_updated);
    }
}
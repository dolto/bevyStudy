mod resources;
mod systems;
mod components;

use bevy::prelude::*;

use crate::steat::AppState;

use self::resources::Score;
use systems::*;

pub struct ScorePlugin;
impl Plugin for ScorePlugin{
    fn build(&self, app: &mut App) {
        app
        .add_startup_system(score_borad_spawn)
        .init_resource::<Score>()
        .add_systems(
            (
                score_up,
                score_borad_update
            )
            .in_set(OnUpdate(AppState::Game))
        )
        .add_system(save_score.in_schedule(OnExit(AppState::Game)));
    }
}
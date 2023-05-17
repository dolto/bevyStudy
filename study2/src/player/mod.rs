use bevy::prelude::*;
use self::{resources::*, systems::*};

pub mod components;
mod systems;
pub mod resources;

pub const PLAYER_SPEED:f32 = 500.0;
pub const PLAYER_SIZE:f32 = 64.0;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin{
    fn build(&self, app: &mut App) {
        app
        .init_resource::<Score>() //리소스를 넣는 방법
        .init_resource::<HighScores>()
        .add_startup_system(sqawn_player)
        .add_system(player_movement)
        .add_system(confine_player_movement)
        .add_system(enemy_hit_player)
        .add_system(star_hit_player)
        .add_system(update_score)
        //.add_system(update_high_scores)
        .add_system(high_scores_updated);
    }
}
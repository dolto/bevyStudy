pub mod events;
mod systems;

use bevy::prelude::*;

use crate::steat::AppState;

use self::{systems::*, events::GameOver};

pub struct EventPlugin;
impl Plugin for EventPlugin{
    fn build(&self, app: &mut App) {
        app
        .add_event::<GameOver>()
        .add_systems((
            game_over_evnet,
            bird_hit_pip
        )
        .in_set(OnUpdate(AppState::Game))
        );
    }
}
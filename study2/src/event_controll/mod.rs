use bevy::prelude::*;
use self::{events::*,systems::*};

pub mod events;
mod systems;

pub struct EventControllPlugin;

impl Plugin for EventControllPlugin{
    fn build(&self, app: &mut App) {
        app
        .add_event::<GameOver>()
        .add_system(exit_game)
        .add_system(handle_game_over)
        .add_system(update_high_scores);
    }
}
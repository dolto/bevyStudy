mod camera;
mod enemies;
mod player;
mod event_controll;
mod star;
mod systems;

use bevy::prelude::*;
use camera::CameraPlugin;
use enemies::EnemyPlugin;
use player::PlayerPlugin;
use event_controll::EventControllPlugin;
use star::StarPlugin;
use systems::*;
use crate::app_state::AppState;

pub struct GamePlugin;

impl Plugin for GamePlugin{
    fn build(&self, app: &mut App){
        app
        .add_state::<SimulationState>()
        .add_system(pause_simulation.in_schedule(OnEnter(AppState::Game)))
        .add_system(resume_simulation.in_schedule(OnExit(AppState::Game)))
        .add_plugin(EventControllPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(StarPlugin)
        .add_system(toggle_simulation.run_if(in_state(AppState::Game)));
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused
}
mod game_logic;

use bevy::{prelude::{*, system_adapter::new}, window::PrimaryWindow, core_pipeline::clear_color::ClearColorConfig};
use rand::{seq::SliceRandom, thread_rng};

fn main() {
    App::new()
    .add_plugins(DefaultPlugins);
}

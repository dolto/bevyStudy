pub mod window_box;
use bevy::prelude::*;

use self::window_box::test_window_spawn;

pub struct WindowBoxPlugin;
impl Plugin for WindowBoxPlugin{
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, 
        test_window_spawn);
    }
}
pub mod window_box;
use bevy::prelude::*;

use self::window_box::*;

pub struct WindowBoxPlugin;
impl Plugin for WindowBoxPlugin{
    fn build(&self, app: &mut App) {
        app
        // .insert_resource(

        // )
        .add_systems(Startup, 
        test_window_spawn
        )
        .add_systems(Update,
            (
                ui_window_move_triger,
                ui_window_move,
                ui_window_remove
            ) 
        );
    }
}
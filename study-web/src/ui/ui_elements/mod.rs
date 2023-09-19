pub mod window_box;
pub mod side_box;
pub mod text_box;
use bevy::prelude::*;

use self::{window_box::*, side_box::*, text_box::*};

pub struct WindowBoxPlugin;
impl Plugin for WindowBoxPlugin{
    fn build(&self, app: &mut App) {
        app
        .insert_resource(
            TextBoxFocus {activity: false, focus: Entity::from_bits(0)}
        )
        .add_systems(Startup, 
        //test_window_spawn,
        //ui_test_sidebox
        ui_test_textbox
        )
        .add_systems(Update,
            (
                ui_window_move_triger,
                ui_window_move,
                ui_window_remove,
                
                ui_sidebox_toggle,
                ui_sidebox_anim,
                ui_sidebox_close,

                ui_textbox_input,
                ui_textbox_set_focus,
                ui_toggle_ime
            ) 
        );
    }
}
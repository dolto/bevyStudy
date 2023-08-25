pub mod test_data;

use bevy::prelude::*;
use self::test_data::*;

pub struct DataBasePlugin;
impl Plugin for DataBasePlugin{
    fn build(&self, app: &mut App) {
       app
       .add_state::<TestDataState>()
       .add_systems(
        Startup, 
    (
                setup_web,
            )
        )
        .add_systems(Update, 
            (
                load_my_message,
            )
        )
        .add_systems(
            OnEnter(TestDataState::Unload), 
            (
                unload_my_message,
            )
        )
        .add_systems(
            OnEnter(TestDataState::Reload), 
            (
                reload_my_message,
            )
        )
        ;
    }
}
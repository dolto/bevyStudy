mod main_menu;

use bevy::prelude::*;
use main_menu::*;


#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum MenuState{
    #[default]
    Titel,
    Menu,
    Game
}

pub struct MenuPlugin;
impl Plugin for MenuPlugin{
    fn build(&self, app: &mut App) {
        app
        .add_state::<MenuState>()
        .init_resource::<MainAnimTime>()
        .add_systems(OnEnter(MenuState::Titel), 
        menu
        )
        .add_systems(Update, 
            (
                anim_timer,
                menu_anime
            ).run_if(state_exists_and_equals(MenuState::Titel))
        );
    }
}
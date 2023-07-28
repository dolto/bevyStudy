mod main_menu;

use bevy::prelude::*;
use main_menu::*;

pub struct MenuPlugin;
impl Plugin for MenuPlugin{
    fn build(&self, app: &mut App) {
        app
        .add_state::<MenuState>()
        .init_resource::<MainAnimTime>()
        .add_systems(OnEnter(MenuState::Titel), 
        spawn_title
        )
        .add_systems(OnEnter(MenuState::Menu), 
        (
                spawn_menu,
            )
        )
        .add_systems(Update, 
            (
                title_timer,
                title_anime,
                title_click
            ).run_if(state_exists_and_equals(MenuState::Titel))
        )
        .add_systems(OnExit(MenuState::Titel), 
        (
                title_delete,
            )
        )
        
        ;
    }
}
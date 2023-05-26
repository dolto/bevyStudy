use bevy::prelude::*;

use crate::{score::resources::Score, steat::AppState};

use super::components::{MainMenu, PlayButton};

pub fn spawn_main_menu(
    mut commands : Commands,
    score: Res<Score>,
    asset_server: Res<AssetServer>
){
    commands.spawn(
        (
            NodeBundle {
                style: Style{
                    size: Size::new(Val::Percent(80.0), Val::Percent(80.0)),
                    margin:UiRect {
                        left: Val::Percent(10.0),
                        right: Val::Percent(10.0),
                        top: Val::Percent(10.0),
                        bottom: Val::Percent(10.0)
                    },
                    align_self: AlignSelf::Center,
                    align_items: AlignItems::Center,
                    gap: Size { width: Val::Px(10.0), height: Val::Px(10.0) },
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                background_color: BackgroundColor(Color::rgb(0.8, 0.8, 0.8)),
                ..default()
            },
            MainMenu {}
        )
    ).with_children(
        |parent| {
            let mut count = 0;
            let score_list = score.score_list.clone();

            for score in score_list.iter(){
                if count > 4{
                    break;
                }
                count += 1;
                parent.spawn(
                    NodeBundle{
                        background_color: BackgroundColor(Color::rgb(0.5, 0.5, 0.5)),
                        style: Style{
                            size: Size::new(Val::Percent(80.0), Val::Percent(12.0)),
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        ..default()
                    }
                ).with_children(
                    |parent| {
                        parent.spawn(
                            TextBundle{
                                text: Text{
                                    sections: vec![
                                        TextSection{
                                            value: format!("{:.2}", score),
                                            style: TextStyle{
                                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                                font_size: 64.0,
                                                color: Color::WHITE
                                            }
                                        }
                                    ],
                                    alignment: TextAlignment::Center,
                                    ..default()
                                },
                                ..default()
                            }
                        );
                    }
                )
                
                ;
            }

            parent.spawn(
                (
                    ButtonBundle{
                        style: Style{
                            // margin: UiRect { left: Val::Percent(20.0), 
                            //     right: Val::Percent(20.0), 
                            //     top: Val::Percent(30.0), 
                            //     bottom: Val::Percent(10.0) },
                            size: Size::new(Val::Percent(10.0), Val::Percent(10.0)),
                            ..default()
                        },
                        background_color: BackgroundColor(Color::rgb(0.3, 0.3, 0.3)),
                        ..default()
                    },
                    PlayButton {}
                )
            );
            
        }
    );
}

pub fn despawn_main_menu(
    mut commands : Commands,
    query_main_menu: Query<Entity, With<MainMenu>>
){
    if let Ok(main_menu_entity) = query_main_menu.get_single(){
        commands.entity(main_menu_entity).despawn_recursive();
    }
}

pub fn interaction_with_play_button(
    mut button_query: Query<(&Interaction, &mut BackgroundColor),
    (Changed<Interaction>, With<PlayButton>)>,
    mut app_state_next_state: ResMut<NextState<AppState>>
){
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut(){
        match *interaction {
            Interaction::Clicked => {
                *background_color = BackgroundColor(Color::rgb(0.5, 0.5, 0.5));
                app_state_next_state.set(AppState::Game);
            },
            Interaction::Hovered => {
                *background_color = BackgroundColor(Color::rgb(0.4, 0.4, 0.4));
            },
            Interaction::None => {
                *background_color = BackgroundColor(Color::rgb(0.3, 0.3, 0.3));
            },
        }
    }
}
use bevy::prelude::*;

use crate::main_menu::{components::{MainMenu, PlayButton, QuitButton}, style::*};

pub fn spawn_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>
){
    let main_menu_entity = build_main_menu(&mut commands, &asset_server);
}

pub fn despawn_main_menu(
    mut commands: Commands,
    main_menu_query: Query<Entity, With<MainMenu>>
){
    if let Ok(main_menu_entity) = main_menu_query.get_single() {
        commands.entity(main_menu_entity).despawn_recursive(); //자식까지 제거
    }
}

pub fn build_main_menu(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>
) -> Entity{
    let main_menu_entity = commands.spawn((
        NodeBundle{
        style: MAIN_MENU_STYLE,
        ..default()
    },
    MainMenu {}
    ))
    .with_children(|parent| {
        //타이틀
        parent.spawn(
            NodeBundle{
                style: TITLE_STYLE,
                ..default()
            }
        ).with_children(|parent| {
            //이미지
            parent.spawn(
                ImageBundle {
                    style: TILTE_IMAGE_STAYLE,
                    image: asset_server.load("sprites/ball_blue_large.png").into(),
                    ..default()
                }
            );
            //글
            parent.spawn(
                TextBundle {
                    text: Text {
                        sections: vec![
                            TextSection::new(
                                "Bevy Ball Game",
                                get_title_text_style(&asset_server)
                            )
                        ],
                        alignment: TextAlignment::Center,
                        ..default()
                    },
                    ..default()
                }
            );
            //이미지
            parent.spawn(
                ImageBundle {
                    style: TILTE_IMAGE_STAYLE,
                    image: asset_server.load("sprites/ball_red_large.png").into(),
                    ..default()
                }
            );
        })
        ;
        //플레이버튼
        parent.spawn((
            ButtonBundle{
                style: BUTTON_STYLE,
                background_color: NORMAL_BUTTON_COLOR.into(),
                ..default()
            },
            PlayButton {}
        ))
        .with_children(|parent| {
            parent.spawn(
                TextBundle{
                    text: Text{
                        sections: vec![
                            TextSection::new(
                                "Play",
                                get_button_text_style(&asset_server)
                            )
                        ],
                        alignment: TextAlignment::Center,
                        ..default()
                    },
                    ..default()
                }
            );
        });
        //나가기버튼
        parent.spawn((
            ButtonBundle{
                style: BUTTON_STYLE,
                background_color: NORMAL_BUTTON_COLOR.into(),
                ..default()
            },
            QuitButton {}
        ))
        .with_children(|parent| {
            parent.spawn(
                TextBundle{
                    text: Text{
                        sections: vec![
                            TextSection::new(
                                "Quit",
                                get_button_text_style(&asset_server)
                            )
                        ],
                        alignment: TextAlignment::Center,
                        ..default()
                    },
                    ..default()
                }
            );
        });
    })
    .id(); 

    main_menu_entity
}
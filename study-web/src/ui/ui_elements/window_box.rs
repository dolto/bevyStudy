use std::convert::TryInto;

use bevy::{prelude::*, input::mouse::MouseMotion};
//    mouse: EventReader<MouseMotion>
#[derive(Component)]
pub struct Windows;

#[derive(Component)]
pub struct WindowsMoveButton;

#[derive(Component)]
pub struct RemoveButton;

//안에 요소를 넣고싶으면...
pub fn ui_spawn_window_color(
    commands: &mut ChildBuilder, 
    title: String,
    title_font: Handle<Font>,
    color: Color,
    w: Val,
    h: Val,
    l: Val,
    b: Val
) -> Entity{
    commands.spawn(
        (
            NodeBundle{
                style: Style{
                    width: w,
                    height: h,
                    left: l,
                    bottom: b,
                    position_type: PositionType::Absolute,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                background_color: BackgroundColor(color),
                ..default()
            },
            Windows
        )
    )
    .with_children(|p|{
        p.spawn(
            NodeBundle{
                style: Style{
                    width: Val::Percent(100.),
                    height: Val::Px(15.),
                    justify_content: JustifyContent::SpaceBetween,
                    justify_items: JustifyItems::Stretch,
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                background_color: BackgroundColor(Color::GRAY),
                ..default()
            }
        )
        .with_children(|pp|{
            pp.spawn( //title
                (
                    ButtonBundle{
                        style:Style{
                            right: Val::Px(15.),
                            left: Val::Px(0.),
                            width: Val::Auto,
                            ..default()
                        },
                        ..default()
                    },
                    WindowsMoveButton
                )
            )
            .with_children(|ppp|{
                ppp.spawn(
                    TextBundle::from_section(
                        title, 
                        TextStyle {font_size: 10., color: Color::BLACK, ..default()}
                    ).with_background_color(Color::GRAY)
                );
            })
            ;
            pp.spawn( //remove button
                (
                    ButtonBundle{
                        background_color: BackgroundColor(Color::RED),
                        style: Style{
                            width:Val::Px(15.),
                            height: Val::Px(15.),
                            left:Val::Auto,
                            right: Val::Px(0.),
                            ..default()
                        },
                        ..default()
                    },
                    RemoveButton
                )
            );
        })
        ;
    }).id()
}

pub fn ui_spawn_window_image(
    mut commands: &mut ChildBuilder,
    img: Handle<Image>,
    width: Val,
    height: Val
){
    
}

pub fn test_window_spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>
){
    let f = asset_server.load("fonts/NotoSansKR-VariableFont_wght.ttf");
    let mut e = Entity::from_bits(0);
    commands.spawn(
        NodeBundle{ //이건 작동하는데...
            style: Style{
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                ..default()
            },
            background_color: BackgroundColor(Color::BLACK),
            ..default()
        }
    ).with_children(|p|{
        e = ui_spawn_window_color(p, "Test".to_string(), f.clone(), Color::WHITE, 
        Val::Px(150.), Val::Px(150.), 
        Val::Px(10.), Val::Percent(20.));
    });
    
    commands.entity(e).with_children(|pp|{
        pp.spawn(
            TextBundle::from_section("이런 형태?", TextStyle { font_size: 10., color: Color::BLACK ,..default()})
        );
        ui_spawn_window_color(pp, "Test2".to_string(), f.clone(), Color::BLUE, 
        Val::Px(100.), Val::Px(100.), 
        Val::Px(10.), Val::Px(10.));
    });
}

// pub fn ui_window_move(
//     mut commands: Commands,

//     mouse: EventReader<MouseMotion>
// ){
//     for mut style in window_query.iter_mut(){

//     }
// }
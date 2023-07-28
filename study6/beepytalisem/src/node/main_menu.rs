use std::{fs::{OpenOptions, File}, io::Read};

use bevy::{prelude::*, window::PrimaryWindow, ui::ContentSize};
use serde::{Deserialize, Serialize};

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum MenuState{
    #[default]
    Titel,
    Menu,
    Game
}
#[derive(Component)]
pub struct Title {

}

#[derive(Component)]
pub struct MainTitle {

}

#[derive(Component)]
pub struct MainTitleTourch {

}

#[derive(Resource)]
pub struct MainAnimTime {
    pub prog: f32,
    pub prog_end: f32
}
impl Default for MainAnimTime {
    fn default() -> MainAnimTime {
        MainAnimTime { prog: 0., prog_end: 1.5 }
    }
}

pub fn spawn_title(
    mut commands: Commands,
    //window_query: Query<&Window, With<PrimaryWindow>>
){
    //let win_height = window_query.get_single().unwrap().height();
    //let win_width = window_query.get_single().unwrap().width();
    commands.spawn(
        (
            NodeBundle {
                style: Style{
                    //display: Display::Flex,
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    margin:UiRect::all(Val::Percent(0.)),
                    align_items: AlignItems::Start,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                //transform: Transform::from_xyz(win_width/2., win_height, 0.),
                background_color: BackgroundColor(Color::GRAY),
                ..default()
            },
            Interaction::None,
            Button{ ..default() },
            Title {}
        )
    ).
    with_children(|ppp|{
        ppp.spawn(
            
                NodeBundle {
                    style: Style{
                        width: Val::Px(393.),
                        height: Val::Px(15.),
                        margin:UiRect::all(Val::Percent(0.)),
                        //align_items: AlignItems::FlexEnd,
                        justify_content: JustifyContent::End,
                        ..default()
                    },
                    //background_color: BackgroundColor(Color::WHITE) ,
                    //transform: Transform::from_xyz(win_width/2., win_height, 0.),
                    ..default()
                },
        ).
        with_children(|p|{
            p.spawn(
                TextBundle{
                    style: Style{
                        //width: Val::Percent(100.),
                        //height: Val::Percent(100.),
                        //align_self: AlignSelf::FlexEnd,
                        //justify_self: JustifySelf::End,
                        ..default()
                    },
                    //background_color: BackgroundColor(Color::GOLD),
                    text: Text::from_section(
                        "ver 0.0.1", 
                        TextStyle{
                            font_size: 14.,
                            color: Color::BLACK,
                            ..default()
                        }).with_alignment(TextAlignment::Right),
                    
                    ..default()
                }
            );
        });
        ppp.spawn(
            (
                MainTitle {},
                NodeBundle {
                    style: Style{
                        width: Val::Px(355.),
                        height: Val::Px(200.),
                        margin:UiRect::all(Val::Px(19.)),
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: BackgroundColor(Color::DARK_GRAY) ,
                    //transform: Transform::from_xyz(win_width/2., win_height + 300., 0.),
                    ..default()
                }
            )
        ).
        with_children(|pp|{
            pp.spawn(
                NodeBundle {
                    style: Style{
                        width: Val::Px(345.),
                        height: Val::Px(67.),
                        margin:UiRect::all(Val::Px(5.)),
                        //justify_content:JustifyContent::Center,
                        justify_content:JustifyContent::Center,
                        ..default()
                    },
                    //background_color: BackgroundColor(Color::YELLOW) ,
                    ..default()
                }
            ).
            with_children(|p|{
                p.spawn(
                    TextBundle{
                        //background_color:BackgroundColor(Color::SALMON),
                        text: Text::from_section(
                            "Beepytal", 
                            TextStyle{
                                font_size: 64.,
                                color: Color::ANTIQUE_WHITE,
                                ..default()
                            }).with_alignment(TextAlignment::Center),
                        ..default()
                    }
                );
            });
        });
        ppp.spawn(
            NodeBundle{
                style: Style{
                    width: Val::Percent(100.),
                    height: Val::Px(20.),
                    justify_content: JustifyContent::Center,
                    margin: UiRect::px(0., 0., 285., 0.),
                    ..default()
                },
                ..default()
            }
        )
        .with_children(|p|{
            p.spawn(
                (
                    TextBundle{
                    text: Text::from_section(
                        "tourch to start!", 
                        TextStyle{
                            font_size: 20.,
                            color: Color::DARK_GRAY,
                            ..default()
                        }).with_alignment(TextAlignment::Center),
                        ..default()
                    },
                    MainTitleTourch {}
                )
            );
        });
    });
}

pub fn title_timer(
    mut timer : ResMut<MainAnimTime>,
    time: Res<Time>
){
    timer.prog += time.delta_seconds();
    // if timer.prog > timer.prog_end {
    //     timer.prog -= timer.prog_end;
    // }
}

pub fn title_anime(
    timer : Res<MainAnimTime>,
    mut title_query: Query<&mut Style, With<MainTitle>>,
    mut toruch_query: Query<&mut Text, With<MainTitleTourch>>
){
    let title = &mut title_query.get_single_mut().unwrap();
    let toruch = &mut toruch_query.get_single_mut().unwrap().sections[0];
    let prog = timer.prog;

    title.top = Val::Px(prog.sin() * 10.);
    toruch.style.color.set_a((prog*2.).sin());
}

pub fn title_click(
    event: Query<&mut Interaction, (Changed<Interaction>, With<Button>)>,
    mut menu_state: ResMut<NextState<MenuState>>
){
    for interaction in event.iter() {
        match interaction {
            Interaction::Hovered => {
                println!("is Hovered!");
            }
            Interaction::Pressed => {
                println!("is Pressed!");
                menu_state.set(MenuState::Menu);
            }
            Interaction::None => {
                println!("is None!");
            }
        }
    }
}

pub fn title_delete(
    mut commands: Commands,
    title: Query<Entity, With<Title>>
){
    for t in title.iter() {
        commands.entity(t).despawn_recursive();
    }
}

#[derive(Serialize, Deserialize)]
pub struct SaveData{
    save_name: String
}

#[derive(Component)]
pub struct Menu {}

#[derive(Component)]
pub struct Profile {}

#[derive(Component)]
pub struct SaveDelete{
    pub count: i32
}

#[derive(Component)]
pub struct Items {
    pub number: u64
}

pub fn spawn_menu(
    mut commadns: Commands
){
    let mut save_count = 0;
    let mut save_data: Vec<SaveData> = Vec::new();
    loop {
        let file = OpenOptions::new().read(true).open(format!("./saveData_{}", save_count));
        match file {
            Ok(mut f) => {
                let mut data = String::new() ;
                f.read_to_string(&mut data).unwrap();
                save_data.push(serde_json::from_str(&data).unwrap());
                save_count += 1;
            },
            Err(_) => { break; },
        }
    }

    commadns.spawn(
        (
            NodeBundle{
                style: Style{
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    ..default()
                },
                background_color: BackgroundColor(Color::GRAY),
                ..default()
            },
            Menu {}
        )
    )
    .with_children(|menu_p|{
        menu_p.spawn(
            (
                NodeBundle{
                    style: Style{
                        width: Val::Px(355.),
                        height: Val::Px(700.),
                        margin: UiRect::new(
                            Val::Px(19.), 
                            Val::Px(19.), 
                            Val::Px(58.), 
                            Val::Px(0.)
                        ),
                        overflow: Overflow::clip_x(),
                        flex_direction: FlexDirection::Row,
                        ..default()
                    },
                    background_color: BackgroundColor(Color::Rgba { red: 200. / 255., green: 148. / 255., blue: 70. / 255., alpha: 1. }),
                    ..default()
                },
                Button{}
            )
        )
        .with_children(|profile_p|{

            //여기에 세이브 개수에 따라서 반복 스폰해야함
            profile_p.spawn(
                //2020 77
                //217
                NodeBundle{
                    style: Style{
                        width: Val::Px(342.),
                        height: Val::Px(660.),
                        margin: UiRect { left: Val::Px(7.), right: Val::Px(31.), top: Val::Px(20.), bottom: Val::Px(0.) },
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    background_color: BackgroundColor(Color::Rgba { red: 217. / 255., green: 217. / 255., blue: 217. / 255., alpha: 1. }),
                    ..default()
                }
            )
            .with_children(|p|{
                p.spawn(
                    NodeBundle{
                        style: Style{
                            width: Val::Percent(100.),
                            height: Val::Px(40.),
                            margin: UiRect::top(Val::Px(20.)),
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        //background_color: BackgroundColor(Color::GOLD),
                        ..default()
                    }
                )
                .with_children(|tp|{
                    tp.spawn(
                        TextBundle{
                            text:Text::from_section(
                                "FileName", 
                                TextStyle {font_size: 25., color: Color::BLACK , ..default()}),
                            ..default()
                        }
                    );
                });
                p.spawn(
                    (
                        NodeBundle{
                            style: Style{
                                width: Val::Px(322.),
                                height: Val::Px(322.),
                                margin: UiRect{
                                    top: Val::Px(7.),
                                    left: Val::Auto,
                                    right: Val::Auto,
                                    bottom: Val::Px(0.)
                                },
                                flex_direction: FlexDirection::Column,
                                ..default()
                            },
                            background_color: BackgroundColor(Color::Rgba { red: 199. / 255., green: 111. / 255., blue: 48. / 255., alpha: 1. }),
                            ..default()
                        },
                    )
                )
                .with_children(|p_info|{
                        p_info.spawn(
                            NodeBundle{
                                style:Style{
                                    width: Val::Px(300.),
                                    height: Val::Px(170.),
                                    flex_direction: FlexDirection::Row,
                                    ..default()
                                },
                                ..default()
                            }
                        )
                        .with_children(|p_port_close|{
                            p_port_close.spawn(
                                NodeBundle{
                                    style: Style{
                                        margin: UiRect{
                                            top: Val::Px(15.),
                                            left: Val::Px(10.),
                                            ..default()
                                        },
                                        width: Val::Px(170.),
                                        height: Val::Px(170.),
                                        ..default()
                                    },
                                    background_color: BackgroundColor(Color::Rgba { red: 217. / 255., green: 217. / 255., blue: 217. / 255., alpha: 1. }),
                                    ..default()
                                }
                            );
        
                            p_port_close.spawn(
                                NodeBundle{
                                    style: Style{
                                        margin: UiRect{
                                            top: Val::Px(15.),
                                            left: Val::Px(3.),
                                            ..default()
                                        },
                                        width: Val::Px(124.),
                                        height: Val::Px(170.),
                                        ..default()
                                    },
                                    //background_color: BackgroundColor(Color::GOLD),
                                    ..default()
                                }
                            )
                            .with_children(|p_close|{
                                p_close.spawn(
                                    (
                                        NodeBundle{
                                            style: Style{
                                                position_type: PositionType::Absolute,
                                                width: Val::Px(35.),
                                                height: Val::Px(35.),
                                                top: Val::Px(20.),
                                                left: Val::Px(44.),
                                                ..default()
                                            },
                                            background_color: BackgroundColor(Color::Rgba { red: 217. / 255., green: 217. / 255., blue: 217. / 255., alpha: 1. }),
                                            ..default()
                                        },
                                        Button {},
                                        Items{number: 0}
                                    )
                                );
                                p_close.spawn(
                                    (
                                        NodeBundle{
                                            style: Style{
                                                position_type: PositionType::Absolute,
                                                width: Val::Px(35.),
                                                height: Val::Px(35.),
                                                top: Val::Px(60.),
                                                left: Val::Px(44.),
                                                ..default()
                                            },
                                            background_color: BackgroundColor(Color::Rgba { red: 217. / 255., green: 217. / 255., blue: 217. / 255., alpha: 1. }),
                                            ..default()
                                        },
                                        Button {},
                                        Items{number: 0}
                                    )
                                );
                                p_close.spawn(
                                    (
                                        NodeBundle{
                                            style: Style{
                                                position_type: PositionType::Absolute,
                                                width: Val::Px(35.),
                                                height: Val::Px(35.),
                                                top: Val::Px(60.),
                                                left: Val::Px(4.),
                                                ..default()
                                            },
                                            background_color: BackgroundColor(Color::Rgba { red: 217. / 255., green: 217. / 255., blue: 217. / 255., alpha: 1. }),
                                            ..default()
                                        },
                                        Button {},
                                        Items{number: 0}
                                    )
                                );
                                p_close.spawn(
                                    (
                                        NodeBundle{
                                            style: Style{
                                                position_type: PositionType::Absolute,
                                                width: Val::Px(35.),
                                                height: Val::Px(35.),
                                                top: Val::Px(60.),
                                                left: Val::Px(84.),
                                                ..default()
                                            },
                                            background_color: BackgroundColor(Color::Rgba { red: 217. / 255., green: 217. / 255., blue: 217. / 255., alpha: 1. }),
                                            ..default()
                                        },
                                        Button {},
                                        Items{number: 0}
                                    )
                                );
                                p_close.spawn(
                                    (
                                        NodeBundle{
                                            style: Style{
                                                position_type: PositionType::Absolute,
                                                width: Val::Px(35.),
                                                height: Val::Px(35.),
                                                top: Val::Px(100.),
                                                left: Val::Px(44.),
                                                ..default()
                                            },
                                            background_color: BackgroundColor(Color::Rgba { red: 217. / 255., green: 217. / 255., blue: 217. / 255., alpha: 1. }),
                                            ..default()
                                        },
                                        Button {},
                                        Items{number: 0}
                                    )
                                );
                                p_close.spawn(
                                    (
                                        NodeBundle{
                                            style: Style{
                                                position_type: PositionType::Absolute,
                                                width: Val::Px(35.),
                                                height: Val::Px(35.),
                                                top: Val::Px(113.),
                                                left: Val::Px(0.),
                                                ..default()
                                            },
                                            background_color: BackgroundColor(Color::Rgba { red: 217. / 255., green: 217. / 255., blue: 217. / 255., alpha: 1. }),
                                            ..default()
                                        },
                                        Button {},
                                        Items{number: 0}
                                    )
                                );
                                p_close.spawn(
                                    (
                                        NodeBundle{
                                            style: Style{
                                                position_type: PositionType::Absolute,
                                                width: Val::Px(35.),
                                                height: Val::Px(35.),
                                                top: Val::Px(113.),
                                                right: Val::Px(0.),
                                                ..default()
                                            },
                                            background_color: BackgroundColor(Color::Rgba { red: 217. / 255., green: 217. / 255., blue: 217. / 255., alpha: 1. }),
                                            ..default()
                                        },
                                        Button {},
                                        Items{number: 0}
                                    )
                                );
                            });
                        });
                        p_info.spawn(
                            NodeBundle{
                                style: Style{
                                    width: Val::Px(300.),
                                    height: Val::Px(122.),
                                    margin: UiRect{
                                        left:Val::Px(10.),
                                        right:Val::Auto,
                                        top: Val::Px(20.),
                                        bottom: Val::Px(0.)
                                    },
                                    ..default()
                                },
                                background_color: BackgroundColor(Color::Rgba { red: 217. / 255., green: 217. / 255., blue: 217. / 255., alpha: 1. }),
                                ..default()
                            }
                        )
                        .with_children(|p_state_trait|{
                            p_state_trait.spawn(
                                (
                                    NodeBundle{
                                        style: Style{
                                            width: Val::Px(80.),
                                            height: Val::Px(111.),
                                            margin: UiRect{
                                                left: Val::Px(7.),
                                                top:Val::Auto,
                                                bottom: Val::Auto,
                                                right: Val::Auto
                                            },
                                            ..default()
                                        },
                                        background_color: BackgroundColor(Color::Rgba { red: 200./255., green: 148./255., blue: 70./255., alpha: 1. }),
                                        ..default()
                                    },
                                )
                            );
                            p_state_trait.spawn(
                                (
                                    NodeBundle{
                                        style: Style{
                                            width: Val::Px(200.),
                                            height: Val::Px(111.),
                                            margin: UiRect{
                                                right: Val::Px(7.),
                                                top:Val::Auto,
                                                bottom: Val::Auto,
                                                left: Val::Auto
                                            },
                                            ..default()
                                        },
                                        background_color: BackgroundColor(Color::Rgba { red: 200./255., green: 148./255., blue: 70./255., alpha: 1. }),
                                        ..default()
                                    },
                                )
                            );
                        });
                    });
                p.spawn(
                    ButtonBundle{
                        style:Style{
                            margin: UiRect{
                                left:Val::Auto,
                                right: Val::Auto,
                                top: Val::Px(7.),
                                bottom: Val::Auto
                            },
                            width: Val::Px(322.),
                            height: Val::Px(200.),
                            ..default()
                        },
                        background_color: BackgroundColor(Color::Rgba { red: 237./255., green: 133./255., blue: 133./255., alpha: 1. }),
                        ..default()
                    }
                );
                    
                p.spawn( //제거 버튼
                    (
                        NodeBundle{
                            style: Style{
                                position_type: PositionType::Absolute,
                                width: Val::Px(100.),
                                height: Val::Px(100.), 
                                top: Val::Px(0.),
                                left: Val::Px(242.),
                                ..default()
                            },
                            background_color: BackgroundColor(Color::Rgba { red: 236. / 255., green: 102. / 255., blue: 102. / 255., alpha: 1. }),
                            ..default()
                        },
                        Button {},
                        SaveDelete { count: 0}
                    )
                );
            });
        });
    });
}
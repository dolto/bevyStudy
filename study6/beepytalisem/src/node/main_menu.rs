use bevy::{prelude::*, window::PrimaryWindow, ui::ContentSize};

#[derive(Component)]
pub struct MainTitle {

}

#[derive(Component)]
pub struct MainTourch {

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

pub fn menu(
    mut commands: Commands,
    //window_query: Query<&Window, With<PrimaryWindow>>
){
    //let win_height = window_query.get_single().unwrap().height();
    //let win_width = window_query.get_single().unwrap().width();
    commands.spawn(
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
        }
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
                    MainTourch {}
                )
            );
        });
    });
}

pub fn anim_timer(
    mut timer : ResMut<MainAnimTime>,
    time: Res<Time>
){
    timer.prog += time.delta_seconds();
    // if timer.prog > timer.prog_end {
    //     timer.prog -= timer.prog_end;
    // }
}

pub fn menu_anime(
    timer : Res<MainAnimTime>,
    mut title_query: Query<&mut Style, With<MainTitle>>,
    mut toruch_query: Query<&mut Text, With<MainTourch>>
){
    let title = &mut title_query.get_single_mut().unwrap();
    let toruch = &mut toruch_query.get_single_mut().unwrap().sections[0];
    let prog = timer.prog;

    title.top = Val::Px(prog.sin() * 10.);
    toruch.style.color.set_a((prog*2.).sin());
}
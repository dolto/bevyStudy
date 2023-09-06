use bevy::{prelude::*, input::mouse::MouseMotion};
use wasm_bindgen::JsValue;
use web_sys::console;
//    mouse: EventReader<MouseMotion>

#[derive(Component)]
pub struct Windows{
    pub is_move: bool
}

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
    t: Val
) -> Entity{
    commands.spawn(
        (
            NodeBundle{
                style: Style{
                    width: w,
                    height: h,
                    left: l,
                    top: t,
                    position_type: PositionType::Absolute,
                    flex_direction: FlexDirection::Column,
                    overflow: Overflow::clip(),
                    
                    ..default()
                },
                z_index: ZIndex::Local(0),
                background_color: BackgroundColor(color),
                ..default()
            },
            Windows {is_move: false}
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
                            width: Val::Percent(100.),
                            ..default()
                        },
                        background_color: BackgroundColor(Color::GRAY),
                        ..default()
                    },
                    WindowsMoveButton
                )
            )
            .with_children(|ppp|{
                ppp.spawn(
                    TextBundle::from_section(
                        title, 
                        TextStyle {font: title_font,font_size: 13., color: Color::BLACK}
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
    title: String,
    title_font: Handle<Font>,
    img: Handle<Image>,
    w: Val,
    h: Val,
    l: Val,
    t: Val
) -> Entity{
    commands.spawn(
        (
            ImageBundle{
                style: Style{
                    width: w,
                    height: h,
                    left: l,
                    top: t,
                    position_type: PositionType::Absolute,
                    flex_direction: FlexDirection::Column,
                    overflow: Overflow::clip(),
                    ..default()
                },
                image: UiImage { texture: img, flip_x: false, flip_y: false },
                ..default()
            },
            Windows {is_move: false}
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
                            width: Val::Percent(100.),
                            ..default()
                        },
                        background_color: BackgroundColor(Color::GRAY),
                        ..default()
                    },
                    WindowsMoveButton
                )
            )
            .with_children(|ppp|{
                ppp.spawn(
                    TextBundle::from_section(
                        title, 
                        TextStyle {font: title_font,font_size: 13., color: Color::BLACK}
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

pub fn test_window_spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>
){
    let f = asset_server.load("fonts/ChosunCentennial.ttf");
    let mut e = Entity::from_bits(0);
    commands.spawn(
        NodeBundle{
            style: Style{
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                ..default()
            },
            //background_color: BackgroundColor(Color::BLACK),
            ..default()
        }
    ).with_children(|p|{
        e = ui_spawn_window_color(p, "테스트".to_string(), f.clone(), Color::WHITE, 
        Val::Px(150.), Val::Px(150.), 
        Val::Px(10.), Val::Px(200.));

        ui_spawn_window_color(p, "어쩼든 해결!!".to_string(), f.clone(), Color::ORANGE, 
        Val::Px(150.), Val::Px(150.), 
        Val::Px(100.), Val::Px(200.));
    });
    
    commands.entity(e).with_children(|pp|{
        pp.spawn(
            TextBundle::from_section("this is test", TextStyle { font_size: 10., color: Color::BLACK ,..default()})
        );
        ui_spawn_window_color(pp, "Test2".to_string(), f.clone(), Color::BLUE, 
        Val::Px(100.), Val::Px(100.), 
        Val::Px(10.), Val::Px(10.));
    });
}

pub fn ui_window_remove(
    mut commands: Commands,
    query_child_title: Query<(&Parent, &Interaction), (With<RemoveButton>, Changed<Interaction>)>,
    query_middle_node: Query<&Parent, With<Node>>,
    query_parent_windows: Query<Entity, With<Windows>>,
){
    for (parent, button) in query_child_title.iter() {
        match button {
            Interaction::Pressed => {
                let middle_node = query_middle_node.get(parent.get()).unwrap();
                let window = query_parent_windows.get(middle_node.get()).unwrap();

                //console::log_1(&JsValue::from_str(format!("종료버튼 누름").as_str()));

                commands.entity(window).despawn_recursive();
            },
            _  => {
            }
        }
    }
}

pub fn ui_window_move_triger(
    //mut commands: Commands,
    query_child_title: Query<(&Parent, &Interaction), (With<WindowsMoveButton>, Changed<Interaction>)>,
    query_middle_node: Query<&Parent, With<Node>>,
    mut query_parent_windows: Query<(&mut Windows, &mut ZIndex)>,
){
    for (parent, button) in query_child_title.iter() {
        match button {
            Interaction::Pressed => {
                for (_, mut z) in query_parent_windows.iter_mut(){
                    *z = ZIndex::Local(0);
                }
                let middle_node = query_middle_node.get(parent.get()).unwrap();
                let (mut window, mut w_index) = query_parent_windows.get_mut(middle_node.get()).unwrap();

                window.is_move = true;
                *w_index = ZIndex::Local(1);
                //console::log_1(&JsValue::from_str(format!("타이틀 클릭! {}", window.is_move).as_str()));
            },
            _  => {
                let middle_node = query_middle_node.get(parent.get()).unwrap();
                let (mut window, _) = query_parent_windows.get_mut(middle_node.get()).unwrap();
                //console::log_1(&JsValue::from_str(format!("타이틀 벗어남! {}", window.is_move).as_str()));

                window.is_move = false;
            }
        }
    }
}

pub fn ui_window_move(
    mut query_windows: Query<(&mut Style, &Windows)>,
    mut mouse: EventReader<MouseMotion>
){
    let mut ev_x = 0.;
    let mut ev_y = 0.;
    for ev in mouse.iter(){
        ev_x = ev.delta.x;
        ev_y = ev.delta.y;
    }
    for (mut style, window) in query_windows.iter_mut(){
        if window.is_move {
            let left = style.left.try_add(Val::Px(ev_x)).unwrap();
            let top = style.top.try_add(Val::Px(ev_y)).unwrap();

            style.left = left;
            style.top = top;
            
            //console::log_1(&JsValue::from_str(format!("x: {:?}, y: {:?}", left, top).as_str()));
        }
    }
}

// fn camera_with_parent(
//     q_child: Query<(&Parent, &Transform), With<Camera>>,
//     q_parent: Query<&GlobalTransform>,
// ) {
//     for (parent, child_transform) in q_child.iter() {
//         // `parent` contains the Entity ID we can use
//         // to query components from the parent:
//         let parent_global_transform = q_parent.get(parent.get());

//         // do something with the components
//     }
// } //자식으로 부모를 찾는 방법
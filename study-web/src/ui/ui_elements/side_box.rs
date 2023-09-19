use bevy::{prelude::*};
use wasm_bindgen::JsValue;
use web_sys::console;

pub enum SideBoxDirection {
    Up,
    Down,
    Left,
    Right
}

pub enum SideBoxPosition {
    Start, End, Center
}

#[derive(Component)]
pub struct SideBox{
    pub is_row: bool, //양 옆에 있는 사이드라면 true, 위아래면 false
    pub max_size: f32,
    pub save_max_size: f32,
    pub size: f32,
    pub is_dead: bool,
    pub is_closed: bool,
    pub time: f32
}

#[derive(Component)]
pub struct SideBoxElement;

#[derive(Component)]
pub struct SideBoxCloseButton;

pub fn ui_spawn_sidebox(
    commands: &mut ChildBuilder, 
    direction: SideBoxDirection,
    position: SideBoxPosition,
    mut h: f32,
    mut w: f32,
    color: Color,
    death: f32
) -> Entity{
    let just_direction: FlexDirection;
    let mut side_box: SideBox = SideBox { is_row: true, max_size: w, save_max_size: w, size: 5.5, is_dead: false, is_closed: false, time: 0. };
    let mut style: Style = Style{..default()};
    let button_style: Style;
    match direction {
        SideBoxDirection::Up => {
            just_direction = FlexDirection::Row;
            side_box.is_row = false;
            side_box.max_size = h;
            side_box.save_max_size = h;
            //t = Val::Px(0.);
            //b = Val::Px(h * -1.);
            style = Style{
                position_type:PositionType::Absolute,
                flex_direction: just_direction,
                width: Val::Percent(100.),
                height: Val::Px(5.5),
                top: Val::Px(0.),
                overflow: Overflow::clip(),
                ..default()
            };
            button_style = Style{
                position_type:PositionType::Absolute,
                right: Val::Px(0.),
                top: Val::Px(0.),
                width: Val::Px(5.),
                height: Val::Px(5.),
                ..default()
            }
            //h = 0.;
        },
        SideBoxDirection::Down => {
            just_direction = FlexDirection::Row;
            side_box.is_row = false;
            side_box.max_size = h;
            side_box.save_max_size = h;
            //b = Val::Px(0.);
            //t = Val::Px(h * -1.);
            style = Style{
                position_type:PositionType::Absolute,
                flex_direction: just_direction,
                align_items: AlignItems::End,
                width: Val::Percent(100.),
                height: Val::Px(5.5),
                bottom: Val::Px(0.),
                overflow: Overflow::clip(),
                ..default()
            };
            button_style = Style{
                position_type:PositionType::Absolute,
                right: Val::Px(0.),
                bottom: Val::Px(0.),
                width: Val::Px(5.),
                height: Val::Px(5.),
                ..default()
            }
            //h = 0.;
        },
        SideBoxDirection::Left => {
            just_direction = FlexDirection::Column;
            //l = Val::Px(0.);
            //r = Val::Px(w * -1.);
            style = Style{
                position_type:PositionType::Absolute,
                flex_direction: just_direction,
                width: Val::Px(5.5),
                height: Val::Percent(100.),
                left: Val::Px(0.),
                overflow: Overflow::clip(),
                ..default()
            };
            button_style = Style{
                position_type:PositionType::Absolute,
                left: Val::Px(0.),
                top: Val::Px(0.),
                width: Val::Px(5.),
                height: Val::Px(5.),
                ..default()
            }
            //w = 0.;
        },
        SideBoxDirection::Right => {
            just_direction = FlexDirection::Column;
            //r = Val::Px(0.);
            //l = Val::Px(w * -1.);
            style = Style{
                position_type:PositionType::Absolute,
                flex_direction: just_direction,
                align_items: AlignItems::End,
                width: Val::Px(5.5),
                height: Val::Percent(100.),
                right: Val::Px(0.),
                overflow: Overflow::clip(),
                ..default()
            };
            button_style = Style{
                position_type:PositionType::Absolute,
                right: Val::Px(0.),
                top: Val::Px(0.),
                width: Val::Px(5.),
                height: Val::Px(5.),
                ..default()
            }
            //w = 0.;
        },
    }
    match position {
        SideBoxPosition::Start => {
            //just_content = JustifyContent::FlexStart;
            style.justify_content = JustifyContent::FlexStart;
        },
        SideBoxPosition::End => {
            style.justify_content = JustifyContent::FlexEnd;
        },
        SideBoxPosition::Center => {
            style.justify_content = JustifyContent::Center;
        },
    }
    if death > 0. {
        side_box.is_dead = true;
        side_box.time = death;
    }
    else{
        side_box.is_dead = false;
        side_box.time = 0.;
    }

    let mut element = Entity::from_bits(0);
    commands.spawn(
      (
        side_box,
        NodeBundle {
            style,
            //background_color: BackgroundColor(Color::GOLD),
            ..default()
        }
      )
    )
    .with_children(
        |p| {
            element = p.spawn(
                (
                    SideBoxElement,
                    NodeBundle {
                        style: Style{
                            width: Val::Px(w),
                            height: Val::Px(h),
                            // top: t,
                            // bottom: b,
                            // left: l,
                            // right: r,
                            overflow:Overflow::clip(),
                            position_type: PositionType::Relative,
                            ..default()
                        },
                        background_color: BackgroundColor(color),
                        ..default()
                    }
                )
            )
            .with_children(|pp|{
                pp.spawn(
                    (
                        SideBoxCloseButton,
                        ButtonBundle{
                            style: button_style,
                            background_color: BackgroundColor(Color::ORANGE_RED),
                            ..default()
                        }
                    )
                );
            })
            .id();
        }
    );

    return element;
}

pub fn ui_sidebox_toggle(
    mut query_sidebox: Query<&mut SideBox>,
    query_element: Query<&Parent, With<SideBoxElement>>,
    query_button: Query<(&Parent, &Interaction), (With<SideBoxCloseButton>, Changed<Interaction>)>
){
    for (element, close_button) in query_button.iter(){
        match close_button {
            Interaction::Pressed => {
                let p_ele =  query_element.get(element.get()).unwrap();
                let mut sidebox = query_sidebox.get_mut(p_ele.get()).unwrap();
                
                if sidebox.time < 0. {
                    continue;
                }
                console::log_1(&JsValue::from_str("닫기버튼 누름 성공"));
                if sidebox.is_closed {
                    sidebox.max_size = sidebox.save_max_size;
                    sidebox.is_closed = false;
                }else {
                    sidebox.max_size = 5.;
                    sidebox.is_closed = true;
                }
            },
            _ => {}
        }
    }
}

pub fn ui_sidebox_anim(
    time: Res<Time>,
    mut query_sidebox: Query<(&mut SideBox, &mut Style)>
){
    //console::log_1(&JsValue::from_str(format!("{}", time.delta_seconds()).as_str()));
    let delta = time.delta_seconds();
    for (mut sidebox, mut style) in query_sidebox.iter_mut() {
        if sidebox.is_dead{
            sidebox.time -= delta;
            if sidebox.time < 0. {
                sidebox.is_closed = true;
            }
        }

        if sidebox.is_closed {
            sidebox.max_size = 5.;
        }
        else {
            sidebox.max_size = sidebox.save_max_size;
        }
        
        sidebox.size += (sidebox.max_size - sidebox.size) * delta * 2.;
        match sidebox.is_row {
            true => {
                style.width = Val::Px(sidebox.size);
            },
            false => {
                style.height = Val::Px(sidebox.size);
            }
        }
    }
}

pub fn ui_sidebox_close(
    mut commands: Commands,
    query_sidebox_entity: Query<(Entity, &SideBox)>
    //query: Query<Entity, With<SideBox>> //This also has an error
){
    for (entity, sidebox) in query_sidebox_entity.iter() {
        if sidebox.is_dead {
            if sidebox.size <= 5.1 {
                console::log_1(&JsValue::from_str("entity를 죽이기 위해서..."));
                commands.entity(entity).despawn_recursive();
            }
        }
    }

    // for entity in query.iter(){
    //     commands.entity(entity).despawn(); //This also has an error
    // }
}

pub fn ui_test_sidebox(
    mut commands: Commands,
    fonts: Res<AssetServer>
){
    let mut sidebox = Entity::from_bits(0);
    commands.spawn(
        NodeBundle{
            style: Style{
                width: Val::Percent(100.),
                height: Val::Percent(50.),
                margin: UiRect::top(Val::Percent(60.)),
                overflow: Overflow::visible(),
                ..default()
            },
            background_color: BackgroundColor(Color::BLACK),
            ..default()
        }
    )
    .with_children(|p| {
        ui_spawn_sidebox(p, 
            SideBoxDirection::Left, 
            SideBoxPosition::Center, 
            300., 50., Color::RED, 0.);
        ui_spawn_sidebox(p, 
            SideBoxDirection::Up, 
            SideBoxPosition::Center, 
            40., 300., Color::OLIVE, 0.);
        ui_spawn_sidebox(p, 
            SideBoxDirection::Down, 
            SideBoxPosition::End, 
            100., 200., Color::AQUAMARINE, 0.);
        sidebox = ui_spawn_sidebox(p, 
            SideBoxDirection::Down, 
            SideBoxPosition::Center, 
            50., 250., Color::ORANGE, 0.);
        ui_spawn_sidebox(p, 
            SideBoxDirection::Right, 
            SideBoxPosition::Center, 
            300., 50., Color::GREEN, 0.);
    });

    commands.entity(sidebox).with_children(|p|{
        p.spawn(
            NodeBundle {
                style: Style{
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            }
        ). with_children(|text|{
            text.spawn(
                TextBundle::from_section(
                    "테스트하는 중입니다. 히오스 좋아.", 
                    TextStyle{
                        font_size: 20.,
                        font: fonts.load("fonts/ChosunCentennial.ttf"),
                        color: Color::BLACK
                    }
                )
            );
        });
    });
}
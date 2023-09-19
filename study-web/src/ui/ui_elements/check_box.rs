use bevy::{prelude::*, window::PrimaryWindow, input::{keyboard::KeyboardInput, ButtonState}};
use wasm_bindgen::JsValue;
use web_sys::console;

#[derive(Component)]
pub struct CheckBox{
    text: String,
    checked: bool
}
#[derive(Component)]
pub struct CheckBoxBase;
#[derive(Component)]
pub struct CheckBoxVisible;

pub fn ui_spawn_check_box(
    commands: &mut ChildBuilder,
    text: String,
    checked: bool,
    font: Handle<Font>,
    text_color: Color
){
    let check = if checked {Visibility::Visible} else {Visibility::Hidden};
    commands.spawn(
        (
            NodeBundle{
                style: Style{
                    width: Val::Auto,
                    height: Val::Px(16.),
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                ..default()
            },
            CheckBox {checked: checked.clone(), text: text.clone()}
        )
    ).
    with_children(|base|{
        base.spawn(
            (
                ButtonBundle{
                    background_color: BackgroundColor(Color::WHITE),
                    style: Style{
                        width: Val::Px(16.),
                        height: Val::Px(16.),
                        margin: UiRect::right(Val::Px(5.)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                },
                CheckBoxBase
            )
        ).with_children(|check_box|{
            check_box.spawn(
                    (
                        NodeBundle{
                        background_color: BackgroundColor(Color::BLACK),
                        visibility: check,
                        style: Style{
                            width: Val::Px(10.),
                            height: Val::Px(10.),
                            ..default()
                        },
                        ..default()
                    },
                    CheckBoxVisible
                )
            );
        });

        base.spawn(
            TextBundle::from_section(
                text, 
                TextStyle { font: font, font_size: 14., color: text_color }
            )
        );
    });
}

pub fn ui_checkbox_click(
    mut query_checkbox: Query<&mut CheckBox>,
    query_checkbox_base: Query<(&Parent, &Children, &Interaction), (With<CheckBoxBase>, Changed<Interaction>)>,
    mut query_checkbox_visible: Query<&mut Visibility, With<CheckBoxVisible>>
){
    for (parent, children ,interaction) in query_checkbox_base.iter(){
        match interaction {
            Interaction::Pressed => {
                let mut checkbox = query_checkbox.get_mut(parent.get()).unwrap();
                let mut visible = query_checkbox_visible.get_mut(children[0]).unwrap();
                checkbox.checked = !checkbox.checked;
                *visible = if checkbox.checked {Visibility::Visible} else {Visibility::Hidden};
            },
            _ => {},
        }
    }
}

pub fn ui_test_checkbox(
    mut commands: Commands,
    fonts: Res<AssetServer>
){
    let font: Handle<Font> = fonts.load("fonts/ChosunCentennial.ttf");

    //let mut checkbox = Entity::from_bits(0);
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
    ).with_children(|p|{
        ui_spawn_check_box(p, "test check box1".to_string(), false, font.clone(), Color::WHITE);
        ui_spawn_check_box(p, "test check box2".to_string(), false, font.clone(), Color::BLUE);
        ui_spawn_check_box(p, "test check box3".to_string(), false, font.clone(), Color::GREEN);
    });
}
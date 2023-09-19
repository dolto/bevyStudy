use bevy::{prelude::*, window::PrimaryWindow, input::{keyboard::KeyboardInput, ButtonState}};
use wasm_bindgen::JsValue;
use web_sys::console;

#[derive(Resource)]
pub struct TextBoxFocus{
    pub activity: bool,
    pub focus: Entity,
    pub capslock: bool,
}

#[derive(Component)]
pub struct TextBox{
    enable: bool,
    max_textsize: usize
}

pub fn ui_spawn_textbox(
    commands: &mut ChildBuilder, 
    font: Handle<Font>,
    max_text_size: usize,
    text_color: Color,
    text_size: f32,
    background: Color,
    enable: bool,
    default_text: String,
    w: Val,
    h: Val
) -> Entity{
    let text = default_text;
    commands.spawn(
        (
            TextBundle::from_section(text, 
                TextStyle { font, font_size: text_size, color: text_color }). with_style(
                    Style{
                        width:w,
                        height:h,
                        ..default()}
                    ).with_background_color(background),
            TextBox{
                enable,
                max_textsize: max_text_size,
            },
            Button, Interaction::None
          )
    ).id()
}

pub fn ui_toggle_ime(
    mut q_window: Query<&mut Window, With<PrimaryWindow>>,
    focus: Res<TextBoxFocus>
){
    let mut window = q_window.single_mut();
    if focus.activity{
        if !window.ime_enabled{
            window.ime_enabled = true;
            console::log_1(&JsValue::from_str(format!("ime활성화 {}", window.ime_enabled).as_str()));
            window.ime_position = Vec2::new(window.width() / 2., window.height() / 2.);
        }
    }
    else{
        if window.ime_enabled{
            window.ime_enabled = false;
            console::log_1(&JsValue::from_str(format!("ime비활성화 {}", window.ime_enabled).as_str()));
            window.ime_position = Vec2::new(0., 0.);
        }
    }
}

pub fn ui_textbox_input(
    mut query_text_box: Query<(&TextBox, &mut Text, Entity)>,
    focus: Res<TextBoxFocus>,
    mut input_key_event: EventReader<Ime>
){
    let mut v = String::new();
    for key in input_key_event.iter(){
        match key {
            Ime::Preedit {value, ..} => {
                console::log_1(&JsValue::from_str(format!("입력중이다 {}", value).as_str()));
            },
            Ime::Commit { value, .. } => {
                console::log_1(&JsValue::from_str(format!("입력하다 {}", value).as_str()));
                v.push_str(&value);
            },
            Ime::Enabled { ..} => {console::log_1(&JsValue::from_str(format!("활성화 중이다").as_str()));},
            Ime::Disabled { ..} => {console::log_1(&JsValue::from_str(format!("비활성화 중이다").as_str()));},
        }
    }
    for (text_box, mut secction, entity) in query_text_box.iter_mut(){
        if text_box.enable && focus.activity {
            if focus.focus == entity{
                let value = &mut secction.sections[0].value;
                if value.len() < text_box.max_textsize{
                    console::log_1(&JsValue::from_str(format!("입력반영 시도 {}", v.clone()).as_str()));
                    value.push_str(v.clone().as_str());
                    break;
                }
            }
        }
    }
}

pub fn ui_textbox_input_without_ime(
    mut query_text_box: Query<(&TextBox, &mut Text, Entity)>,
    focus: Res<TextBoxFocus>,
    keycode: Res<Input<KeyCode>>,
    mut keyboard_evnt: EventReader<ReceivedCharacter>
){
    let mut v: Vec<char> = Vec::with_capacity(4);
    for key in keyboard_evnt.iter(){
        let k = key.char;
        v.push(k);
    }
    for (textbox, mut text, entity) in query_text_box.iter_mut(){
        if textbox.enable && focus.activity {
            if focus.focus == entity{
                if keycode.just_pressed(KeyCode::Return) {
                    text.sections[0].value.push_str("\n");
                }
                else if keycode.just_pressed(KeyCode::Back) {
                    text.sections[0].value.pop();
                }
                else {
                    for value in v.iter(){
                        let ch = value.clone();
                        console::log_1(&JsValue::from_str(format!("입력하다 {}", ch).as_str()));
                        text.sections[0].value.push(ch);
                    }
                }
            }
        }
    }
}

pub fn ui_textbox_set_focus(
    mut query_text_box: Query<(&Interaction, Entity), (Changed<Interaction>, With<TextBox>)>,
    mouse_click: Res<Input<MouseButton>>,
    mut focus: ResMut<TextBoxFocus>
){
    if mouse_click.just_pressed(MouseButton::Left){
        let mut f = false;
        for (inter, entity) in query_text_box.iter_mut(){
            match inter {
                Interaction::Pressed => {
                    console::log_1(&JsValue::from_str("text box포커스 활성화"));
                    focus.activity = true;
                    focus.focus = entity;
                    f = true;
                    continue;
                },
                _ => {}
            }
        }
        if !f{
            focus.activity = false;
        }
    }
}

pub fn ui_test_textbox(
    mut commands: Commands,
    assets: Res<AssetServer>
){
    let font = assets.load("fonts/ChosunCentennial.ttf");

    commands.spawn(
        NodeBundle{
            style:Style{
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                ..default()
            },
            ..default()
        }
    ).with_children(|p|{
        ui_spawn_textbox(p, font, 100, 
            Color::BLACK, 20., Color::WHITE, 
            true, "텍스트 입력: ".to_string(), Val::Percent(100.), Val::Px(500.));
    });
}
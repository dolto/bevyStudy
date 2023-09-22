use bevy::{prelude::*, window::PrimaryWindow, input::{keyboard::KeyboardInput, ButtonState}};
use wasm_bindgen::JsValue;
use web_sys::console;

#[derive(Component)]
pub struct CheckBox{
    //text: String,
    checked: bool
}
#[derive(Component)]
pub struct CheckBoxBase;
#[derive(Component)]
pub struct CheckBoxVisible;

#[derive(Component)]
pub struct RadioGroup;

#[derive(Event)]
pub struct CheckBoxChecked{
    checkbox_entity: Entity,
    group_entity: Entity
}
#[derive(Event)]
pub struct CheckBoxSet;

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
            CheckBox {checked: checked.clone()}
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
    mut query_checkbox: Query<(&Parent ,&mut CheckBox, Entity)>,
    query_checkbox_base: Query<(&Parent, &Interaction), (With<CheckBoxBase>, Changed<Interaction>)>,
    mut checked: EventWriter<CheckBoxChecked>,
    mut set: EventWriter<CheckBoxSet>
){
    for (parent, interaction) in query_checkbox_base.iter(){
        match interaction {
            Interaction::Pressed => {
                let (check_p, mut checkbox, entity) = query_checkbox.get_mut(parent.get()).unwrap();
                checkbox.checked = !checkbox.checked;
                if checkbox.checked{
                    checked.send(CheckBoxChecked { checkbox_entity: entity, group_entity: check_p.get() });
                }
                set.send(CheckBoxSet);
            },
            _ => {},
        }
    }
}

pub fn ui_checkbox_set_event(
    query_checkbox: Query<(&Children, &CheckBox)>,
    query_checkbox_base: Query<&Children, With<CheckBoxBase>>,
    mut query_checkbox_visible: Query<&mut Visibility>,
    mut set_ev: EventReader<CheckBoxSet>
){
    for _ in set_ev.iter(){
        for (child,checkbox) in query_checkbox.iter(){
            let mut base;
            let mut visible;
            for c in child.iter(){
                base = query_checkbox_base.get(c.clone());
                if base.is_ok(){
                    visible = query_checkbox_visible.get_mut(base.unwrap()[0]).unwrap();
                    *visible = if checkbox.checked {
                        Visibility::Visible
                    } else {Visibility::Hidden};
                    break;
                }
            }
            
        }
    }

}

pub fn ui_cehckbox_radio_event(
    query_radio_group: Query<&Children, With<RadioGroup>>,
    mut query_checkbox: Query<&mut CheckBox>,
    mut checked_events: EventReader<CheckBoxChecked>,
    mut set: EventWriter<CheckBoxSet>
){
    for ev in checked_events.iter(){
        let radio_group = query_radio_group.get(ev.group_entity);
        if radio_group.is_ok(){
            let _radio_group = radio_group.unwrap();
            for ch in _radio_group.iter(){
                if ch.index() != ev.checkbox_entity.index(){
                    let checkbox = query_checkbox.get_mut(ch.clone());
                    if checkbox.is_ok(){
                        let mut _checkbox = checkbox.unwrap();
                        _checkbox.checked = false;
                    }
                }
            }
        }

        set.send(CheckBoxSet);
    }
}

// pub fn ui_checkbox_change_text(
//     mut query_checkbox: Query<(&CheckBox, &mut Text), Changed<CheckBox>>
// ){
//     for (checkbox, mut text) in query_checkbox.iter_mut(){
//         text.sections[0].value = checkbox.text.clone(); //최적화에 문제가 될 수도 있기 때문에 동기화는 굳이 안함
//     }
// }

pub fn ui_test_checkbox(
    mut commands: Commands,
    fonts: Res<AssetServer>
){
    let font: Handle<Font> = fonts.load("fonts/ChosunCentennial.ttf");

    //let mut checkbox = Entity::from_bits(0);
    commands.spawn(
        (
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
            },
            RadioGroup
        )
        
    ).with_children(|p|{
        ui_spawn_check_box(p, "test Radio box1".to_string(), false,font.clone(), Color::WHITE);
        ui_spawn_check_box(p, "test Radio box2".to_string(), false,font.clone(), Color::BLUE);
        ui_spawn_check_box(p, "test Radio box3".to_string(), false,font.clone(), Color::GREEN);
    });
}


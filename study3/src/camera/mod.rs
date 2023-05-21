use bevy::{prelude::*, window::PrimaryWindow, core_pipeline::clear_color::ClearColorConfig};

pub fn camera_spawn(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>
){
    let window = window_query.get_single().unwrap();
    commands.spawn(
        Camera2dBundle{
            transform: Transform::from_xyz(
                window.width()/2.0
                , window.height()/2.0
                , 0.0
            ),
            camera_2d: Camera2d{
                clear_color: ClearColorConfig::Custom(Color::rgb(0.3,0.3,0.65))
            },
            ..default()
        }
    );
}
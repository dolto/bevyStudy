use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::{RigidBody, Velocity, GravityScale, LockedAxes, Collider, Restitution};
use crate::components::block::*;

pub fn setup_physics(mut commands: Commands) {
    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(500.0, 50.0))
        .insert(TransformBundle::from(Transform::from_xyz(400.0, 100.0, 0.0)));

    /* Create the bouncing ball. */
    commands
        .spawn((
            RigidBody::Dynamic,
            Collider::ball(50.0),
            Restitution::coefficient(0.7),
            TransformBundle::from(Transform::from_xyz(400.0, 400.0, 0.0))
        ));
}

fn spawn_mino(
    commands: &mut ChildBuilder,
    position: Vec3
) -> Entity {
    commands.spawn(
        (
            Block {},
            RigidBody::Dynamic,
            TransformBundle::from(Transform::from_translation(position)),
            Velocity{
                linvel: Vec2::new(0.0, 0.0),
                angvel: 0.0
            },
            GravityScale(3.0),
            LockedAxes::all(),
        )
    ).insert(Collider::cuboid(15.0, 15.0))
    .id()
}

pub fn spawn_j_mino(
    mut commands: Commands,
    main_window: Query<&Window, With<PrimaryWindow>>
){
    if let Ok(window) = main_window.get_single(){
        let mut on_position = Vec3::new(window.width() / 2.0, window.height(), 0.0);
        
        on_position.y -= 120.0;
        commands.spawn(
            (
                TransformBundle::from(Transform::from_translation(on_position)),
                Mino { is_controll : true}
            )
        )
        .with_children(
            |parent| {
                let mut pos = Vec3::new(0.0,0.0,0.0);
                spawn_mino(parent, pos.clone());
                pos.x -= 30.0;
                spawn_mino(parent, pos.clone());
                pos.x += 30.0;
                pos.y += 30.0;
                spawn_mino(parent, pos.clone());
                pos.y += 30.0;
                spawn_mino(parent, pos.clone());
            }
        )
        ;

        //println!("{}", )
    }
}

pub fn spawn_l_mino(
    mut commands: Commands,
    main_window: Query<&Window, With<PrimaryWindow>>
){
    if let Ok(window) = main_window.get_single(){
        let mut on_position = Vec3::new(window.width() / 2.0, window.height(), 0.0);
        
        on_position.y -= 120.0;
        commands.spawn(
            (
                TransformBundle::from(Transform::from_translation(on_position)),
                Mino { is_controll : true}
            )
        )
        .with_children(
            |parent| {
                let mut pos = Vec3::new(0.0,0.0,0.0);
                spawn_mino(parent, pos.clone());
                pos.x += 30.0;
                spawn_mino(parent, pos.clone());
                pos.x -= 30.0;
                pos.y += 30.0;
                spawn_mino(parent, pos.clone());
                pos.y += 30.0;
                spawn_mino(parent, pos.clone());
            }
        )
        ;

        //println!("{}", )
    }
}

pub fn spawn_o_mino(
    mut commands: Commands,
    main_window: Query<&Window, With<PrimaryWindow>>
){
    if let Ok(window) = main_window.get_single(){
        let mut on_position = Vec3::new(window.width() / 2.0, window.height(), 0.0);
        
        on_position.y -= 120.0;
        commands.spawn(
            (
                TransformBundle::from(Transform::from_translation(on_position)),
                Mino { is_controll : true}
            )
        )
        .with_children(
            |parent| {
                let mut pos = Vec3::new(0.0,0.0,0.0);
                spawn_mino(parent, pos.clone());
                pos.x += 30.0;
                spawn_mino(parent, pos.clone());
                pos.y += 30.0;
                spawn_mino(parent, pos.clone());
                pos.x -= 30.0;
                spawn_mino(parent, pos.clone());
            }
        )
        ;

        //println!("{}", )
    }
}

pub fn spawn_i_mino(
    mut commands: Commands,
    main_window: Query<&Window, With<PrimaryWindow>>
){
    if let Ok(window) = main_window.get_single(){
        let mut on_position = Vec3::new(window.width() / 2.0, window.height(), 0.0);
        
        on_position.y -= 120.0;
        commands.spawn(
            (
                TransformBundle::from(Transform::from_translation(on_position)),
                Mino { is_controll : true}
            )
        )
        .with_children(
            |parent| {
                let mut pos = Vec3::new(0.0,0.0,0.0);
                spawn_mino(parent, pos.clone());
                pos.y += 30.0;
                spawn_mino(parent, pos.clone());
                pos.y += 30.0;
                spawn_mino(parent, pos.clone());
                pos.y += 30.0;
                spawn_mino(parent, pos.clone());
            }
        )
        ;

        //println!("{}", )
    }
}

pub fn spawn_z_mino(
    mut commands: Commands,
    main_window: Query<&Window, With<PrimaryWindow>>
){
    if let Ok(window) = main_window.get_single(){
        let mut on_position = Vec3::new(window.width() / 2.0, window.height(), 0.0);
        
        on_position.y -= 120.0;
        commands.spawn(
            (
                TransformBundle::from(Transform::from_translation(on_position)),
                Mino { is_controll : true}
            )
        )
        .with_children(
            |parent| {
                let mut pos = Vec3::new(0.0,0.0,0.0);
                spawn_mino(parent, pos.clone());
                pos.x += 30.0;
                spawn_mino(parent, pos.clone());
                pos.x -= 30.0;
                pos.y += 30.0;
                spawn_mino(parent, pos.clone());
                pos.x -= 30.0;
                spawn_mino(parent, pos.clone());
            }
        )
        ;

        //println!("{}", )
    }
}

pub fn spawn_s_mino(
    mut commands: Commands,
    main_window: Query<&Window, With<PrimaryWindow>>
){
    if let Ok(window) = main_window.get_single(){
        let mut on_position = Vec3::new(window.width() / 2.0, window.height(), 0.0);
        
        on_position.y -= 120.0;
        commands.spawn(
            (
                TransformBundle::from(Transform::from_translation(on_position)),
                Mino { is_controll : true}
            )
        )
        .with_children(
            |parent| {
                let mut pos = Vec3::new(0.0,0.0,0.0);
                spawn_mino(parent, pos.clone());
                pos.x -= 30.0;
                spawn_mino(parent, pos.clone());
                pos.x += 30.0;
                pos.y += 30.0;
                spawn_mino(parent, pos.clone());
                pos.x += 30.0;
                spawn_mino(parent, pos.clone());
            }
        )
        ;

        //println!("{}", )
    }
}
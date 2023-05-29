use bevy::prelude::*;

#[derive(Component)]
pub struct Block{}

#[derive(Component)]
pub struct Mino{
    pub is_controll: bool,
}
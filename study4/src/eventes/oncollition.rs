use bevy::prelude::*;
use bevy_rapier2d::prelude::RigidBody;

pub struct OnCollitionMino{
    parent: Entity,
    child: Vec<Entity>
}

pub fn on_collition_mino(
    mut oncollition_mino : EventReader<OnCollitionMino>,
    mut commands: Commands
){
    for mino_event in oncollition_mino.iter(){
        let parent = mino_event.parent;
        let child = &mino_event.child;

        for ch in child.iter(){
            commands.entity(ch.clone()).insert(RigidBody::KinematicVelocityBased);
        }
        commands.entity(parent).despawn();
    }
}
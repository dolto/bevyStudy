use bevy::prelude::{Vec3, Mesh};

pub trait Structure{
    fn set_rims(&mut self, center_point: &Vec3, rim_center:f32);
    fn get_rims(&self) -> Vec<(Vec3, usize)>;
}
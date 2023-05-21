use bevy::prelude::*;

#[derive(Component)]
pub struct Pip {
    pub width:f32,
    pub height:f32
}
impl Pip{
    pub fn hit_trigger(&self, position:Vec3,target_position: Vec3, half_target_width: f32, half_target_height: f32) -> bool{
        let mut hit_count = 0;
        let half_width = self.width / 2.0;
        let half_height = self.height / 2.0;

        if position.x + half_width > target_position.x + half_target_width && 
        position.x - half_width < target_position.x + half_target_width{
            hit_count += 1;
        }
        else if position.x + half_width < target_position.x - half_target_width && 
        position.x - half_width > target_position.x - half_target_width{
            hit_count += 1;
        }

        if position.y + half_height > target_position.y + half_target_height &&
        position.y - half_height < target_position.y + half_height{
            hit_count += 1;
        }
        else if position.y + half_height > target_position.y - half_target_height &&
        position.y - half_height < target_position.y - half_height{
            hit_count += 1;
        }

        if hit_count >= 2 {
            println!("부딪힘! bird_position: {}\tpipe_position: {}",target_position, position);
            println!("pip_width: {}\tpop_height: {}", half_width, half_height);
            return true;
        }
        return false;
    }
}
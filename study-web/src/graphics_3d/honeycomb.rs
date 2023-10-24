use std::f32::consts::PI;

use bevy::prelude::*;
//use bevy::render::mesh;
use bevy::render::render_resource::PrimitiveTopology;
use wasm_bindgen::JsValue;
use web_sys::console;
use super::rim::Structure;

const HUNEYCOMB_SIZE:f32 = 0.12; //한 변의 길이

#[derive(Resource)]
pub struct HuneycombRes{
    pub honeycomb_center:f32,
    pub huneycomb_mesh:Mesh,
    pub spawn_position:Vec3,
    pub spawn_count:i32,
    pub spawn_size:i32
}
impl Default for HuneycombRes{
    fn default() -> Self {
        HuneycombRes{
            honeycomb_center: f32::sqrt(f32::powi(HUNEYCOMB_SIZE, 2) - f32::powi(HUNEYCOMB_SIZE / 2., 2)) * 2.,
            huneycomb_mesh: {
                let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
                let mut l_transform = Transform::from_translation(Vec3::new(0., 0., HUNEYCOMB_SIZE));
                let mut su_transform = 
                    Transform::from_translation(Vec3::new(0., HUNEYCOMB_SIZE * 0.3, HUNEYCOMB_SIZE * 0.75));
                let mut sd_transform = 
                    Transform::from_translation(Vec3::new(0., HUNEYCOMB_SIZE * -0.3, HUNEYCOMB_SIZE * 0.75));
                let mut base_large_vertex = Vec::with_capacity(6);
                let mut base_small_up_vertex = Vec::with_capacity(6);
                let mut base_small_down_vertex = Vec::with_capacity(6);
                let mut vertex: Vec<[f32;3]> = Vec::with_capacity(96);
                for _ in 0..6 {
                    base_large_vertex.push(l_transform.translation.to_array());
                    base_small_up_vertex.push(su_transform.translation.to_array());
                    base_small_down_vertex.push(sd_transform.translation.to_array());
                    l_transform.rotate_around(Vec3::ZERO, Quat::from_rotation_y(PI / 3.));
                    su_transform.rotate_around(Vec3::ZERO, Quat::from_rotation_y(PI / 3.));
                    sd_transform.rotate_around(Vec3::ZERO, Quat::from_rotation_y(PI / 3.));
                }
                for i in 0..5{
                    vertex = [vertex, vec![
                        base_large_vertex[i].clone(), base_large_vertex[i+1].clone(), base_small_up_vertex[i+1].clone(),
                        base_small_up_vertex[i].clone(), base_large_vertex[i].clone(), base_small_up_vertex[i+1].clone(), 
                        base_large_vertex[i].clone(), base_small_down_vertex[i+1].clone(), base_large_vertex[i+1].clone(), 
                        base_small_down_vertex[i].clone(), base_small_down_vertex[i+1].clone(), base_large_vertex[i].clone(),
                    ]].concat();
                }
                vertex = [vertex, vec![
                    base_large_vertex[5].clone(), base_large_vertex[0].clone(), base_small_up_vertex[0].clone(),
                    base_small_up_vertex[5].clone(), base_large_vertex[5].clone(), base_small_up_vertex[0].clone(), 
                    base_large_vertex[5].clone(), base_small_down_vertex[0].clone(), base_large_vertex[0].clone(), 
                    base_small_down_vertex[5].clone(), base_small_down_vertex[0].clone(), base_large_vertex[5].clone(),
                ]].concat();

                vertex = [vertex, vec![
                    base_small_up_vertex[0].clone(), base_small_up_vertex[1].clone(), base_small_up_vertex[5].clone(),
                    base_small_up_vertex[1].clone(), base_small_up_vertex[2].clone(), base_small_up_vertex[5].clone(),
                    base_small_up_vertex[2].clone(), base_small_up_vertex[4].clone(), base_small_up_vertex[5].clone(),
                    base_small_up_vertex[2].clone(), base_small_up_vertex[3].clone(), base_small_up_vertex[4].clone(),
                ]].concat();
                vertex = [vertex, vec![
                    base_small_down_vertex[0].clone(), base_small_down_vertex[5].clone(), base_small_down_vertex[1].clone(), 
                    base_small_down_vertex[1].clone(), base_small_down_vertex[5].clone(), base_small_down_vertex[2].clone(), 
                    base_small_down_vertex[2].clone(), base_small_down_vertex[5].clone(), base_small_down_vertex[4].clone(), 
                    base_small_down_vertex[2].clone(), base_small_down_vertex[4].clone(), base_small_down_vertex[3].clone(), 
                ]].concat();
                mesh.insert_attribute(
                    Mesh::ATTRIBUTE_POSITION, 
                    vertex
                );

                mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0., 1., 0.]; 96]);
                mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0., 0.]; 96]);
                mesh
            },
            spawn_position: Vec3::ZERO,
            spawn_count: 100,
            spawn_size: 100,
        }
    }
}


#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum HuneycombSpawnState{
    #[default]
    IsSpawn,
    StopSpawn
}

#[derive(Component)]
pub struct Huneycomb{
    pub rims: Vec<Vec3>,
    pub node: [Option<Entity>; 6]
}
impl Structure for Huneycomb{
    fn get_rims(&self) -> Vec<(Vec3, usize)> {
        let _rims = self.rims.clone();
        let mut result = Vec::with_capacity(6);
        let mut count = 0;
        _rims.iter().for_each(|r|{
            if self.node[count].is_none(){
                result.push((r.clone(), count.clone()));
            }
            count += 1;
        });
        result
    }
    fn set_rims(&mut self, center_point: &Vec3, rim_center:f32) {
        let mut result = Vec::with_capacity(6);
        let mut transform = Transform::from_translation(center_point.clone() + Vec3 { x: rim_center, y: 0., z: 0. });
        for _ in 0..6{
            result.push(transform.translation.clone());
            transform.rotate_around(center_point.clone(), Quat::from_rotation_y(PI / 3.));
        }
        self.rims = result;
    }
}

impl Default for Huneycomb{
    fn default() -> Self {
        Huneycomb { 
            rims: Vec::new(), 
            node: [None; 6]
        }
    }
}

pub fn spawn_huneycomb(
    mut commands: Commands,
    huneycomb_res: Res<HuneycombRes>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
){
    let mesh = &huneycomb_res.huneycomb_mesh;
    let mut huneycomb = Huneycomb::default();
    huneycomb.set_rims(&huneycomb_res.spawn_position, huneycomb_res.honeycomb_center);
    commands.spawn(
        (
            PbrBundle{
                mesh: meshes.add(mesh.clone()),
                material: materials.add(Color::rgb(1., 1., 0.0).into()),
                transform: Transform::from_translation(huneycomb_res.spawn_position.clone()),
                ..Default::default()
            },
            huneycomb
        )
    );
    console::log_1(&JsValue::from_str("생성완료"));
}

pub fn set_huneycomb_node(
    mut query_honeycomb: Query<(&mut Huneycomb, &Transform, Entity)>,
    mut huneycomb_res: ResMut<HuneycombRes>,
    mut spawn_state: ResMut<NextState<HuneycombSpawnState>>
){
    let mut set_pos = false;
    let mut honey_entity = Vec::with_capacity(huneycomb_res.spawn_size as usize);
    for (_,t,e) in query_honeycomb.iter(){
        honey_entity.push((t.translation.clone(), e));
    }
    for (mut h,_, _) in query_honeycomb.iter_mut(){
        for (v, u) in h.get_rims().iter(){
            for (t, e) in honey_entity.iter(){
                if v.abs_diff_eq(t.clone(), HUNEYCOMB_SIZE/5.){
                    h.node[u.clone()] = Some(e.clone());
                    break;
                }
            }
        }
    }
    for (h, _, _) in query_honeycomb.iter(){
        for (v, _) in h.get_rims().iter(){
            huneycomb_res.spawn_position = v.clone();
            console::log_1(&JsValue::from_str("비어있는 자리 선정"));
            console::log_1(&JsValue::from_str(format!("t:{:?}",v.clone()).as_str()));
            set_pos = true;
            break;
        }
        if set_pos{
            break;
        }
    }
    if huneycomb_res.spawn_count > 1{
        console::log_1(&JsValue::from_str("생성 이벤트"));
        //events_huneycomb_spawn.send(HuneycombSpawnEventAfter);
        huneycomb_res.spawn_count -= 1;
    }
    else{
        spawn_state.set(HuneycombSpawnState::StopSpawn);
    }
}
use std::{f32::consts::PI, collections::{HashMap, HashSet}};

use bevy::{prelude::*, render::render_resource::{PrimitiveTopology, encase::rts_array::Length}, window::PrimaryWindow};
use bevy_mod_picking::{PickableBundle, prelude::{Pointer, On, Move, Listener, Over, Out, RaycastPickTarget, Click, PointerButton}, focus::HoverMap};
use hexx::{*, algorithms::a_star};
use rand::Rng;
use wasm_bindgen::JsValue;
use web_sys::console;

const HEX_SIZE:f32 = 0.15;
const MAP_RADIUS: u32 = 10;
fn hexagonal_column(huneycomb_size: f32) -> Mesh{
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    let mut l_transform = Transform::from_translation(Vec3::new(0., 0., huneycomb_size));
    let mut su_transform = 
        Transform::from_translation(Vec3::new(0., huneycomb_size * 0.3, huneycomb_size * 0.75));
    let mut sd_transform = 
        Transform::from_translation(Vec3::new(0., huneycomb_size * -0.3, huneycomb_size * 0.75));
    let mut base_large_vertex = Vec::with_capacity(6);
    let mut base_small_up_vertex = Vec::with_capacity(6);
    let mut base_small_down_vertex = Vec::with_capacity(6);
    let mut vertex: Vec<[f32;3]> = Vec::with_capacity(96);

    l_transform.rotate_around(Vec3::ZERO, Quat::from_rotation_y(PI / 6.));
    su_transform.rotate_around(Vec3::ZERO, Quat::from_rotation_y(PI / 6.));
    sd_transform.rotate_around(Vec3::ZERO, Quat::from_rotation_y(PI / 6.));

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
}

#[derive(Debug, Resource)]
pub struct Map {
    pub entities: HashMap<Hex, Entity>,
    pub entities_forentity: HashMap<Entity, Hex>,
    pub blocked_coords: HashSet<Hex>,
    pub path_entities: HashSet<Entity>,
    pub pathlist_hex: Vec<Hex>,
    pub layout: HexLayout,
    pub default_mat: Handle<StandardMaterial>,
    pub blocked_mat: Handle<StandardMaterial>,
    pub path_mat: Handle<StandardMaterial>,
    pub path_list_mat: Handle<StandardMaterial>,
}

#[derive(Component)]
pub struct Honeycomb;

pub fn setup_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
){
    let layout = HexLayout {
        hex_size: Vec2::splat(HEX_SIZE),
        ..default()
    };
    // materials
    let default_mat = materials.add(Color::WHITE.into());
    let blocked_mat = materials.add(Color::BLACK.into());
    let path_mat = materials.add(Color::CYAN.into());
    let path_list_mat = materials.add(Color::TOMATO.into());

    // mesh
    let mesh = hexagonal_column(HEX_SIZE);
    let mesh_handle = meshes.add(mesh);
    let mut blocked_coords = HashSet::with_capacity(MAP_RADIUS as usize * 3 );
    let mut entities_forentity: HashMap<Entity, Hex> = HashMap::with_capacity(MAP_RADIUS as usize * 3);
    let entities = shapes::hexagon(Hex::ZERO, MAP_RADIUS)
        //.enumerate()
        .map(|hex| {
            let pos = layout.hex_to_world_pos(hex);
            let mut rgn = rand::thread_rng();
            let is_blocked = rgn.gen_bool(1./4.);
            let mat = 
                if is_blocked {
                    blocked_coords.insert(hex);
                    blocked_mat.clone()
                } else {
                    default_mat.clone()
                };
            let id = commands
                .spawn((
                    PbrBundle {
                    transform: Transform::from_xyz(pos.x, 0., pos.y)
                        .with_scale(Vec3::splat(0.9)),
                    mesh: mesh_handle.clone(),
                    material: mat,
                    ..default()
                    },
                    Honeycomb,
                    PickableBundle::default(),
                    RaycastPickTarget::default(),
                    On::<Pointer<Over>>::run(on_over),
                    On::<Pointer<Out>>::run(on_out),
                    On::<Pointer<Click>>::run(handle_input_node)
                ))
                .id();
            entities_forentity.insert(id, hex);
            (hex, id)
        })
        .collect();
    let map = Map {
        entities,
        default_mat,
        blocked_mat,
        path_mat,
        blocked_coords,
        entities_forentity,
        path_entities: Default::default(),
        pathlist_hex: Default::default(),
        layout,
        path_list_mat,
    };
    //console::log_1(&JsValue::from_str(format!("{:?}\n", map).as_str()));
    commands.insert_resource(map);
}

fn on_over(
    mut commands: Commands,
    event: Listener<Pointer<Over>>,
    grid: Res<Map>,
){
    //console::log_1(&JsValue::from_str("over이벤트!"));
    commands.entity(event.target).insert(grid.path_mat.clone());
}

fn on_out(
    mut commands: Commands,
    event: Listener<Pointer<Out>>,
    grid: Res<Map>,
){
    //console::log_1(&JsValue::from_str("out이벤트!"));
    if grid.blocked_coords.get(&grid.entities_forentity[&event.target]).is_some(){
        commands.entity(event.target).insert(grid.blocked_mat.clone());
    }
    else if grid.path_entities.get(&event.target).is_some(){
        commands.entity(event.target).insert(grid.path_mat.clone());
    }
    else{
        commands.entity(event.target).insert(grid.default_mat.clone());
    }
}

fn handle_input_node(
    mut commands: Commands,
    event: Listener<Pointer<Click>>,
    mut grid: ResMut<Map>,
) {
    if event.button == PointerButton::Primary{
        let entity = event.target;
        let find_hex = grid.entities_forentity[&entity].clone();
        grid.pathlist_hex.push(find_hex);
        let mut path_list: Vec<Hex> = Vec::new();
        let mut point: Hex = grid.pathlist_hex[0];
        for i in 1..grid.pathlist_hex.len(){
            let path_comp = a_star(point, grid.pathlist_hex[i], |h| {
                (grid.entities.contains_key(&h) && !grid.blocked_coords.contains(&h)).then_some(1)
            });
            if path_comp.is_some(){
                let comp = path_comp.unwrap();
                path_list = [path_list, comp].concat();
                point = grid.pathlist_hex[i];
            }
            else{
                console::log_1(&JsValue::from_str("경로를 찾을 수 없었습니다!"));
                let index = grid.pathlist_hex.len();
                grid.pathlist_hex.remove( index - 1);
            }
        }
        //console::log_1(&JsValue::from_str(format!("만들어진 경로: {:?}", path_list).as_str()));
        for p in grid.path_entities.iter(){
            commands.entity(*p).insert(grid.default_mat.clone());
        }
        grid.path_entities.clear();
        for h in path_list.iter(){
            let entity = grid.entities[h];
            commands.entity(entity).insert(grid.path_mat.clone());
            grid.path_entities.insert(entity);
        }
        for h in grid.pathlist_hex.iter(){
            commands.entity(grid.entities[h]).insert(grid.path_list_mat.clone());
        }
        //commands.entity(entity).insert(grid.path_list_mat.clone());
    }
    else if event.button == PointerButton::Secondary{
        console::log_1(&JsValue::from_str("오른쪽 클릭 테스트"));
    }
    
    
}

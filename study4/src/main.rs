use std::cmp;

use bevy::{prelude::{*, system_adapter::new}, window::PrimaryWindow, core_pipeline::clear_color::ClearColorConfig};
use rand::{seq::SliceRandom, thread_rng};

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_startup_system(camera_spawn)
    .add_plugin(MinoPlugin)
    .run();
}

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

#[derive(Component)]
pub struct Block{
    pub line: i32, //어떤 y축 라인에 있는지
    pub side: i32, //어떤 x축 라인에 있는지
    pub is_controll: bool, //현재 조작중인 블록인지
    pub rotation_mode: [(i32, i32);4], //회전할 때 움직일 상대적 위치리스트
    pub rotation_state: usize //현재 어떻게 회전했는지
}
pub enum MinoType{
    I, J, L, S, Z, T
}
const LINE_SIZE:usize = 20;
const SIDE_SIZE:usize = 16;
#[derive(Resource)]
pub struct MinoSpawnSelecter{
    pub selecter : usize, //0~6까지의 미노 종류를 선택함
    pub selecte_list : Vec<MinoType>,
    
}
impl Default for MinoSpawnSelecter{
    fn default() -> MinoSpawnSelecter {
        MinoSpawnSelecter { selecter: 6 , 
            selecte_list: vec![MinoType::I, MinoType::J, MinoType::L, MinoType::S, MinoType::Z ,MinoType::T]
        }
    }
}
#[derive(Resource)]
pub struct MinoLineArray{
    pub line_list : [[bool;SIDE_SIZE];LINE_SIZE] //line,side
}
impl Default for MinoLineArray{
    fn default() -> MinoLineArray {
        let temp = [[false;SIDE_SIZE];LINE_SIZE];
        //print!("{:?}",temp);
        MinoLineArray { line_list: temp}
    }
}

#[derive(Resource)]
pub struct MinoMoveTimer{
    pub timer: Timer
}
impl Default for MinoMoveTimer{
    fn default() -> MinoMoveTimer {
        MinoMoveTimer { timer: Timer::from_seconds(0.75, TimerMode::Repeating) }
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum MinoSpawnToggle{
    #[default]
    On,
    Off
}
pub fn spawn_mino(
    mut mino_selector: ResMut<MinoSpawnSelecter>,
    mut next_block: ResMut<NextState<MinoSpawnToggle>>,
    mut commands: Commands,
    window: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
){
    if mino_selector.selecter > 5{
        let mut rng = thread_rng();
        mino_selector.selecte_list.shuffle(&mut rng);
        mino_selector.selecter = 0;
    }
    let selecter = mino_selector.selecter;
    let main_window = window.get_single().unwrap();
    let texture:Handle<Image> = asset_server.load("black_dot.png");
    let center = (SIDE_SIZE / 2) as i32;
    match mino_selector.selecte_list[selecter] {
        MinoType::I => {
            spawn_block(&mut commands, 0, center, &main_window, &texture, [(-2,2), (2,2), (2,-2), (-2,-2)]);
            spawn_block(&mut commands, 1, center, &main_window, &texture,[(-1,1), (1,1), (1,-1), (-1,-1)]);
            spawn_block(&mut commands, 2, center, &main_window, &texture,[(0,0), (0,0), (0,0), (0,0)]);
            spawn_block(&mut commands, 3, center, &main_window, &texture, [(1,-1), (-1,-1), (-1,1), (1,1)]);
        },
        MinoType::J => {
            spawn_block(&mut commands, 0, center + 1, &main_window, &texture, [(-2,2), (2,2), (2,-2), (-2,-2)]);
            spawn_block(&mut commands, 1, center + 1, &main_window, &texture, [(-1,1), (1,1), (1,-1), (-1,-1)]);
            spawn_block(&mut commands, 2, center + 1, &main_window, &texture, [(0,0), (0,0), (0,0), (0,0)]);
            spawn_block(&mut commands, 2, center, &main_window, &texture,[(-1,-1), (-1,1), (1,1), (1,-1)]);
        },
        MinoType::L => {
            spawn_block(&mut commands, 0, center, &main_window, &texture,[(-2,2), (2,2), (2,-2), (-2,-2)]);
            spawn_block(&mut commands, 1, center, &main_window, &texture,[(-1,1), (1,1), (1,-1), (-1,-1)]);
            spawn_block(&mut commands, 2, center, &main_window, &texture,[(0,0), (0,0), (0,0), (0,0)]);
            spawn_block(&mut commands, 2, center + 1, &main_window, &texture,[(1,1), (1,-1), (-1,-1), (-1,1)]);
        },
        MinoType::S => {
            spawn_block(&mut commands, 0, center+2, &main_window, &texture, [(0,2), (2,0), (0,-2), (-2,0)]);
            spawn_block(&mut commands, 0, center+1, &main_window, &texture, [(-1,1), (1,1), (1,-1), (-1,-1)]);
            spawn_block(&mut commands, 1, center+1, &main_window, &texture, [(0,0), (0,0), (0,0), (0,0)]);
            spawn_block(&mut commands, 1, center, &main_window, &texture, [(-1,-1), (-1,1), (1,1), (1,-1)]);
        },
        MinoType::Z => {
            spawn_block(&mut commands, 0, center, &main_window, &texture,[(-2,0), (0,2), (2,0), (0,-2)]);
            spawn_block(&mut commands, 0, center+1, &main_window, &texture,[(-1,1), (1,1), (1,-1), (-1,-1)]);
            spawn_block(&mut commands, 1, center+1, &main_window, &texture,[(0,0), (0,0), (0,0), (0,0)]);
            spawn_block(&mut commands, 1, center+2, &main_window, &texture,[(1,1), (1,-1), (-1,-1), (-1,1)]);
        },
        MinoType::T => {
            spawn_block(&mut commands, 0, center, &main_window, &texture,[(-1,-1), (-1,1), (1,1), (1,-1)]);
            spawn_block(&mut commands, 0, center+1, &main_window, &texture,[(0,0), (0,0), (0,0), (0,0)]);
            spawn_block(&mut commands, 0, center+2, &main_window, &texture,[(1,1),(1,-1),(-1,-1),(-1,1)]);
            spawn_block(&mut commands, 1, center+1, &main_window, &texture,[(1,-1), (-1,-1), (-1,1),(1,1)]);
        },
    }
    mino_selector.selecter += 1;
    next_block.set(MinoSpawnToggle::Off);
}

fn spawn_block(
    commands: &mut Commands,
    line: i32,
    side: i32,
    main_window: &Window,
    texture: &Handle<Image>,
    rotation_mode: [(i32, i32);4]
){
    commands.spawn(
        (
            Block{
                line,side, is_controll: true,rotation_mode: rotation_mode, rotation_state: 0
            },
            SpriteBundle{
                sprite: Sprite {
                    custom_size: Some(Vec2::new(32.0,32.0)),
                    ..default()
                },
                transform: Transform{
                    translation: Vec3 { x: (side * 32) as f32 + 16.0, 
                        y: main_window.height() - 16.0 - (line * 32) as f32, z: 0.0 },
                    ..default()
                },
                texture: texture.clone(),
                ..default()
            }
        )
    );
}

pub fn mino_movement(
    mut block_query: Query<(&mut Block, &mut Transform)>,
    window: Query<&Window, With<PrimaryWindow>>,
    mut next_block: ResMut<NextState<MinoSpawnToggle>>,
    mut mino_line_array: ResMut<MinoLineArray>,
    input_key: Res<Input<KeyCode>>,
    move_timer: Res<MinoMoveTimer>
){
    let mut move_direction: (i32, i32) = (0,0);
    let mut collition = false;
    let line_map = &mut mino_line_array.line_list;
    let mut c_block = Vec::with_capacity(4);
    let mut rotation_left = false;
    let mut rotation_right = false;
    let main_window = window.get_single().unwrap();

    if move_timer.timer.finished() || input_key.just_pressed(KeyCode::S) || input_key.just_pressed(KeyCode::Down){
        move_direction.0 += 1;
    }
    if input_key.just_pressed(KeyCode::A) || input_key.just_pressed(KeyCode::Left){
        move_direction.1 -= 1;
    }
    if input_key.just_pressed(KeyCode::D) || input_key.just_pressed(KeyCode::Right){
        move_direction.1 += 1;
    }
    if input_key.just_pressed(KeyCode::W) || input_key.just_pressed(KeyCode::Up) || input_key.just_pressed(KeyCode::X){
        rotation_right = true;
    }
    if input_key.just_pressed(KeyCode::Z){
        rotation_left = true;
    }
    for (block, block_transform) in block_query.iter_mut(){
        if block.is_controll{
            if block.side + move_direction.1 < 0 || block.side + move_direction.1 >= SIDE_SIZE as i32{
                move_direction.1 = 0;
            }
            if block.line + move_direction.0 < 0 || block.line + move_direction.0 >= LINE_SIZE as i32{
                collition = true;
                move_direction.0 = 0;
            }
            if line_map[(block.line + move_direction.0) as usize][(block.side + move_direction.1) as usize]{
                if move_direction.0 != 0{
                    collition = true;
                    move_direction.0 = 0;
                }
                if move_direction.1 != 0{
                    move_direction.1 = 0;
                }
            }
            if rotation_right{
                println!("시작");
                let mut temp = block.rotation_state as i32 + 1;
                if temp >= 4{
                    temp = 0;
                }
                println!("temp 설정");
                if block.line + move_direction.0 + block.rotation_mode[temp as usize].0 < 0 || 
                block.line + move_direction.0 + block.rotation_mode[temp as usize].0 >= LINE_SIZE as i32{
                    rotation_right = false;
                    println!("취소");
                }
                else if block.side + move_direction.1 + block.rotation_mode[temp as usize].1 < 0 ||
                block.side + move_direction.1 + block.rotation_mode[temp as usize].1 >= SIDE_SIZE as i32{
                    rotation_right = false;
                    println!("취소");
                }
                else if line_map[(block.line + move_direction.0 + block.rotation_mode[temp as usize].0) as usize]
                [(block.side + move_direction.1 + block.rotation_mode[temp as usize].1) as usize]{
                    println!("line: {}          side: {}", 
                    block.line + move_direction.0 + block.rotation_mode[temp as usize].0, 
                    block.side + move_direction.1 + block.rotation_mode[temp as usize].1);
                    println!("취소");
                    rotation_right = false;
                }
                else{
                    println!("이 블록은 옮기기 가능");
                }
            }
            if rotation_left{
                let mut temp = block.rotation_state as i32 - 1;
                if temp < 0 {
                    temp = 3;
                }
                if block.line + move_direction.0 - block.rotation_mode[temp as usize].0 < 0 ||
                block.line + move_direction.0 - block.rotation_mode[temp as usize].0 >= LINE_SIZE as i32{
                    rotation_left = false;
                    println!("취소");
                }
                else if block.side + move_direction.1 - block.rotation_mode[temp as usize].1 < 0 ||
                block.side + move_direction.1 - block.rotation_mode[temp as usize].1 >= SIDE_SIZE as i32{
                    rotation_left = false;
                    println!("취소");
                }
                else if line_map[(block.line + move_direction.0 - block.rotation_mode[temp as usize].0) as usize]
                [(block.side + move_direction.1 - block.rotation_mode[temp as usize].1) as usize]{
                    rotation_left = false;
                }
            }
            c_block.push((block, block_transform));
        }
    }

    if collition{
        for (c, t) in c_block.iter_mut(){
            c.line += move_direction.0;
            c.side += move_direction.1;
            if rotation_right {
                c.rotation_state += 1;
                if c.rotation_state >= 4{
                    c.rotation_state = 0;
                }
                c.line += c.rotation_mode[c.rotation_state].0;
                c.side += c.rotation_mode[c.rotation_state].1;
            }
            if rotation_left {
                c.line -= c.rotation_mode[c.rotation_state].0;
                c.side -= c.rotation_mode[c.rotation_state].1;
                if c.rotation_state == 0{
                    c.rotation_state = 4;
                }
                c.rotation_state -= 1;
            }
            c.is_controll = false;

            t.translation.x = (c.side * 32) as f32 + 16.0;
            t.translation.y = main_window.height() - 16.0 - (c.line * 32) as f32;
            line_map[c.line as usize][c.side as usize] = true;
            //print!("여기서 터지나? line: {}     side: {}", c.line, c.side);
            next_block.set(MinoSpawnToggle::On);
            //이후에 라인이 꽉 찼음을 체크하는 건 다른 곳에서 확인
        }
    }else{
        for (c, t) in c_block.iter_mut(){
            c.line += move_direction.0;
            c.side += move_direction.1;
            if rotation_right {
                c.rotation_state += 1;
                if c.rotation_state >= 4{
                    c.rotation_state = 0;
                }
                c.line += c.rotation_mode[c.rotation_state].0;
                c.side += c.rotation_mode[c.rotation_state].1;
            }
            if rotation_left {
                c.line -= c.rotation_mode[c.rotation_state].0;
                c.side -= c.rotation_mode[c.rotation_state].1;
                if c.rotation_state == 0{
                    c.rotation_state = 4;
                }
                c.rotation_state -= 1;
            }
            t.translation.x = (c.side * 32) as f32 + 16.0;
            t.translation.y = main_window.height() - 16.0 - (c.line * 32) as f32;
        }
    }
    
}
pub fn mino_move_timer_tick(
    time: Res<Time>,
    mut timer: ResMut<MinoMoveTimer>
){
    timer.timer.tick(time.delta());
}
pub struct MinoPlugin;
impl Plugin for MinoPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_state::<MinoSpawnToggle>()
        .init_resource::<MinoSpawnSelecter>()
        .init_resource::<MinoLineArray>()
        .init_resource::<MinoMoveTimer>()
        //.add_startup_system(spawn_mino)
        .add_systems(
            (
                spawn_mino,
            ).in_schedule(OnEnter(MinoSpawnToggle::On))
        )
        .add_systems(
            (
                mino_move_timer_tick,
                mino_movement
            ).in_set(OnUpdate(MinoSpawnToggle::Off)) //임시
        );
    }
}

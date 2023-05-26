//use std::{fs::{OpenOptions, create_dir_all}, io::{Write}};

use bevy::prelude::*;

use super::{resources::Score, components::ScoreBord, set_item_to_web_storage};

pub fn score_up(
    time:Res<Time>,
    mut score: ResMut<Score>
){
    score.score += time.delta_seconds();
}

pub fn score_borad_spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>
){
    commands.spawn((
        TextBundle{
            style: Style{
                position_type: PositionType::Absolute,
                position: UiRect { 
                    left: Val::Percent(25.0), 
                    right: Val::Percent(25.0), 
                    top: Val::Percent(0.0), 
                    bottom: Val::Percent(90.0) },
                overflow: Overflow::Hidden,
                //direction: Direction::LeftToRight,
                justify_content: JustifyContent::Center,
                border: UiRect { left: Val::Px(1.0), right: Val::Px(1.0), top: Val::Px(1.0), bottom: Val::Px(1.0) },
                padding: UiRect {
                    left: Val::Px(5.0),
                    right: Val::Px(5.0),
                    top: Val::Px(5.0),
                    bottom: Val::Px(5.0)
                },
                ..default()
            },
            background_color: BackgroundColor(Color::rgb(0.2, 0.2, 0.2)),
            text: Text { sections: vec![
                TextSection::new("0.0", TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size:64.0,
                    ..default()
                })
            ], alignment: TextAlignment::Center,
            ..default() },
            ..default()
        },
        ScoreBord {}
    ));
}

pub fn score_borad_update(
    mut score_borad_query: Query<&mut Text ,With<ScoreBord>>,
    score: Res<Score>
){
    for mut score_bord in score_borad_query.iter_mut(){
        score_bord.sections[0].value = format!("{:.2}",score.score) ;
    }
}

pub fn save_score(
    mut score: ResMut<Score>
){
    // create_dir_all("./savedata").expect("폴더 생성 실패!");
    // let mut f = match OpenOptions::new().write(true).create(true).open("./savedata/save.csv") {
    //     Ok(e) => e,
    //     Err(_) => {
    //         panic!("파일 생성 실패!");
    //     },
    // };
    let sc = score.score;
    score.score_list.push(sc);
    score.score_list.sort_by(|a, b| b.partial_cmp(a).unwrap());
    let mut score_save_db = String::new();
    for s in score.score_list.iter(){
        //f.write_all(format!("{:.2}\n",s).as_bytes()).unwrap();
        let mut sd = s.to_string();
        sd.push(',');
        score_save_db.push_str(&sd);
    }
    set_item_to_web_storage("score", &score_save_db);
    score.score = 0.0;
}
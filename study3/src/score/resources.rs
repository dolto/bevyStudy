use std::{fs::{OpenOptions, create_dir_all, canonicalize}, io::Read, path::{PathBuf}};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys;
use bevy::prelude::*;

use super::get_item_from_web_storage;

#[derive(Resource)]
pub struct Score{
    pub score: f32,
    pub score_list: Vec<f32>
}
impl Default for Score{
    fn default() -> Score {
        // let debug_path = PathBuf::from("./savedata");
        // create_dir_all("./savedata").expect(&format!("{:?} 폴더 생성 실패!",canonicalize(&debug_path)));
        // let mut f = match OpenOptions::new().append(true).create(true).read(true).open("./savedata/save.csv") {
        //     Ok(e) => e,
        //     Err(e) => {
        //         panic!("{} 파일 생성 실패!",e.to_string());
        //     },
        // };
        if let Some(db) = get_item_from_web_storage("score"){
            let db_list = db.split(",");
            let mut data_list = Vec::with_capacity(db_list.clone().count());
            for d in db_list{
                data_list.push(match d.parse::<f32>() {
                    Ok(i) => {i},
                    Err(_) => {0.0},
                })
            }
            data_list.sort_by(|a, b| b.partial_cmp(a).unwrap());
    
            return Score { 
                score: 0.0,
                score_list: data_list
            };
        }
        else {
            return Score { 
                score: 0.0,
                score_list: Vec::new()
            }
        }
        //f.read_to_string(&mut db).unwrap();
        
    }
}
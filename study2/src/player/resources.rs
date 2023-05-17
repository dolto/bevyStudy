use std::{fs::File, io::{Read, Write}};
use bevy::prelude::*;

#[derive(Resource)]
pub struct Score {
    pub value: u32,
}
impl Default for Score {
    fn default() -> Score {
        Score { value: 0 }
    }
}

#[derive(Resource, Debug)]
pub struct HighScores{
    pub scores: Vec<(String, u32)>
}
impl Default for HighScores{
    fn default() -> HighScores {
        let save_file_open_try = File::open("data.save");
        let mut save_file = match save_file_open_try {
            Err(_) => {
                match File::create("data.save") {
                    Err(_) => panic!("file create is faild!"),
                    Ok(_) => {
                        File::open("data.save").unwrap()
                    }
                }
            },
            Ok(f) => f
        };
        let mut data = String::new();
        save_file.read_to_string(&mut data).unwrap();

        let data_list = data.split("\n").collect::<Vec<&str>>();
        let mut save_data: Vec<(String, u32)> = Vec::with_capacity(data_list.len());
        for db in data_list{
            if db.eq(""){
                continue;
            } 
            let ele = db.split(" ").collect::<Vec<&str>>();
            save_data.push((ele[0].to_string(), match ele[1].parse::<u32>() {
                Ok(u) => u,
                Err(_) => 0,
            }));
        }
        HighScores { scores: save_data}
    }
}
impl HighScores {
    pub fn save(&mut self){
        //println!("save is process...");
        // let mut save_file = OpenOptions::new()
        // .append(true).open("data.save").expect(
        //     "cannot open file"
        // );
        let mut save_file = 
        match File::create("data.save"){
            Err(_) => panic!("file create is faild!"),
            Ok(f) => f
        };

        self.scores.sort_by(|(_, a2), (_, b2)| b2.cmp(a2));

        for (name, score) in self.scores.iter(){
            let data_string = format!("{} {}\n", name, score);
            println!("{}",data_string);
            save_file.write(data_string.as_bytes()).expect("write failed!");
        }
    }
}



use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;

#[derive(Serialize, Deserialize, Debug)]
pub struct Relics {
    pub name: String,
    pub influence: i32,
    pub science: i32,
    pub fertility: i32,
    pub diplomacy: i32,
    pub mastery: i32,
    //pub has_been_found: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Expandable {
    pub name: String,
    pub warpower: i32,
    pub influence: i32,
    pub science: i32,
    pub fertility: i32,
    pub diplomacy: i32,
    pub mastery: i32,
    //pub count: bool,
}

pub fn read_expandables(path: String) -> Vec<Expandable> {
    let mut file = File::open(path).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let array: Vec<Expandable> = serde_json::from_str(&data).unwrap();

    for elem in array.iter() {
        println!("{:?}", elem);
    }
    array
}

pub fn read_relics(path: String) -> Vec<Relics> {
    let mut file = File::open(path).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let array: Vec<Relics> = serde_json::from_str(&data).unwrap();

    for elem in array.iter() {
        println!("{:?}", elem);
    }
    array
}

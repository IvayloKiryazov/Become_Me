use serde::Deserialize;
use std::fs::File;
use std::io::Read;

// Permanent buffs
#[derive(Deserialize, Clone, Debug)]
pub struct Relics {
    pub name: String,
    pub influence: i32,
    pub science: i32,
    pub fertility: i32,
    pub diplomacy: i32,
    pub mastery: i32,
}

// Just this turn.
#[derive(Deserialize, Clone, Debug)]
pub struct Expandable {
    pub name: String,
    pub warpower: i32,
    pub influence: i32,
    pub science: i32,
    pub fertility: i32,
    pub diplomacy: i32,
    pub mastery: i32,
}

// TODO: unify these two functions.
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

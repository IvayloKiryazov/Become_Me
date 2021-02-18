extern crate ggez;
extern crate rand;

use ggez::Context;
use rand::Rng;

use map::*;

#[derive(Clone)]
pub struct Leader {
    pub name: String,
    pub influence: u32,
    pub science: u32,
    pub fertility: u32,
    pub diplomacy: u32,
    pub mastery: u32,
    pub population: u32,
    pub search_counter: u32,
    pub color: ggez::graphics::Color,
    pub owned_tiles: Vec<Position>,
    //pub Inventory:       []TempItem,
    pub inventory_size: u32,
    pub artefact_counter: u32,
}

impl Leader {
    pub fn new(name: String, color: ggez::graphics::Color) -> Self {
        let mut rng = rand::thread_rng();
        let res = Leader {
            name: name,
            influence: rng.gen_range(0..2),
            science: rng.gen_range(0..2),
            fertility: rng.gen_range(0..2),
            diplomacy: rng.gen_range(0..2),
            mastery: rng.gen_range(0..2),
            population: 0,
            search_counter: 0,
            color: color,
            owned_tiles: vec![],
            //pub Inventory:       []TempItem,
            inventory_size: 0,
            artefact_counter: 0,
        };
        res
    }

    pub fn starting_village(
        &mut self,
        ctx: &mut Context,
        x: usize,
        y: usize,
        map: &mut Vec<Row>,
        direction: usize,
    ) -> () {
        let mut direction_vector: Vec<Endpoint> = Vec::new();

        direction_vector.push(Endpoint::new(1, 1)); //left up corner
        direction_vector.push(Endpoint::new(-1, 1)); //right up corner
        direction_vector.push(Endpoint::new(1, -1)); //left down corner
        direction_vector.push(Endpoint::new(-1, -1)); //right down corner

        let mut column = y;
        let mut _row = x;
        for _i in 0..5 {
            _row = x;
            for _j in 0..5 {
                map[column][_row].owner = self.name.clone();
                self.population += map[column][_row].population;
                self.owned_tiles
                    .push(Position::new(map[column][_row].i, map[column][_row].j));
                map[column][_row].change_color(ctx, self.color);

                if direction_vector[direction].x == -1 {
                    _row -= 1;
                } else {
                    _row += 1;
                }
            }
            if direction_vector[direction].y == -1 {
                column -= 1;
            } else {
                column += 1;
            }
        }
    }
}

#[derive(Clone)]
pub struct Endpoint {
    pub x: i32,
    pub y: i32,
}

impl Endpoint {
    pub fn new(x: i32, y: i32) -> Self {
        let res = Endpoint { x: x, y: y };
        res
    }
}

#[derive(Clone, PartialEq)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        let res = Position { x: x, y: y };
        res
    }
}

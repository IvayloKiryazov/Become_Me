extern crate ggez;
extern crate rand;

use ggez::graphics::Color;
use ggez::Context;

use leader::*;
use map::*;

//TODO get these their own place
pub const YELLOW: usize = 7;
pub const BLUE: usize = 6;
pub const GRAY: usize = 5;
pub const GREEN: usize = 4;
pub const RED: usize = 3;
pub const BROWN: usize = 2;
pub const PURPLE: usize = 1;
pub const CYAN: usize = 0;

pub struct UI {
    pub curr_player: Leader,
    pub actions: Vec<Rectangle>,
    pub curr_square: Square,
    pub prev_square: Square,
    pub prev_color: Color,
}

impl UI {
    pub fn new(
        ctx: &mut Context,
        curr_player: Leader,
        curr_square: Square,
        prev_square: Square,
        color_pallete: Vec<Color>,
    ) -> Self {
        let mut actions = Vec::new();
        let rect = Rectangle::new(
            ctx,
            0.0,
            400.0,
            200.0,
            100.0,
            "Move".to_string(),
            color_pallete[CYAN],
        );
        actions.push(rect);
        let rect = Rectangle::new(
            ctx,
            205.0,
            400.0,
            200.0,
            100.0,
            "Search".to_string(),
            color_pallete[CYAN],
        );
        actions.push(rect);
        let rect = Rectangle::new(
            ctx,
            0.0,
            505.0,
            200.0,
            100.0,
            "Create".to_string(),
            color_pallete[CYAN],
        );
        actions.push(rect);
        let rect = Rectangle::new(
            ctx,
            205.0,
            505.0,
            200.0,
            100.0,
            "Populate".to_string(),
            color_pallete[CYAN],
        );
        actions.push(rect);
        let rect = Rectangle::new(
            ctx,
            0.0,
            610.0,
            200.0,
            100.0,
            "Use Item".to_string(),
            color_pallete[CYAN],
        );
        actions.push(rect);
        let rect = Rectangle::new(
            ctx,
            205.0,
            610.0,
            200.0,
            100.0,
            "End Turn".to_string(),
            color_pallete[CYAN],
        );
        actions.push(rect);

        let rect = Rectangle::new(
            ctx,
            410.0,
            400.0,
            145.0,
            205.0,
            "Tile stats:".to_string(),
            color_pallete[BROWN],
        );
        actions.push(rect);

        let rect = Rectangle::new(
            ctx,
            410.0,
            610.0,
            145.0,
            100.0,
            "Time Left:".to_string(),
            color_pallete[PURPLE],
        );
        actions.push(rect);

        let rect = Rectangle::new(
            ctx,
            0.0,
            0.0,
            195.0,
            355.0,
            "Your stats:".to_string(),
            color_pallete[BROWN],
        );
        actions.push(rect);

        let rect = Rectangle::new(
            ctx,
            200.0,
            0.0,
            355.0,
            355.0,
            "Inventory:".to_string(),
            color_pallete[PURPLE],
        );
        actions.push(rect);

        let res = UI {
            curr_player: curr_player.clone(),
            actions: actions,
            curr_square: curr_square,
            prev_square: prev_square,
            prev_color: curr_player.color,
        };
        res
    }
}

extern crate ggez;

use ggez::graphics::Rect;
use ggez::{graphics, Context};

//Square is the smallest structural point of the game.
//It's the tiles that make up the battlefield.
#[derive(Clone)]
pub struct Square {
    pub owner: String,
    pub population: u32,
    pub rect_obj: Rect,
    pub rect_mesh: graphics::Mesh,
    pub color: ggez::graphics::Color,
    pub searched: bool,
    pub can_create_on: bool,
    pub i: usize,
    pub j: usize,
    pub usable: bool,
}

pub type Row = Vec<Square>;

impl Square {
    pub fn new(
        ctx: &mut Context,
        _place_x: f32,
        _place_y: f32,
        color: ggez::graphics::Color,
        i: usize,
        j: usize,
    ) -> Self {
        let rect = graphics::Rect::new(_place_x, _place_y, 40.0, 40.0);
        //error handle
        let r = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, color);
        let res = Square {
            owner: "Ol'uns".to_string(),
            population: 10,
            rect_obj: rect,
            rect_mesh: r.unwrap(),
            color: color,
            searched: false,
            can_create_on: true,
            i: i,
            j: j,
            usable: true,
        };
        res
    }

    pub fn change_color(&mut self, ctx: &mut Context, color: ggez::graphics::Color) {
        self.color = color;
        self.rect_mesh =
            graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), self.rect_obj, color)
                .unwrap();
    }
}

#[derive(Clone)]
pub struct Rectangle {
    pub rect_obj: Rect,
    pub rect_mesh: graphics::Mesh,
    pub text: String,
}

impl Rectangle {
    pub fn new(
        ctx: &mut Context,
        _place_x: f32,
        _place_y: f32,
        size_x: f32,
        size_y: f32,
        text: String,
        color: ggez::graphics::Color,
    ) -> Self {
        let rect_obj = graphics::Rect::new(_place_x, _place_y, size_x, size_y);
        //error handle
        let rect_mesh =
            graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect_obj, color);
        let res = Rectangle {
            rect_obj: rect_obj,
            rect_mesh: rect_mesh.unwrap(),
            text: text,
        };
        res
    }
}

extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::Duration;

//Square is the smallest structural point of the game.
//It's the tiles that make up the battlefield.
pub struct Square {
    pub owner: String,
    pub population: i32,
    pub rect_obj: Rect,
    pub color: i32,
    pub searched: bool,
    pub can_create_on: bool,
    pub i: i32,
    pub j: i32,
    pub action_done: bool,
}

type Row = Vec<Square>;

impl Square {
    pub fn new(
        rect_obj: Rect,
        color: i32,
        i: i32,
        j: i32,
    ) -> Self {
        let res = Square {
            owner: "Free Men".to_string(),
            population: 10,
            rect_obj: rect_obj,
            color: color,
            searched: false,
            can_create_on: true,
            i: i,
            j: j,
            action_done: false,
        };
        res
    }
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Become me", 1280, 720)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let rect_width = 40; //32 columns
    let rect_height = 40; //18 rows

    let rect = Rect::new(1, 0, rect_width, rect_height);

    let mut map: Vec<Row> = Vec::new();

    canvas.set_draw_color(Color::RGB(255, 64, 0));
    canvas.draw_rect(rect).unwrap();
    canvas.fill_rect(rect).unwrap();

    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 64, 255));
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

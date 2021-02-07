extern crate ggez;

use ggez::graphics::{Color, DrawMode, DrawParam};
use ggez::graphics::Rect;
use ggez::event::{self, EventHandler};
use ggez::{graphics, Context, ContextBuilder, GameResult};


//Square is the smallest structural point of the game.
//It's the tiles that make up the battlefield.
pub struct Square {
    pub owner: String,
    pub population: i32,
    pub rect_obj: Rect,
    pub rect_mesh: graphics::Mesh,
    pub color: ggez::graphics::Color,
    pub searched: bool,
    pub can_create_on: bool,
    pub i: i32,
    pub j: i32,
    pub action_done: bool,
}

type Row = Vec<Square>;

impl Square {
    pub fn new(ctx: &mut Context, place_x: f32, place_y: f32, color: ggez::graphics::Color, i: i32, j: i32) -> Self {

        let rect = graphics::Rect::new(place_x, place_y, 40.0, 40.0);
        //error handle
        let r = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            rect,
            color,
        );
        let res = Square {
            owner: "Free Men".to_string(),
            population: 10,
            rect_obj: rect,
            rect_mesh: r.unwrap(),
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

fn main() {
    // Make a Context.
    let (ctx, event_loop) = &mut ggez::ContextBuilder::new("Become me", "aa")
        .window_mode(ggez::conf::WindowMode::default().dimensions(1280.0, 720.0))
        .build()
        .unwrap();

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let mut my_game = MyGame::new(ctx);
    // Run!
    //error handle
    event::run(ctx, event_loop, &mut my_game).unwrap();
}

struct MyGame {
    // Your state here...
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        // Load/create resources such as images here.
        MyGame {
            // ...
        }
    }
}


impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // Update code here...
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        //let rect_width = 40; //32 columns
        //let rect_height = 40; //18 rows
        let mut place_x = 560.0;
        let mut place_y = 0.0;
        let mut map: Vec<Row> = Vec::new();
        for i in 1..18 {
            place_x = 560.0;
            let mut row = Row::new();
            for j in 1..18 {
                let rect = Square::new(ctx,place_x, place_y, RED, i, j);
                graphics::draw(ctx, &rect.rect_mesh, DrawParam::default())?;
                let mut scoreboard_text = graphics::Text::new(format!("{}", rect.population));
                scoreboard_text.set_font(graphics::Font::default(), graphics::Scale::uniform(25.0));
        
                let coords = [place_x, place_y];
        
                let params = graphics::DrawParam::default().dest(coords);
                graphics::draw(ctx, &scoreboard_text, params).expect("error drawing scoreboard text");

                row.push(rect);
                place_x += 45.0;
            }
            map.push(row);
            place_y += 45.0;
        }


        graphics::present(ctx)
    }
}

//TODO get these their own place


/// White
pub const WHITE: Color = Color {
    r: 1.0,
    g: 1.0,
    b: 1.0,
    a: 1.0,
};

/// Black
pub const BLACK: Color = Color {
    r: 0.0,
    g: 0.0,
    b: 0.0,
    a: 1.0,
};

/// Red
pub const RED: Color = Color {
    r: 1.0,
    g: 0.0,
    b: 0.0,
    a: 1.0,
};

/// Green
pub const GREEN: Color = Color {
    r: 0.0,
    g: 1.0,
    b: 0.0,
    a: 1.0,
};

/// Blue
pub const BLUE: Color = Color {
    r: 0.0,
    g: 0.0,
    b: 1.0,
    a: 1.0,
};

/// Cyan
pub const CYAN: Color = Color {
    r: 0.0,
    g: 1.0,
    b: 1.0,
    a: 1.0,
};

/// Magenta
pub const MAGENTA: Color = Color {
    r: 1.0,
    g: 0.0,
    b: 1.0,
    a: 1.0,
};

/// Yellow
pub const YELLOW: Color = Color {
    r: 1.0,
    g: 1.0,
    b: 0.0,
    a: 1.0,
};

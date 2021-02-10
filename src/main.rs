extern crate ggez;
extern crate rand;

use ggez::event::{self, EventHandler};
use ggez::graphics::Rect;
use ggez::graphics::{Color, DrawParam};
use ggez::{graphics, Context, GameResult};
use rand::{thread_rng, Rng};

//Square is the smallest structural pou32 of the game.
//It's the tiles that make up the battlefield.
pub struct Square {
    pub owner: String,
    pub population: u32,
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
    pub fn new(
        ctx: &mut Context,
        _place_x: f32,
        _place_y: f32,
        color: ggez::graphics::Color,
        i: i32,
        j: i32,
    ) -> Self {
        let rect = graphics::Rect::new(_place_x, _place_y, 40.0, 40.0);
        //error handle
        let r = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, color);
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
    //pub OwnedTiles:      []OwnedPoint,
    //pub Inventory:       []TempItem,
    pub inventory_size: u32,
    pub artefact_counter: u32,
}

impl Leader {
    pub fn new(name: String, color: ggez::graphics::Color) -> Self {
        // Exclusive range
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
            //pub OwnedTiles:      []OwnedPoint,
            //pub Inventory:       []TempItem,
            inventory_size: 0,
            artefact_counter: 0,
        };
        res
    }
}

//endpoints is local for a reason.
pub struct  Endpoint  {
    pub x: i32,
    pub y: i32,
}
impl Endpoint {
    pub fn new(x: i32, y: i32) -> Self {
        let res = Endpoint {
            x: x,
            y: y,
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
    pub map: Vec<Row>,
    pub players: Vec<Leader>,
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        let mut _place_x = 560.0;
        let mut _place_y = 0.0;
        let mut map: Vec<Row> = Vec::new();
        let mut corners: Vec<Endpoint> = Vec::new();
        
        corners.push(Endpoint::new(0, 0));
        corners.push(Endpoint::new(15, 0));
        corners.push(Endpoint::new(0, 15));
        corners.push(Endpoint::new(15, 15));
        

        for i in 1..18 {
            _place_x = 560.0;
            let mut row = Row::new();
            for j in 1..18 {
                let rect = Square::new(_ctx, _place_x, _place_y, RED, i, j);
                row.push(rect);
                _place_x += 45.0;
            }
            map.push(row);
            _place_y += 45.0;
        }

        
        MyGame {
            map : map,
            players: vec![],
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
        for i in 0..17 {
            for j in 0..17 {
                graphics::draw(ctx, &self.map[i][j].rect_mesh, DrawParam::default())?;
                let mut scoreboard_text = graphics::Text::new(format!("{}", self.map[i][j].population));
                scoreboard_text.set_font(graphics::Font::default(), graphics::Scale::uniform(25.0));

                let coords = [self.map[i][j].rect_obj.x, self.map[i][j].rect_obj.y];

                let params = graphics::DrawParam::default().dest(coords);
                graphics::draw(ctx, &scoreboard_text, params)
                    .expect("error drawing scoreboard text");
            }
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

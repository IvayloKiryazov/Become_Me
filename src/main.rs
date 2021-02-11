extern crate ggez;
extern crate rand;

use ggez::event::{self, EventHandler};
use ggez::graphics::Rect;
use ggez::graphics::{Color, DrawParam};
use ggez::{graphics, Context, GameResult};
use rand::Rng;

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

    pub fn change_color(&mut self, ctx: &mut Context, color: ggez::graphics::Color) {
        self.rect_mesh =
            graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), self.rect_obj, color)
                .unwrap();
    }
}

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
    //pub OwnedTiles:      []OwnedPoint,
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
            //pub OwnedTiles:      []OwnedPoint,
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
                /*player.OwnedTiles = append(player.OwnedTiles,
                MakeOwnedPoint(column, row, map[column][row].RectObj.X,
                    map[column][row].RectObj.Y));*/
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

//just use point :)
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

pub struct UI {
    pub curr_player: Leader,
    pub actions: Vec<Rectangle>,
    pub curr_square: Square,
}

impl UI {
    pub fn new(ctx: &mut Context, curr_player: Leader, curr_square: Square) -> Self {
        let mut actions = Vec::new();
        let rect = Rectangle::new(ctx, 0.0, 200.0, 260.0, 260.0, "Move".to_string(), CYAN);
        actions.push(rect);

        let res = UI {
            curr_player: curr_player,
            actions: actions,
            curr_square: curr_square,
        };
        res
    }
}

fn main() {
    let (ctx, event_loop) = &mut ggez::ContextBuilder::new("Become me", "aa")
        .window_mode(ggez::conf::WindowMode::default().dimensions(1280.0, 720.0))
        .build()
        .unwrap();

    let mut my_game = MyGame::new(ctx);

    //error handle
    event::run(ctx, event_loop, &mut my_game).unwrap();
}

struct MyGame {
    pub map: Vec<Row>,
    pub players: Vec<Leader>,
    pub ui: UI,
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        let mut _place_x = 560.0;
        let mut _place_y = 0.0;
        let mut map: Vec<Row> = Vec::new();

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

        let mut corners: Vec<Endpoint> = Vec::new();

        corners.push(Endpoint::new(0, 0));
        corners.push(Endpoint::new(15, 0));
        corners.push(Endpoint::new(0, 15));
        corners.push(Endpoint::new(15, 15));

        let mut players: Vec<Leader> = Vec::new();
        let mut player_colors: Vec<Color> = Vec::new();

        player_colors.push(BLUE);
        player_colors.push(GREEN);
        player_colors.push(CYAN);
        player_colors.push(MAGENTA);

        for i in 0..4 {
            let player = Leader::new(format!("Player{}", i), player_colors[i]);
            players.push(player);
        }

        for (pos, e) in corners.iter().enumerate() {
            players[pos].starting_village(_ctx, e.x as usize, e.y as usize, &mut map, pos);
        }

        let mut ui = UI::new(_ctx, players[0].clone(), map[0][0].clone());

        MyGame {
            map: map,
            players: players,
            ui: ui,
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
                let mut population = graphics::Text::new(format!("{}", self.map[i][j].population));
                population.set_font(graphics::Font::default(), graphics::Scale::uniform(25.0));

                let coords = [
                    self.map[i][j].rect_obj.x + 5.0,
                    self.map[i][j].rect_obj.y + 5.0,
                ];

                let params = graphics::DrawParam::default().dest(coords);
                //err
                graphics::draw(ctx, &population, params).unwrap();
            }
        }
        graphics::draw(ctx, &self.ui.actions[0].rect_mesh, DrawParam::default())?;

        let mut population = graphics::Text::new(format!("{}", self.ui.actions[0].text));
        population.set_font(graphics::Font::default(), graphics::Scale::uniform(25.0));

        let coords = [
            self.ui.actions[0].rect_obj.x + 5.0,
            self.ui.actions[0].rect_obj.y + 5.0,
        ];

        let params = graphics::DrawParam::default().dest(coords);
        //err
        graphics::draw(ctx, &population, params).unwrap();
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

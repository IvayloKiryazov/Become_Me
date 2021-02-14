extern crate ggez;
extern crate rand;

use ggez::conf::WindowSetup;
use ggez::event::{self, EventHandler};
use ggez::graphics::Rect;
use ggez::graphics::{Color, DrawParam};
use ggez::input::mouse::MouseButton;
use ggez::{graphics, Context, GameResult};
use rand::Rng;

//TODO get these their own place
pub const YELLOW: usize = 7;
pub const BLUE: usize = 6;
pub const GRAY: usize = 5;
pub const GREEN: usize = 4;
pub const RED: usize = 3;
pub const BROWN: usize = 2;
pub const PURPLE: usize = 1;
pub const CYAN: usize = 0;

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

type Row = Vec<Square>;

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
            "UseItem".to_string(),
            color_pallete[CYAN],
        );
        actions.push(rect);
        let rect = Rectangle::new(
            ctx,
            205.0,
            610.0,
            200.0,
            100.0,
            "Time Left".to_string(),
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
            "End Turn".to_string(),
            color_pallete[PURPLE],
        );
        actions.push(rect);

        let rect = Rectangle::new(
            ctx,
            0.0,
            0.0,
            555.0,
            195.0,
            "Your stats:".to_string(),
            color_pallete[BROWN],
        );
        actions.push(rect);

        let rect = Rectangle::new(
            ctx,
            0.0,
            200.0,
            555.0,
            145.0,
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

fn main() {
    let (ctx, event_loop) = &mut ggez::ContextBuilder::new("Become me", "Ivaylo Kiryazov")
        .window_setup(WindowSetup::default().title("Become me!"))
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
    pub color_pallete: Vec<Color>,
    pub field_click: bool,
    pub populate: bool,
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        let mut color_pallete: Vec<Color> = Vec::new();

        let cyan = ggez::graphics::Color::from_rgb_u32(0x4D8AB5);
        color_pallete.push(cyan);
        let purple = ggez::graphics::Color::from_rgb_u32(0x330066);
        color_pallete.push(purple);
        let brown = ggez::graphics::Color::from_rgb_u32(0x4D1518);
        color_pallete.push(brown);

        let red = ggez::graphics::Color::from_rgb_u32(0xC72523);
        color_pallete.push(red);
        let green = ggez::graphics::Color::from_rgb_u32(0x35B535);
        color_pallete.push(green);
        let gray = ggez::graphics::Color::from_rgb_u32(0x6A6A6A);
        color_pallete.push(gray);
        let blue = ggez::graphics::Color::from_rgb_u32(0x2C2A89);
        color_pallete.push(blue);
        let yellow = ggez::graphics::Color::from_rgb_u32(0xFBCA03);
        color_pallete.push(yellow);

        let mut _place_x = 560.0;
        let mut _place_y = 0.0;
        let mut map: Vec<Row> = Vec::new();

        for i in 0..17 {
            _place_x = 560.0;
            let mut row = Row::new();
            for j in 0..17 {
                let rect = Square::new(_ctx, _place_x, _place_y, color_pallete[GRAY], i, j);
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

        player_colors.push(color_pallete[BLUE]);
        player_colors.push(color_pallete[GREEN]);
        player_colors.push(color_pallete[CYAN]);
        player_colors.push(color_pallete[RED]);

        for i in 0..4 {
            let player = Leader::new(format!("Player{}", i), player_colors[i]);
            players.push(player);
        }

        for (pos, e) in corners.iter().enumerate() {
            players[pos].starting_village(_ctx, e.x as usize, e.y as usize, &mut map, pos);
        }

        let mut _ui = UI::new(
            _ctx,
            players[0].clone(),
            map[0][0].clone(),
            map[0][0].clone(),
            color_pallete.clone(),
        );

        MyGame {
            map: map,
            players: players,
            ui: _ui,
            color_pallete: color_pallete,
            field_click: false,
            populate: false,
        }
    }
}

pub fn mouse_clicked_on_action(actions: Vec<Rectangle>, x: f32, y: f32) -> Option<Rectangle> {
    for i in 0..5 {
        if x >= actions[i].rect_obj.x
            && x <= actions[i].rect_obj.x + 200.0
            && y >= actions[i].rect_obj.y
            && y <= actions[i].rect_obj.y + 100.0
        {
            return Some(actions[i].clone());
        }
    }

    return None;
}

pub fn mouse_clicked_on_field(map: Vec<Row>, x: f32, y: f32) -> Option<Square> {
    // TODO make size of map+ 1
    let mut column = 20;
    let mut row = 20;
    for i in 0..17 {
        if x >= map[0][i].rect_obj.x && x <= map[0][i].rect_obj.x + 40.0 {
            column = i;
            for j in 0..17 {
                if y >= map[j][column].rect_obj.y && y <= map[j][column].rect_obj.y + 40.0 {
                    row = j;
                    break;
                }
            }
            break;
        }
    }

    if row == 20 || column == 20 {
        return None;
    }
    Some(map[row][column].clone())
}

pub fn player_owned(map: Vec<Position>, position: Position) -> bool {
    for e in map.iter() {
        if e == &position {
            return true;
        }
    }

    return false;
}

impl EventHandler for MyGame {
    //TODO: make keyboard shortcuts
    //ТODO make stuff un-usable after they've been used
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        if self.field_click {
            self.map[self.ui.prev_square.i][self.ui.prev_square.j].color = self.ui.prev_color;
            self.map[self.ui.prev_square.i][self.ui.prev_square.j].rect_mesh =
                graphics::Mesh::new_rectangle(
                    _ctx,
                    graphics::DrawMode::fill(),
                    self.ui.prev_square.rect_obj,
                    self.ui.prev_color,
                )
                .unwrap();
            self.map[self.ui.curr_square.i][self.ui.curr_square.j].color =
                self.color_pallete[YELLOW];
            self.map[self.ui.curr_square.i][self.ui.curr_square.j].rect_mesh =
                graphics::Mesh::new_rectangle(
                    _ctx,
                    graphics::DrawMode::fill(),
                    self.ui.curr_square.rect_obj,
                    self.color_pallete[YELLOW],
                )
                .unwrap();
        }

        if self.populate {
            let i = self.ui.curr_square.i;
            let j = self.ui.curr_square.j;
            if self.field_click
                && player_owned(self.ui.curr_player.owned_tiles.clone(), Position::new(i, j))
            {
                if self.map[i][j].population < 50 {
                    let increase = self.map[i][j].population / 2;
                    if (self.map[i][j].population + increase) >= 50 {
                        let diff = 50 - self.map[i][j].population;
                        self.map[i][j].population = 50;
                        self.ui.curr_player.population += diff;
                    } else {
                        self.map[i][j].population += increase;
                        self.ui.curr_player.population += increase;
                    }
                    self.ui.curr_square.population = self.map[i][j].population;
                    for (pos, _e) in self.players.clone().iter().enumerate() {
                        if self.players[pos].name == self.ui.curr_player.name {
                            self.players[pos].population = self.ui.curr_player.population;
                        }
                    }
                }
            }
            self.populate = false;
            self.field_click = false;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);
        for i in 0..17 {
            for j in 0..17 {
                graphics::draw(ctx, &self.map[i][j].rect_mesh, DrawParam::default())?;
            }
        }

        for (pos, _) in self.ui.actions.iter().enumerate() {
            graphics::draw(ctx, &self.ui.actions[pos].rect_mesh, DrawParam::default())?;

            let mut txt = graphics::Text::new(format!("{}", self.ui.actions[pos].text));
            txt.set_font(graphics::Font::default(), graphics::Scale::uniform(30.0));

            let coords = [
                self.ui.actions[pos].rect_obj.x + 5.0,
                self.ui.actions[pos].rect_obj.y + 5.0,
            ];

            let params = graphics::DrawParam::default().dest(coords);
            //err
            graphics::draw(ctx, &txt, params).unwrap();
        }

        let mut txt = graphics::Text::new(format!("Owner: {}", self.ui.curr_square.owner));
        txt.set_font(graphics::Font::default(), graphics::Scale::uniform(20.0));

        let coords = [
            self.ui.actions[6].rect_obj.x + 5.0,
            self.ui.actions[6].rect_obj.y + 40.0,
        ];

        let params = graphics::DrawParam::default().dest(coords);
        //err
        graphics::draw(ctx, &txt, params).unwrap();

        let mut txt =
            graphics::Text::new(format!("Population: {}", self.ui.curr_square.population));
        txt.set_font(graphics::Font::default(), graphics::Scale::uniform(20.0));

        let coords = [
            self.ui.actions[6].rect_obj.x + 5.0,
            self.ui.actions[6].rect_obj.y + 65.0,
        ];

        let params = graphics::DrawParam::default().dest(coords);
        //err
        graphics::draw(ctx, &txt, params).unwrap();

        let mut txt =
            graphics::Text::new(format!("Can craft: {}", self.ui.curr_square.can_create_on));
        txt.set_font(graphics::Font::default(), graphics::Scale::uniform(20.0));

        let coords = [
            self.ui.actions[6].rect_obj.x + 5.0,
            self.ui.actions[6].rect_obj.y + 90.0,
        ];

        let params = graphics::DrawParam::default().dest(coords);
        //err
        graphics::draw(ctx, &txt, params).unwrap();

        let mut txt = graphics::Text::new(format!("Usable: {}", self.ui.curr_square.usable));
        txt.set_font(graphics::Font::default(), graphics::Scale::uniform(20.0));

        let coords = [
            self.ui.actions[6].rect_obj.x + 5.0,
            self.ui.actions[6].rect_obj.y + 115.0,
        ];

        let params = graphics::DrawParam::default().dest(coords);
        //err
        graphics::draw(ctx, &txt, params).unwrap();

        let mut txt = graphics::Text::new(format!("Searched: {}", self.ui.curr_square.searched));
        txt.set_font(graphics::Font::default(), graphics::Scale::uniform(20.0));

        let coords = [
            self.ui.actions[6].rect_obj.x + 5.0,
            self.ui.actions[6].rect_obj.y + 140.0,
        ];

        let params = graphics::DrawParam::default().dest(coords);
        //err
        graphics::draw(ctx, &txt, params).unwrap();

        let mut txt = graphics::Text::new(format!("Influence: {}", self.ui.curr_player.influence));
        txt.set_font(graphics::Font::default(), graphics::Scale::uniform(20.0));

        let coords = [
            self.ui.actions[8].rect_obj.x + 5.0,
            self.ui.actions[8].rect_obj.y + 40.0,
        ];

        let params = graphics::DrawParam::default().dest(coords);
        //err
        graphics::draw(ctx, &txt, params).unwrap();

        let mut txt = graphics::Text::new(format!("Science: {}", self.ui.curr_player.science));
        txt.set_font(graphics::Font::default(), graphics::Scale::uniform(20.0));

        let coords = [
            self.ui.actions[8].rect_obj.x + 5.0,
            self.ui.actions[8].rect_obj.y + 65.0,
        ];

        let params = graphics::DrawParam::default().dest(coords);
        //err
        graphics::draw(ctx, &txt, params).unwrap();

        let mut txt = graphics::Text::new(format!("Fertility: {}", self.ui.curr_player.fertility));
        txt.set_font(graphics::Font::default(), graphics::Scale::uniform(20.0));

        let coords = [
            self.ui.actions[8].rect_obj.x + 5.0,
            self.ui.actions[8].rect_obj.y + 90.0,
        ];

        let params = graphics::DrawParam::default().dest(coords);
        //err
        graphics::draw(ctx, &txt, params).unwrap();

        let mut txt = graphics::Text::new(format!("Diplomacy: {}", self.ui.curr_player.diplomacy));
        txt.set_font(graphics::Font::default(), graphics::Scale::uniform(20.0));

        let coords = [
            self.ui.actions[8].rect_obj.x + 5.0,
            self.ui.actions[8].rect_obj.y + 115.0,
        ];

        let params = graphics::DrawParam::default().dest(coords);
        //err
        graphics::draw(ctx, &txt, params).unwrap();

        let mut txt = graphics::Text::new(format!("Mastery: {}", self.ui.curr_player.mastery));
        txt.set_font(graphics::Font::default(), graphics::Scale::uniform(20.0));

        let coords = [
            self.ui.actions[8].rect_obj.x + 5.0,
            self.ui.actions[8].rect_obj.y + 140.0,
        ];

        let params = graphics::DrawParam::default().dest(coords);
        //err
        graphics::draw(ctx, &txt, params).unwrap();

        let mut txt =
            graphics::Text::new(format!("Population: {}", self.ui.curr_player.population));
        txt.set_font(graphics::Font::default(), graphics::Scale::uniform(20.0));

        let coords = [
            self.ui.actions[8].rect_obj.x + 5.0,
            self.ui.actions[8].rect_obj.y + 170.0,
        ];

        let params = graphics::DrawParam::default().dest(coords);
        //err
        graphics::draw(ctx, &txt, params).unwrap();

        graphics::present(ctx)
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
        if ggez::input::mouse::button_pressed(_ctx, _button) && _button == MouseButton::Left {
            let cur_square = mouse_clicked_on_field(self.map.clone(), _x, _y);
            if cur_square.is_some() {
                if self.ui.curr_square.color != self.color_pallete[YELLOW] {
                    self.ui.prev_color = self.ui.curr_square.color.clone();
                }
                self.ui.prev_square = self.ui.curr_square.clone();
                self.ui.curr_square = cur_square.unwrap();
                self.field_click = true;
            }

            let action = mouse_clicked_on_action(self.ui.actions.clone(), _x, _y);
            if action.is_some() {
                if action.unwrap().text.contains("Populate") {
                    self.populate = true;
                }
            }
        }
    }
}

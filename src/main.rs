extern crate ggez;
extern crate rand;
extern crate serde;
extern crate serde_json;

pub mod actions;
pub mod items;
pub mod leader;
pub mod map;
pub mod ui;

use ggez::conf::WindowSetup;
use ggez::event::{self, EventHandler};
use ggez::graphics::{Color, DrawParam};
use ggez::input::mouse::MouseButton;
use ggez::timer;
use ggez::{graphics, Context, GameResult};
use rand::Rng;
use std::env;

pub const TURN_TIME: f64 = 30.0;

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

#[derive(PartialEq)]
pub enum State {
    Start,
    Populate,
    Moving,
    Create,
    Search,
    UseItem,
    EndTurn,
}

struct MyGame {
    pub map: Vec<map::Row>,
    pub players: Vec<leader::Leader>,
    pub ui: ui::UI,
    pub color_pallete: Vec<Color>,
    pub field_click: bool,
    pub second_click: bool,
    pub tmp_items: Vec<items::Expandable>,
    pub perm_items: Vec<items::Relics>,
    pub end_time: f64,
    pub game_state: State,
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
        let mut map: Vec<map::Row> = Vec::new();

        for i in 0..17 {
            _place_x = 560.0;
            let mut row = map::Row::new();
            for j in 0..17 {
                let rect =
                    map::Square::new(_ctx, _place_x, _place_y, color_pallete[ui::GRAY], i, j);
                row.push(rect);
                _place_x += 45.0;
            }
            map.push(row);

            _place_y += 45.0;
        }

        let mut corners: Vec<leader::Endpoint> = Vec::new();

        corners.push(leader::Endpoint::new(0, 0));
        corners.push(leader::Endpoint::new(15, 0));
        corners.push(leader::Endpoint::new(0, 15));
        corners.push(leader::Endpoint::new(15, 15));

        let mut players: Vec<leader::Leader> = Vec::new();
        let mut player_colors: Vec<Color> = Vec::new();

        player_colors.push(color_pallete[ui::BLUE]);
        player_colors.push(color_pallete[ui::GREEN]);
        player_colors.push(color_pallete[ui::CYAN]);
        player_colors.push(color_pallete[ui::RED]);

        for i in 0..4 {
            let player = leader::Leader::new(format!("Player{}", i), player_colors[i]);
            players.push(player);
        }

        for (pos, e) in corners.iter().enumerate() {
            players[pos].starting_village(_ctx, e.x as usize, e.y as usize, &mut map, pos);
        }

        let mut _ui = ui::UI::new(
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
            second_click: false,
            /*
            // for debug :)
            tmp_items: items::read_expandables(format!(
                "{}\\..\\..\\src\\tempitems.json",
                env::current_dir().unwrap().display()
            )),
            perm_items: items::read_relics(format!(
                "{}\\..\\..\\src\\permanentitems.json",
                env::current_dir().unwrap().display()
            )),*/
            tmp_items: items::read_expandables(format!(
                "{}\\src\\tempitems.json",
                env::current_dir().unwrap().display()
            )),
            perm_items: items::read_relics(format!(
                "{}\\src\\permanentitems.json",
                env::current_dir().unwrap().display()
            )),
            end_time: timer::duration_to_f64(timer::time_since_start(_ctx)).trunc() + TURN_TIME,
            game_state: State::Start,
        }
    }
}

pub fn draw_text(ctx: &mut Context, text: String, x: f32, y: f32, size: f32) {
    let mut txt = graphics::Text::new(text);
    txt.set_font(graphics::Font::default(), graphics::Scale::uniform(size));

    let coords = [x, y];

    let params = graphics::DrawParam::default().dest(coords);
    //err
    graphics::draw(ctx, &txt, params).unwrap();
}

impl MyGame {
    fn highlight_tile(&mut self, _ctx: &mut Context) {
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
                self.color_pallete[ui::YELLOW];
            self.map[self.ui.curr_square.i][self.ui.curr_square.j].rect_mesh =
                graphics::Mesh::new_rectangle(
                    _ctx,
                    graphics::DrawMode::fill(),
                    self.ui.curr_square.rect_obj,
                    self.color_pallete[ui::YELLOW],
                )
                .unwrap();
        }
    }

    fn update_player(&mut self) {
        for (pos, _e) in self.players.clone().iter().enumerate() {
            if self.players[pos].name == self.ui.curr_player.name {
                self.players[pos] = self.ui.curr_player.clone();
            }
        }
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // comparing floats is fun :)
        if (self.end_time - timer::duration_to_f64(timer::time_since_start(_ctx)).trunc()) <= 0.0 {
            self.game_state = State::EndTurn;
        }
        self.highlight_tile(_ctx);

        let i = self.ui.curr_square.i;
        let j = self.ui.curr_square.j;

        if self.game_state == State::Populate && self.field_click {
            if actions::player_owned(
                self.ui.curr_player.owned_tiles.clone(),
                leader::Position::new(i, j),
            ) {
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
                    self.update_player();
                }

                self.map[i][j].color = self.ui.curr_player.color;
                self.map[i][j].rect_mesh = graphics::Mesh::new_rectangle(
                    _ctx,
                    graphics::DrawMode::fill(),
                    self.ui.curr_square.rect_obj,
                    self.ui.curr_player.color,
                )
                .unwrap();
            }
            self.game_state = State::Start;
            self.field_click = false;
        } else if self.game_state == State::Moving && self.second_click {
            let toi = i;
            let toj = j;
            let fromi = self.ui.prev_square.i;
            let fromj = self.ui.prev_square.j;

            if actions::player_owned(
                self.ui.curr_player.owned_tiles.clone(),
                leader::Position::new(fromi, fromj),
            ) && actions::is_adjacent(
                leader::Endpoint::new(toi as i32, toj as i32),
                leader::Endpoint::new(fromi as i32, fromj as i32),
            ) && self.map[fromi][fromj].population > 0
            {
                if self.map[fromi][fromj].owner == self.map[toi][toj].owner {
                    // no battle
                    self.map[toi][toj].population += self.map[fromi][fromj].population;
                    self.map[fromi][fromj].population = 0;
                } else {
                    //we win the battle
                    if self.map[fromi][fromj].population >= self.map[toi][toj].population {
                        let result =
                            self.map[fromi][fromj].population - self.map[toi][toj].population;
                        let loss = self.map[toi][toj].population;
                        self.map[toi][toj].population = result;
                        self.ui.curr_player.population -= loss;

                        self.map[fromi][fromj].population = 0;
                        self.map[toi][toj].owner = self.map[fromi][fromj].owner.clone();

                        self.update_player();

                        //TODO test more
                        if !self.map[toi][toj].owner.contains("Ol") {
                            for (pos, _e) in self.players.clone().iter().enumerate() {
                                if self.players[pos].name == self.map[toi][toj].owner {
                                    for (p, _el) in
                                        self.players[pos].owned_tiles.clone().iter().enumerate()
                                    {
                                        if self.players[pos].owned_tiles[p].x == toi
                                            && self.players[pos].owned_tiles[p].y == toj
                                        {
                                            self.players[pos].owned_tiles.remove(p);
                                        }
                                    }
                                    self.players[pos].population -= loss;
                                }
                            }
                        }

                        self.ui
                            .curr_player
                            .owned_tiles
                            .push(leader::Position::new(toi, toj));
                    } else {
                        // we lose the battle
                        let loss = self.map[fromi][fromj].population;
                        self.map[toi][toj].population -= loss;
                        self.ui.curr_player.population -= loss;
                        self.map[fromi][fromj].population = 0;

                        self.update_player();

                        if !self.map[toi][toj].owner.contains("Ol") {
                            for (pos, _e) in self.players.clone().iter().enumerate() {
                                if self.players[pos].name == self.map[toi][toj].owner {
                                    self.players[pos].population -= loss;
                                }
                            }
                        }
                    }
                }

                self.ui.curr_square = self.map[toi][toj].clone();
                self.map[toi][toj].color = self.ui.curr_player.color;
                self.map[toi][toj].rect_mesh = graphics::Mesh::new_rectangle(
                    _ctx,
                    graphics::DrawMode::fill(),
                    self.ui.curr_square.rect_obj,
                    self.ui.curr_player.color,
                )
                .unwrap();
            }
            self.game_state = State::Start;
            self.field_click = false;
            self.second_click = false;
        } else if self.game_state == State::Create && self.field_click {
            if self.ui.curr_player.inventory.len() < 20
                && self.map[i][j].can_create_on
                && actions::player_owned(
                    self.ui.curr_player.owned_tiles.clone(),
                    leader::Position::new(i, j),
                )
            {
                let mut rng = rand::thread_rng();
                let item = rng.gen_range(0..(self.tmp_items.len() - 1));
                self.ui
                    .curr_player
                    .inventory
                    .push(self.tmp_items[item].clone());
                // TODO make true at end of turn
                self.map[i][j].can_create_on = false;

                self.update_player();

                self.map[i][j].color = self.ui.curr_player.color;
                self.map[i][j].rect_mesh = graphics::Mesh::new_rectangle(
                    _ctx,
                    graphics::DrawMode::fill(),
                    self.ui.curr_square.rect_obj,
                    self.ui.curr_player.color,
                )
                .unwrap();
            }
            self.game_state = State::Start;
            self.field_click = false;
        } else if self.game_state == State::Search && self.field_click {
            if !self.map[i][j].searched
                && actions::player_owned(
                    self.ui.curr_player.owned_tiles.clone(),
                    leader::Position::new(i, j),
                )
            {
                // make name appear in inventory.
                if self.ui.curr_player.search_counter == 25 {
                    let mut rng = rand::thread_rng();
                    let item = rng.gen_range(0..self.perm_items.len());
                    let buff: items::Relics = self.perm_items[item].clone();
                    self.perm_items.remove(item);
                    self.ui.curr_player.diplomacy += buff.diplomacy;
                    self.ui.curr_player.fertility += buff.fertility;
                    self.ui.curr_player.science += buff.science;
                    self.ui.curr_player.influence += buff.influence;
                    self.ui.curr_player.mastery += buff.mastery;
                } else {
                    let mut rng = rand::thread_rng();
                    let dice = rng.gen_range(1..26);
                    if dice == 25 {
                        let item = rng.gen_range(0..self.perm_items.len());
                        let buff: items::Relics = self.perm_items[item].clone();
                        self.perm_items.remove(item);
                        self.ui.curr_player.diplomacy += buff.diplomacy;
                        self.ui.curr_player.fertility += buff.fertility;
                        self.ui.curr_player.science += buff.science;
                        self.ui.curr_player.influence += buff.influence;
                        self.ui.curr_player.mastery += buff.mastery;
                    } else {
                        self.ui.curr_player.search_counter += 1;
                    }
                }
                self.update_player();
                self.map[i][j].color = self.ui.curr_player.color;
                self.map[i][j].rect_mesh = graphics::Mesh::new_rectangle(
                    _ctx,
                    graphics::DrawMode::fill(),
                    self.ui.curr_square.rect_obj,
                    self.ui.curr_player.color,
                )
                .unwrap();
                self.map[i][j].searched = true;
                self.game_state = State::Start;
                self.field_click = false;
            }
        } else if self.game_state == State::UseItem {
            //Only action that does not require tile click
            if self.ui.curr_player.inventory.len() > 0 {
                let mut rng = rand::thread_rng();
                let item = rng.gen_range(0..self.ui.curr_player.inventory.len());
                let buff: items::Expandable = self.ui.curr_player.inventory[item].clone();
                self.ui.curr_player.inventory.remove(item);
                self.ui.curr_player.diplomacy += buff.diplomacy;
                self.ui.curr_player.fertility += buff.fertility;
                self.ui.curr_player.science += buff.science;
                self.ui.curr_player.influence += buff.influence;
                self.ui.curr_player.mastery += buff.mastery;
                self.update_player();
            }
            self.game_state = State::Start;
        //TODO: at the end we have to reduce the stats.
        } else if self.game_state == State::EndTurn {
            self.update_player();
            for (pos, _e) in self.players.clone().iter().enumerate() {
                if self.players[pos].name == self.ui.curr_player.name {
                    if pos + 1 == self.players.len() {
                        self.ui.curr_player = self.players[0].clone();
                    } else {
                        self.ui.curr_player = self.players[pos + 1].clone();
                    }
                    break;
                }
            }
            self.end_time =
                timer::duration_to_f64(timer::time_since_start(_ctx)).trunc() + TURN_TIME;
            self.game_state = State::Start;
            self.field_click = false;
            self.second_click = false;
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
            draw_text(
                ctx,
                format!("{}", self.ui.actions[pos].text),
                self.ui.actions[pos].rect_obj.x + 5.0,
                self.ui.actions[pos].rect_obj.y + 5.0,
                30.0,
            );
        }

        // Tile info
        draw_text(
            ctx,
            format!("Owner: {}", self.ui.curr_square.owner),
            self.ui.actions[6].rect_obj.x + 5.0,
            self.ui.actions[6].rect_obj.y + 40.0,
            20.0,
        );

        draw_text(
            ctx,
            format!("Population: {}", self.ui.curr_square.population),
            self.ui.actions[6].rect_obj.x + 5.0,
            self.ui.actions[6].rect_obj.y + 65.0,
            20.0,
        );

        draw_text(
            ctx,
            format!("Can craft: {}", self.ui.curr_square.can_create_on),
            self.ui.actions[6].rect_obj.x + 5.0,
            self.ui.actions[6].rect_obj.y + 90.0,
            20.0,
        );

        draw_text(
            ctx,
            format!("Usable: {}", self.ui.curr_square.usable),
            self.ui.actions[6].rect_obj.x + 5.0,
            self.ui.actions[6].rect_obj.y + 115.0,
            20.0,
        );

        draw_text(
            ctx,
            format!("Searched: {}", self.ui.curr_square.searched),
            self.ui.actions[6].rect_obj.x + 5.0,
            self.ui.actions[6].rect_obj.y + 140.0,
            20.0,
        );

        //Player info
        draw_text(
            ctx,
            format!("Name: {}", self.ui.curr_player.name),
            self.ui.actions[8].rect_obj.x + 5.0,
            self.ui.actions[8].rect_obj.y + 40.0,
            20.0,
        );

        draw_text(
            ctx,
            format!("Population: {}", self.ui.curr_player.population),
            self.ui.actions[8].rect_obj.x + 5.0,
            self.ui.actions[8].rect_obj.y + 65.0,
            20.0,
        );

        draw_text(
            ctx,
            format!("Science: {}", self.ui.curr_player.science),
            self.ui.actions[8].rect_obj.x + 5.0,
            self.ui.actions[8].rect_obj.y + 90.0,
            20.0,
        );

        draw_text(
            ctx,
            format!("Fertility: {}", self.ui.curr_player.fertility),
            self.ui.actions[8].rect_obj.x + 5.0,
            self.ui.actions[8].rect_obj.y + 115.0,
            20.0,
        );

        draw_text(
            ctx,
            format!("Diplomacy: {}", self.ui.curr_player.diplomacy),
            self.ui.actions[8].rect_obj.x + 5.0,
            self.ui.actions[8].rect_obj.y + 140.0,
            20.0,
        );

        draw_text(
            ctx,
            format!("Mastery: {}", self.ui.curr_player.mastery),
            self.ui.actions[8].rect_obj.x + 5.0,
            self.ui.actions[8].rect_obj.y + 170.0,
            20.0,
        );

        draw_text(
            ctx,
            format!("Influence: {}", self.ui.curr_player.influence),
            self.ui.actions[8].rect_obj.x + 5.0,
            self.ui.actions[8].rect_obj.y + 195.0,
            20.0,
        );


        //Inventory
        let mut initial_y = 35.0;
        for (pos, _) in self.ui.curr_player.inventory.iter().enumerate() {
            draw_text(
                ctx,
                format!("{}", self.ui.curr_player.inventory[pos].name),
                self.ui.actions[9].rect_obj.x + 5.0,
                self.ui.actions[9].rect_obj.y + initial_y,
                20.0,
            );
            initial_y += 20.0;
        }

        //Timer
        draw_text(
            ctx,
            format!(
                "{}",
                self.end_time - timer::duration_to_f64(timer::time_since_start(ctx)).trunc()
            ),
            self.ui.actions[7].rect_obj.x + 5.0,
            self.ui.actions[7].rect_obj.y + 45.0,
            50.0,
        );

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
            let cur_square = actions::mouse_clicked_on_field(self.map.clone(), _x, _y);
            if cur_square.is_some() {
                if self.game_state == State::Moving {
                    self.second_click = true;
                }
                if self.ui.curr_square.color != self.color_pallete[ui::YELLOW] {
                    self.ui.prev_color = self.ui.curr_square.color.clone();
                }
                self.ui.prev_square = self.ui.curr_square.clone();
                self.ui.curr_square = cur_square.unwrap();
                self.field_click = true;
            }

            let action = actions::mouse_clicked_on_action(self.ui.actions.clone(), _x, _y);

            if action.is_some() {
                match action.as_ref().unwrap().text.as_str() {
                    "Populate" => self.game_state = State::Populate,
                    "Move" => self.game_state = State::Moving,
                    "Create" => self.game_state = State::Create,
                    "Search" => self.game_state = State::Search,
                    "Use Item" => self.game_state = State::UseItem,
                    "End Turn" => self.game_state = State::EndTurn,
                    _ => self.game_state = State::Start,
                }
            }
        }
    }
}

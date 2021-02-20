use leader::*;
use map::*;

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
    // special check for END_TURN
    if x >= actions[7].rect_obj.x
        && x <= actions[7].rect_obj.x + 145.0
        && y >= actions[7].rect_obj.y
        && y <= actions[7].rect_obj.y + 100.0
    {
        return Some(actions[7].clone());
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

pub fn is_adjacent(to: Endpoint, from: Endpoint) -> bool {
    let mut is_adjacent = false;
    let mut eight_directions: Vec<Endpoint> = Vec::new();

    eight_directions.push(Endpoint::new(-1, -1));
    eight_directions.push(Endpoint::new(0, 1));
    eight_directions.push(Endpoint::new(1, -1));
    eight_directions.push(Endpoint::new(1, 0));
    eight_directions.push(Endpoint::new(1, 1));
    eight_directions.push(Endpoint::new(0, -1));
    eight_directions.push(Endpoint::new(-1, 1));
    eight_directions.push(Endpoint::new(-1, 0));

    for i in eight_directions {
        let new_row = from.x + i.x;
        let new_column = from.y + i.y;
        if new_row == to.x && new_column == to.y {
            is_adjacent = true;
            break;
        }
    }
    is_adjacent
}

use leader::*;
use map::*;

// UI utility
// The first 6 are clickable buttons
pub fn mouse_clicked_on_action(actions: Vec<Rectangle>, x: f32, y: f32) -> Option<Rectangle> {
    for i in 0..6 {
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

// Determine on which tile we have clicked
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

// Checking if we have it determines that we don't have teleporting units.
pub fn player_owned(map: Vec<Position>, position: Position) -> bool {
    for e in map.iter() {
        if e == &position {
            return true;
        }
    }

    return false;
}


// This is also needed for us to know that we don't have teleporting units.
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


#[test]
fn test_basic_is_owned() {
    let mut  map: Vec<Position> = vec![];
    map.push(Position::new(1,2));
    let position = Position::new(1,2);
    assert_eq!(player_owned(map, position), true);
}  

#[test]
fn test_basic_is_not_owned() {
    let mut  map: Vec<Position> = vec![];
    map.push(Position::new(1,2));
    let position = Position::new(2,2);
    assert_eq!(player_owned(map, position), false);
}  


#[test]
fn test_basic_is_adj() {
    let position1 = Endpoint::new(2,2);
    let position2 = Endpoint::new(1,2);
    assert_eq!(is_adjacent(position1, position2), true);
}  

#[test]
fn test_basic_is_not_adj() {
    let position1 = Endpoint::new(2,2);
    let position2 = Endpoint::new(5,5);
    assert_eq!(is_adjacent(position1, position2), false);
}  
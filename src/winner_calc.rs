use objects::Position;
use BOARD_SIZE;

pub const UP: [(i64, i64); (BOARD_SIZE / 2) as usize] = [(0, 0), (0, 1), (0, 2), (0, 3)];
pub const RIGHT: [(i64, i64); (BOARD_SIZE / 2) as usize] = [(0, 0), (1, 0), (2, 0), (3, 0)];
pub const UP_RIGHT: [(i64, i64); (BOARD_SIZE / 2) as usize] = [(0, 0), (-1, 1), (-2, 2), (-3, 3)];
pub const UP_LEFT: [(i64, i64); (BOARD_SIZE / 2) as usize] = [(0, 0), (1, 1), (2, 2), (3, 3)];

pub fn calculate_winner(positions: &Vec<Position>) -> i64 {
    for p in positions.clone() {
        let x = p.x;
        let y = p.y;
        if y < (BOARD_SIZE - 3) {
            if direction_taken(UP, &x, &y, &positions) {
                return p.player;
            }
            if x > 2 {
                if direction_taken(UP_RIGHT, &x, &y,  &positions) {
                    return p.player;
                }
            }
            if x < (BOARD_SIZE - 3)  {
                if direction_taken(UP_LEFT, &x, &y, &positions) {
                    return p.player;
                }
            }
        }
        if x < (BOARD_SIZE - 3) {
            if direction_taken(RIGHT, &x, &y,  &positions) {
                return p.player;
            } 
        } 
    }
    0
}

pub fn get_pos(x: &i64, y: &i64, positions: &Vec<Position>) -> i64 {
    for p in positions {
        if p.x == *x && p.y == *y {
            return p.player;
        }
    }
    0
}

fn direction_taken(direction: [(i64, i64); 4], x: &i64, y: &i64, positions: &Vec<Position>) -> bool {
    let last = positions.last().unwrap();
    let mut counter = 0;
    for p in direction.iter() {
        let tx = x + p.0;
        let ty = y + p.1;
        let player = get_pos(&tx, &ty, positions);
        if player > 0 {
            if last.player == player {
                counter += 1;
            }
        }
    }
    counter == 4
}


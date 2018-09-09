use objects::Position;
use BOARD_SIZE;
use winner_calc::{UP, RIGHT, UP_RIGHT, UP_LEFT};

const DIRECTIONS:[[(i64, i64); 4]; 4] = [UP, RIGHT, UP_RIGHT, UP_LEFT]; 

pub struct Ai {
}

impl Ai {
    pub fn new() -> Self {
        Ai {}
    }

    pub fn get_move(&self, positions: &Vec<Position>) -> i64 {
        let mut mv = 0;
        let counter = self.get_counter();
        if counter == 0 {
            let direction = self.get_possible_directions(positions);
        } else {
            mv = counter
        }
        mv
    }

    fn get_counter(&self) -> i64 {
        0
    }

    fn get_possible_directions(&self, positions: &Vec<Position>) -> Vec<[(i64, i64); 4]> {
        let mut possible = Vec::<[(i64, i64); 4]>::new();
        for d in DIRECTIONS.iter() {
            for p in d {
                for pos in positions {
                    if p.0 == pos.x && p.1 == pos.y {
                         
                    }
                }
            }
        }
        possible
    }
}


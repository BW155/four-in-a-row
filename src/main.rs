extern crate rprompt;

use draw_board::{clear_board, draw_board};
use objects::{Position, Ai};
use winner_calc::calculate_winner;

mod draw_board;
mod objects;
mod winner_calc;

const BOARD_SIZE: i64 = 8;

fn main() {
    println!("Welcome to four-in-a-row");
    let mut turn: i64 = 0;
    let mut winner: i64 = 0;
    let mut positions: Vec<Position> = Vec::new();
    let mut error: Option<String> = None;
    let ai = Ai::new();

    
    let multiplayer = rprompt::prompt_reply_stdout("(S)ingleplayer/(M)ultiplayer ? (default S): ").unwrap();
    if multiplayer.to_lowercase() == "m".to_string() {    
        let server = rprompt::prompt_reply_stdout("(S)erver/(C)lient ? (default S): ").unwrap();
        if server.to_lowercase() == "s".to_string() {    
            let mut port = 0;
            while port < 1000 {
                clear_board();
                let port_str = rprompt::prompt_reply_stdout("Port 4-digits: (5005): ").unwrap();
                if port_str.len() == 0 {
                    port = 5005;
                } else {
                    port = if let Ok(p) = port_str.parse::<i64>() {
                        p
                    } else {
                        println!("Thats not a number");
                        0
                    };

                }
            }
            
            
        }
    }

    // let enable_ai = rprompt::prompt_reply_stdout("enable ai? y/n: ").unwrap();
    let ai_enabled = false; // enable_ai == "y".to_string();

    while winner == 0 {
        clear_board();
        winner = calculate_winner(&positions);
        match winner {
            1 => show_winner(1),
            2 => show_winner(2),
            _ => {}
        }

        if let Some(err) = error {
            println!("Error: {}", err);
            error = None;
        } else {
            match turn {
                0 => turn = 1,
                1 => turn = 2,
                2 => turn = 1,
                _ => {},
            }
        }
        
        if winner == 0 {
            println!("Player {} is.", turn);
        }
        draw_board(&positions);

        if winner > 0 {
            return;
        }
        
        let row_number = 
            if ai_enabled && turn == 2 {
                Ok(ai.get_move(&positions))
            } else {
                let reply = rprompt::prompt_reply_stdout("Choose row number: ").unwrap();
                reply.parse::<i64>()
            };

        if let Ok(row_number) = row_number {
            if row_number <= BOARD_SIZE && row_number > 0 {
                let row_number = row_number - 1;
                let y = get_y(&row_number, &positions);
                if y < 0 {
                    error = Some("Row is full".to_string());
                } else {
                    let position = Position::new(&row_number, &y, &turn);
                    positions.push(position);
                }
            } else {
                error = Some(format!("Number must be higher than 0 and lower than {}", BOARD_SIZE + 1));
            }
        } else {
            error = Some("Not a number".to_string());
        }
    }
}

fn get_y(x: &i64, positions: &Vec<Position>) -> i64 {
    let mut max_size = 0;
    for p in positions {
        if p.x == *x {
            max_size += 1;
        }
    }
    BOARD_SIZE - 1 - max_size
}

fn show_winner(winner: i64) {
    println!("Player {} WON!!!", winner);
}


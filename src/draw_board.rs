use objects::Position;
use BOARD_SIZE;

pub fn draw_board(positions: &Vec<Position>) {
    for y in 0..BOARD_SIZE {
        for x in 0..BOARD_SIZE {
            let mut found = false;
            for p in positions {
                if p.x == x  && p.y == y {
                    let sent = match p.player {
                        1 => " 🔴 ".to_string(),
                        2 => " 🔵 ".to_string(),
                        _ => format!(" {} ", p.player),
                    };
                    print!("{}", sent);
                    found = true;
                } 
            }
            if !found {
                print!(" ⚫️ ");
            }
        }
        print!("\n")
    }
}

pub fn clear_board() {
    print!("{}[2J", 27 as char);
}


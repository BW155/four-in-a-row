#[derive(Debug)]
pub struct Position {
    pub x: i64,
    pub y: i64,
    pub player: i64,
}

impl Position {
    pub fn new(x: &i64, y: &i64, player: &i64) -> Self {
        Position {
            x: x.to_owned(),
            y: y.to_owned(),
            player: player.to_owned(),
        }
    }
}


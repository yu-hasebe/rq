pub struct Player {
    pub direction: Direction,
}

#[derive(Clone)]
pub enum Direction {
    Left, Up, Right, Down,
}
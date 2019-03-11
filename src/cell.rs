#[derive(Debug, Clone)]
pub struct Cell {
    pub live: bool
}

impl Cell {
    pub fn new() -> Cell {
        Cell { live: false }
    }
}

use crate::cell::Cell;
use crate::world::World;

mod cell;
mod world;
mod rle;
mod standard_error;

fn main() {
    let args = std::env::args().skip(1);

    Cell::new();
    let w = World::new(10, 10);
    println!("{:?}", w.cells);
}

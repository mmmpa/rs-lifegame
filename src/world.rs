use crate::cell::Cell;

#[derive(Debug, Clone)]
pub struct World {
    width: usize,
    height: usize,
    pub cells: Vec<Cell>,
}

impl World {
    pub fn new(width: usize, height: usize) -> World {
        let count = width * height;
        let cells = vec![Cell::new(); count];

        World { width, height, cells }
    }

    pub fn is_live(&self, x: isize, y: isize) -> bool {
        match self.is_in(x, y) {
            Ok((x, y)) => self.cells[self.width * y + x].live,
            _ => false
        }
    }

    pub fn set_live(&mut self, x: isize, y: isize, doa: bool) {
        match self.is_in(x, y) {
            Ok((x, y)) => { self.cells[self.width * y + x].live = doa; },
            _ => ()
        }
    }

    fn is_in(&self, x: isize, y: isize) -> Result<(usize, usize), ()> {
        let w = self.width as isize;
        let h = self.height as isize;

        if x < 0 || w <= x || y < 0 || h <= y {
            return Err(())
        }

        Ok((x as usize, y as usize))
    }
}

#[test]
fn test_is_in() {
    let w = World::new(10, 10);

    assert!(w.is_in(0, -1).is_err());
    assert!(w.is_in(-1, 0).is_err());
    assert_eq!(w.is_in(0, 0), Ok((0, 0)));

    assert_eq!(w.is_in(9, 9), Ok((9, 9)));
    assert!(w.is_in(9, 10).is_err());
    assert!(w.is_in(10, 9).is_err());
}

#[test]
fn test_is_live() {
    let mut w = World::new(10, 10);

    w.cells[99].live = true;
    assert_eq!(w.is_live(9, 8), false);
    assert_eq!(w.is_live(9, 9), true);
}

#[test]
fn test_dead_or_alive() {
    let mut w = World::new(10, 10);

    w.set_live(9, 9, true);
    assert_eq!(w.cells[99].live, true);
}

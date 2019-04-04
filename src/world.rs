#[derive(Debug, Clone)]
pub struct World {
    width: usize,
    height: usize,
    w: isize,
    h: isize,
    pub cells: Vec<bool>,
}

impl World {
    pub fn new(width: usize, height: usize) -> World {
        let count = width * height;
        let cells = vec![false; count];

        World {
            width,
            height,
            w: width as isize,
            h: height as isize,
            cells,
        }
    }

    pub fn is_live(&self, x: isize, y: isize) -> bool {
        match self.is_in(x, y) {
            Ok((x, y)) => unsafe { *self.cells.get_unchecked(self.width * y + x) },
            _ => false
        }
    }

    pub fn set_life(&mut self, x: isize, y: isize, doa: bool) {
        match self.is_in(x, y) {
            Ok((x, y)) => unsafe { *self.cells.get_unchecked_mut(self.width * y + x) = doa},
            _ => ()
        }
    }

    pub fn set_lives(&mut self, x: usize, y: usize, doa: Vec<bool>) {
        let head = self.width * y + x;
        let tail = head + doa.len();

        if tail <= self.cells.len() {
            self.cells.splice(head..tail, doa);
        }
    }

    fn is_in(&self, x: isize, y: isize) -> Result<(usize, usize), ()> {
        if x < 0 || self.w <= x || y < 0 || self.h <= y {
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

    w.cells[99] = true;
    assert_eq!(w.is_live(9, 8), false);
    assert_eq!(w.is_live(9, 9), true);
}

#[test]
fn test_set_vec_live() {
    let mut w = World::new(4, 3);

    w.set_lives(0, 0, vec![true, false, true]);
    w.set_lives(1, 1, vec![true, false, true]);
    w.set_lives(1, 2, vec![true, false, true]);

    assert_eq!(w.is_live(0, 0), true);
    assert_eq!(w.is_live(1, 0), false);
    assert_eq!(w.is_live(2, 0), true);
    assert_eq!(w.is_live(1, 1), true);
    assert_eq!(w.is_live(2, 1), false);
    assert_eq!(w.is_live(3, 1), true);
    assert_eq!(w.is_live(1, 2), true);
    assert_eq!(w.is_live(2, 2), false);
    assert_eq!(w.is_live(3, 2), true);
}

#[test]
fn test_dead_or_alive() {
    let mut w = World::new(10, 10);

    w.set_life(9, 9, true);
    assert_eq!(w.cells[99], true);
}

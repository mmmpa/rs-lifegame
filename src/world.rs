use crate::cell::Cell;

#[derive(Debug)]
pub struct World {
    width: usize,
    height: usize,
    pub cells: Vec<Cell>,
    poses: Poses
}

type Poses = Vec<(isize, isize)>;

impl World {
    pub fn new(width: usize, height: usize) -> World {
        let count = width * height;
        let poses = World::poses();
        let cells = vec![Cell::new(); count];

        World { width, height, cells, poses }
    }

    fn is_live(&self, x: isize, y: isize) -> bool {
        match self.is_in(x, y) {
            Ok((x, y)) => self.cells[self.width * y + x].live,
            _ => false
        }
    }

    fn next_live(&self, x: isize, y: isize) -> bool {
        let now = self.is_live(x, y);

        let lives = self.poses.iter().fold(0, |a, (offset_x, offset_y)| {
            if self.is_live(x + offset_x, y + offset_y) {
                a + 1
            } else {
                a
            }
        });

        match lives {
            3 => true,
            2 => now,
            _ => false
        }
    }

    fn set_live(&mut self, x: isize, y: isize, doa: bool) {
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

    fn poses() -> Poses {
        let xs = vec![-1, 0, 1];
        let ys = vec![-1, 0, 1];

        let mut poses = Vec::with_capacity(8);
        for x in &xs {
            for y in &ys {
                if *x == 0 && *y == 0 {
                    continue;
                }
                poses.push((*x, *y));
            }
        }

        poses
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

#[test]
fn test_next_life() {
    let poses = World::poses();

    assert_eq!(poses, [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)]);
}

#[test]
fn next_live() {
    let mut w = World::new(10, 10);

    w.set_live(1, 1, true);
    w.set_live(2, 1, true);
    w.set_live(3, 1, true);
    assert_eq!(w.next_live(2, 2), true);
    assert_eq!(w.next_live(1, 2), false);

    w.set_live(4, 3, true);
    w.set_live(5, 3, true);
    w.set_live(6, 3, true);
    w.set_live(4, 4, true);
    w.set_live(5, 4, true);
    w.set_live(6, 4, true);
    assert_eq!(w.next_live(5, 3), false);
    assert_eq!(w.next_live(5, 4), false);

    w.set_live(1, 8, true);
    w.set_live(2, 8, true);
    w.set_live(2, 9, true);
    assert_eq!(w.next_live(1, 8), true);
    assert_eq!(w.next_live(1, 9), true);
    assert_eq!(w.next_live(2, 8), true);
    assert_eq!(w.next_live(2, 9), true);
}

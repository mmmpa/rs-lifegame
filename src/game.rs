extern crate num_cpus;

use crate::world::World;
use std::mem::swap;
use std::thread::spawn;
use std::sync::{Arc, RwLock};

pub struct Game {
    pub width: isize,
    pub height: isize,
    world_a: Arc<RwLock<World>>,
    world_b: Arc<RwLock<World>>,
    cpu_num: usize,
    cpu_rows: usize,
}

impl Game {
    pub fn new(width: usize, height: usize, lives: &Vec<bool>) -> Game {
        let mut world_a = World::new(width, height);
        let world_b = World::new(width, height);
        let cpu_num = num_cpus::get();
        let cpu_rows = height / cpu_num + 1;

        for (y, cols) in lives.chunks(width).enumerate() {
            for (x, doa) in cols.iter().enumerate() {
                world_a.set_live(x as isize, y as isize, *doa);
            }
        }

        Game {
            width: width as isize,
            height: height as isize,
            world_a: Arc::new(RwLock::new(world_a)),
            world_b: Arc::new(RwLock::new(world_b)),
            cpu_num,
            cpu_rows,
        }
    }

    pub fn step_p<'a>(&mut self) {
        let mut workers = Vec::with_capacity(self.cpu_num);

        for i in 0..self.cpu_num {
            let head = i * self.cpu_rows;
            let y_range = head..(head + self.cpu_rows);
            let width = self.width as usize;
            let height = self.height as usize;
            let cap = self.cpu_rows * width;
            let world_a = self.world_a.clone();

            workers.push(spawn(move || {
                let mut lives = Vec::with_capacity(cap);
                let mut rows = 0;
                let world_a = &world_a.read().unwrap();

                for y in y_range {
                    if y >= height {
                        break;
                    }
                    rows += 1;
                    for x in 0..width {
                        lives.push(next_live(world_a, x as isize, y as isize));
                    }
                }
                (head as isize, rows, lives)
            }));
        }

        {
            let mut world_b = self.world_b.write().unwrap();
            for worker in workers {
                let (head, rows, lives) = worker.join().unwrap();
                if rows == 0 {
                    continue;
                }

                for y in 0..rows {
                    for x in 0..self.width {
                        world_b.set_live(x, head + y, lives[(y * self.width + x) as usize]);
                    }
                }
            }
        }

        self.swap();
    }

    pub fn step(&mut self) {
        {
            let world_a = self.world_a.read().unwrap();
            let mut world_b = self.world_b.write().unwrap();

            for y in 0..self.height {
                for x in 0..self.width {
                    world_b.set_live(x, y, next_live(&world_a, x, y));
                }
            }
        }

        self.swap();
    }

    pub fn lives(&self) -> Vec<bool> {
        let world_a = self.world_a.read().unwrap();
        world_a.cells.iter().map(|cell| cell.live).collect()
    }

    fn swap(&mut self) {
        swap(&mut self.world_a, &mut self.world_b)
    }
}

static POSES: [(isize, isize); 8] = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

pub fn next_live(world: &World, x: isize, y: isize) -> bool {
    let now = world.is_live(x, y);

    let lives = POSES.iter().fold(0, |a, (offset_x, offset_y)| {
        if world.is_live(x + offset_x, y + offset_y) {
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

#[test]
fn test_new() {
    let v = vec![true, true, false, false, true, false];
    let g = Game::new(4, 4, &v);
    let world_a = &g.world_a.read().unwrap();
    assert!(world_a.is_live(0, 0));
    assert!(world_a.is_live(1, 0));
    assert!(!world_a.is_live(2, 0));
    assert!(!world_a.is_live(3, 0));
    assert!(world_a.is_live(0, 1));
    assert!(!world_a.is_live(1, 1));
    assert!(!world_a.is_live(2, 1));
    assert!(!world_a.is_live(3, 1));
}

#[test]
fn test_step_blinker() {
    let blinker = vec![
        false, false, false,
        true, true, true,
        false, false, false,
    ];
    let mut g = Game::new(3, 3, &blinker);

    g.step();

    assert_eq!(g.lives(), vec![
        false, true, false,
        false, true, false,
        false, true, false,
    ]);

    g.step();

    assert_eq!(g.lives(), vec![
        false, false, false,
        true, true, true,
        false, false, false,
    ]);
}

#[test]
fn test_step_p() {
    let blinker = vec![
        false, false, false,
        true, true, true,
        false, false, false,
    ];
    let mut g = Game::new(3, 3, &blinker);

    g.step_p();

    assert_eq!(g.lives(), vec![
        false, true, false,
        false, true, false,
        false, true, false,
    ]);

    g.step_p();

    assert_eq!(g.lives(), vec![
        false, false, false,
        true, true, true,
        false, false, false,
    ]);
}

#[test]
fn test_swap() {
    let v = vec![true, true, false, false, true, false];
    let mut g = Game::new(4, 4, &v);
    {
        let world_a = g.world_a.read().unwrap();
        assert!(world_a.is_live(0, 0));
    }
    g.swap();
    {
        let world_a = g.world_a.read().unwrap();
        assert!(!world_a.is_live(0, 0));
    }
    g.swap();
    {
        let world_a = g.world_a.read().unwrap();
        assert!(world_a.is_live(0, 0));
    }
}

#[test]
fn test_next_live() {
    let v = vec![
        true, true, true, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false,
        false, false, false, true, true, true, false, false, false, false,
        false, false, false, true, true, true, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false,
        true, true, false, false, false, false, false, false, false, false,
        false, true, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false,
    ];

    let w = Game::new(10, 10, &v);

    let world_a = &w.world_a.read().unwrap();
    assert_eq!(next_live(world_a, 3, 3), true);
    assert_eq!(next_live(world_a, 2, 3), false);
    assert_eq!(next_live(world_a, 6, 4), false);
    assert_eq!(next_live(world_a, 6, 5), false);
    assert_eq!(next_live(world_a, 0, 7), true);
    assert_eq!(next_live(world_a, 0, 8), true);
    assert_eq!(next_live(world_a, 1, 7), true);
    assert_eq!(next_live(world_a, 1, 8), true);
}

extern crate num_cpus;

use crate::world::World;
use std::mem::swap;
use std::thread::spawn;
use std::sync::{Arc, RwLock};
use std::sync::mpsc::{Receiver, channel};

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
                world_a.set_life(x as isize, y as isize, *doa);
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

    pub fn step_farm(self, trigger_receiver: Receiver<()>) -> (Arc<RwLock<Game>>, Receiver<()>) {
        let Game { cpu_num, width, height, cpu_rows, .. } = self;

        let workers = cpu_num;
        let cap = cpu_rows * width as usize;

        let self_wrapper = Arc::new(RwLock::new(self));
        let (result_sender, result_receiver) = channel();
        let (turn_end_sender, turn_end_receiver) = channel();

        let mut world_senders = vec![];

        for i in 0..workers {
            let head = i * cpu_rows;
            let y_range = head..(head + cpu_rows);
            let sender = result_sender.clone();

            let (world_sender, world_receiver) = channel::<Arc<RwLock<World>>>();
            world_senders.push(world_sender);

            spawn(move || {
                while let Ok(world_a_arc) = world_receiver.recv() {
                    let world_a = world_a_arc.read().unwrap();
                    let mut lives = Vec::with_capacity(cap);
                    let mut rows = 0;
                    for y in y_range.clone() {
                        if y >= height as usize {
                            break;
                        }
                        rows += 1;
                        for x in 0..width {
                            lives.push(next_live(&world_a, x as isize, y as isize));
                        }
                    }
                    sender.send((head, rows, lives)).unwrap();
                }
            });
        }

        let game_writer = self_wrapper.clone();
        spawn(move || {
            let mut worked = 0;
            let limit = workers - 1;

            while let Ok((head, rows, lives)) = result_receiver.recv() {
                let mut game = game_writer.write().unwrap();
                if rows != 0 {
                    game.world_b.write().unwrap().set_lives(0, head, lives);
                }

                if worked >= limit {
                    worked = 0;
                    game.swap();
                    turn_end_sender.send(()).unwrap();
                } else {
                    worked += 1;
                }
            }
        });

        let game_reader = self_wrapper.clone();
        spawn(move || {
            while let Ok(()) = trigger_receiver.recv() {
                for world_sender in &world_senders {
                    world_sender.send(game_reader.read().unwrap().world_a.clone()).unwrap();
                }
            }
        });

        (self_wrapper, turn_end_receiver)
    }

    pub fn step(&mut self) {
        {
            let world_a = self.world_a.read().unwrap();
            let mut world_b = self.world_b.write().unwrap();

            for y in 0..self.height {
                for x in 0..self.width {
                    world_b.set_life(x, y, next_live(&world_a, x, y));
                }
            }
        }

        self.swap();
    }

    pub fn lives(&self) -> Vec<bool> {
        let world_a = self.world_a.read().unwrap();
        world_a.cells.clone()
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
fn test_step_farm() {
    let blinker = vec![
        false, false, false,
        true, true, true,
        false, false, false,
    ];
    let g = Game::new(3, 3, &blinker);
    let (trigger, receiver) = channel();

    let (game_wrapper, r) = g.step_farm(receiver);

    trigger.send(()).unwrap();
    r.recv().unwrap();
    assert_eq!(game_wrapper.read().unwrap().lives(), vec![
        false, true, false,
        false, true, false,
        false, true, false,
    ]);

    trigger.send(()).unwrap();
    r.recv().unwrap();
    assert_eq!(game_wrapper.read().unwrap().lives(), vec![
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

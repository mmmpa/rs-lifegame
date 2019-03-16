use crate::rle::Rle;
use crate::game::Game;
use std::str::FromStr;
use std::io::Write;
use std::{thread, time};
use gif::{Frame, Encoder, Repeat, SetParameter};
use std::fs::File;
use std::borrow::Cow;
use std::mem;
use std::sync::mpsc::channel;

mod world;
mod rle;
mod game;
mod standard_error;

/// Usage: lifegame gif      INPUT MARGIN DELAY TURNS OUTPUT
/// Usage: lifegame terminal INPUT MARGIN DELAY
fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.is_empty() {
        write_usage_and_exit()
    }

    let mode = (&args[0]).as_str();

    match mode {
        "gif" | "gif_p" | "gif_pp" if args.len() <= 6 => (),
        "term" if args.len() <= 4 => (),
        _ => write_usage_and_exit()
    }
    let is_gif = mode != "term";
    let margin = usize::from_str(&args[2]).expect("invalid MARGIN");
    let delay = u16::from_str(&args[3]).expect("invalid DELAY");

    let (w, h, map) = Rle::from_file(&args[1], margin).expect("parse INPUT error");
    let game = Game::new(w, h, &map);

    if is_gif {
        let turns = usize::from_str(&args[4]).expect("invalid TURNS");
        let output = &args[5];
        if mode == "gif_pp" {
            animation_gif_p(game, delay, turns, output);
        } else {
            animation_gif(game, delay, turns, output);
        }
    } else {
        terminal(game, delay);
    }
}

fn write_usage_and_exit() {
    writeln!(std::io::stderr(), "Usage: lifegame gif  INPUT MARGIN DELAY TURNS OUTPUT").unwrap();
    writeln!(std::io::stderr(), "       lifegame term INPUT MARGIN DELAY").unwrap();
    std::process::exit(1);
}

fn animation_gif_p(game: Game, delay: u16, turns: usize, output: &String) {
    let (width, height, mut encoder) = prepare(&game, output);
    let (trigger_sender, trigger_receiver) = channel();
    let (game_wrapper, result_receiver) = game.step_farm(trigger_receiver);

    for _ in 0..turns {
        let state = unsafe {
            mem::transmute::<Vec<bool>, Vec<u8>>(game_wrapper.read().unwrap().lives())
        };

        trigger_sender.send(()).unwrap();
        encoder(&frame(delay, width, height, Cow::Borrowed(&*state)));
        result_receiver.recv().unwrap();
    }
}

fn animation_gif(mut game: Game, delay: u16, turns: usize, output: &String) {
    let (width, height, mut encoder) = prepare(&game, output);

    for _ in 0..turns {
        let state = unsafe { mem::transmute::<Vec<bool>, Vec<u8>>(game.lives()) };

        encoder(&frame(delay, width, height, Cow::Borrowed(&*state)));
        game.step();
    }
}

fn frame(delay: u16, width: u16, height: u16, buffer: Cow<[u8]>) -> Frame {
    let mut frame = Frame::default();
    frame.delay = delay;
    frame.width = width;
    frame.height = height;
    frame.buffer = buffer;

    return frame
}

fn prepare(game: &Game, output: &String) -> (u16, u16, Box<FnMut(&Frame) -> ()>) {
    let color_map = &[0xFF, 0xFF, 0xFF, 0, 0, 0];
    let (width, height) = (game.width as u16, game.height as u16);

    let image = File::create(output).unwrap();
    let mut encoder = Encoder::new(image, width, height, color_map).unwrap();
    encoder.set(Repeat::Infinite).unwrap();

    let encoder = Box::new(move |frame: &Frame| {
        encoder.write_frame(&frame).unwrap()
    });

    (width, height, encoder)
}

fn terminal(mut game: Game, delay: u16) {
    let h = game.height as usize;
    let w = game.width as usize;
    let wait = time::Duration::from_millis(delay as u64);

    for i in 0.. {
        if i > 0 {
            writeln!(std::io::stdout(), "\x1B[{}F", h + 1).unwrap();
        }
        game.lives().chunks(w).for_each(|row| {
            let row_string = row.iter().fold(String::with_capacity(w * 2), |a, doa| {
                if *doa { a + "■ " } else { a + "□ " }
            });
            writeln!(std::io::stdout(), "{}", row_string).unwrap();
        }
        );
        game.step();

        thread::sleep(wait);
    }
}

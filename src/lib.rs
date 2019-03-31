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

pub mod world;
pub mod rle;
pub mod game;
pub mod standard_error;

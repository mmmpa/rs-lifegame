extern crate regex;

use crate::standard_error::StandardError;

use std::fs;
use std::error::Error;
use self::regex::Regex;
use std::str::FromStr;

/// http://www.conwaylife.com/wiki/Run_Length_Encoded

pub struct Rle {}

impl Rle {
    pub fn from_file(filename: &str, margin: usize) -> Result<(usize, usize, Vec<bool>), Box<Error>> {
        let raw = fs::read_to_string(filename)?;
        Self::from_string(&raw, margin)
    }

    pub fn from_string(body: &str, margin: usize) -> Result<(usize, usize, Vec<bool>), Box<Error>> {
        let (setting, data) = split(body.to_string())?;
        let (w, h) = parse_setting(setting)?;
        let map = parse_map(w, h, margin, data);

        Ok((w + margin * 2, h + margin * 2, map))
    }
}

#[test]
fn test_parse() {
    assert!(Rle::from_file("fixtures/sample.rl", 0).is_err());
    Rle::from_file("fixtures/valid.rle", 0).unwrap();
}

fn split(raw: String) -> Result<(String, String), Box<Error>> {
    let mut setting: String = "".to_string();
    let mut data: String = "".to_string();

    for line in raw.lines() {
        if line.starts_with("#") {
            continue;
        } else if line.starts_with("x = ") {
            setting = line.to_string();
        } else {
            data += &line.to_string();
        }
    }

    if setting.is_empty() || data.is_empty() {
        return Err(Box::new(StandardError::new("lack")))
    }

    Ok((setting, data))
}

#[test]
fn test_split() {
    let raw = fs::read_to_string("fixtures/valid.rle").unwrap();
    let (setting, data) = split(raw).unwrap();
    assert_eq!(setting, "x = 1, y = 2, rule = B3/S23".to_string());
    assert_eq!(data, "13b22o!".to_string());

    let raw = fs::read_to_string("fixtures/no_data.rle").unwrap();
    assert!(split(raw).is_err());

    let raw = fs::read_to_string("fixtures/no_setting.rle").unwrap();
    assert!(split(raw).is_err());
}

fn parse_setting(line: String) -> Result<(usize, usize), Box<Error>> {
    let setting = Regex::new(r"x = ([0-9]+), y = ([0-9]+)")?;

    let cap = match setting.captures(line.as_str()) {
        Some(c) => c,
        None => return Err(Box::new(StandardError::new(&format!("invalid: {:?}", line))))
    };

    match (usize::from_str(&cap[1]), usize::from_str(&cap[2])) {
        (Ok(l), Ok(r)) => Ok((l, r)),
        _ => Err(Box::new(StandardError::new("invalid")))
    }
}

#[test]
fn test_parse_setting() {
    assert!(parse_setting("x = 1, y = x".to_string()).is_err());
    assert_eq!(parse_setting("x = 1, y = 23".to_string()).unwrap(), (1, 23));
}

fn parse_map(raw_w: usize, raw_h: usize, margin: usize, life_map: String) -> Vec<bool> {
    let w = raw_w + margin * 2;
    let h = raw_h + margin * 2;
    let mut num = "".to_string();
    let total = w * h;
    let mut lives = Vec::with_capacity(total);

    // # format
    //
    // Cells states are formatted as <run_count><tag>.
    // When <run_count> is 1, it can be omitted.
    //
    // # tags
    //
    // b: dead cell
    // o: alive cell
    // $: end of line
    // !: end of cell
    //
    // # expansion
    //
    // "12b" means "12 dead cells."
    // "2ob" means "2 alive cells and a dead cell."
    //
    // Dead cells between last alive cell in a line and "$" can be omitted.
    //

    lives.extend(vec![false; w * margin + margin]);

    let mut rows = 0;
    for c in life_map.chars() {
        match c {
            c if c.is_digit(10) => num.push(c),
            '!' => break,
            'b' | 'o' => {
                let n = usize::from_str(&num).unwrap_or(1);

                lives.extend(vec![is_alive(c); n]);

                num.clear();
                rows += n;
            },
            '$' => {
                let n = usize::from_str(&num).unwrap_or(1);
                if rows < w {
                    lives.extend(vec![false; w - rows]);
                }
                if n > 1 {
                    lives.extend(vec![false; w * (n - 1)]);
                }
                num.clear();
                rows = 0;
            },
            _ => ()
        }
    }

    if lives.len() < total {
        lives.extend(vec![false; total - lives.len()]);
    }

    lives
}

fn is_alive(c: char) -> bool {
    c == 'o'
}

#[test]
fn test_parse_map() {
    assert_eq!(parse_map(4, 4, 0, "2o$bobo$3bo!".to_string()), vec![
        true, true, false, false,
        false, true, false, true,
        false, false, false, true,
        false, false, false, false,
    ]);

    assert_eq!(parse_map(4, 4, 2, "2o$bobo$3bo!".to_string()), vec![
        false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false,
        false, false, true, true, false, false, false, false,
        false, false, false, true, false, true, false, false,
        false, false, false, false, false, true, false, false,
        false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false,
    ]);
}

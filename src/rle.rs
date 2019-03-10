extern crate regex;

use crate::standard_error::StandardError;

use std::fs;
use std::error::Error;
use self::regex::Regex;
use std::str::FromStr;

struct Rle {}

impl Rle {
    fn parse(filename: &str) -> Result<(), Box<Error>> {
        let raw = fs::read_to_string(filename)?;

        let mut data = "".to_string();
        let mut wh: (usize, usize);

        for line in raw.lines() {
            if line.starts_with("#") {
                continue;
            } else if line.starts_with("x") {
                wh = Rle::parse_setting(line)?;
            } else {
                data += &line.to_string();
            }
        }

        println!("####### text content {:?}", data);

        Ok(())
    }

    fn parse_setting(line: &str) -> Result<(usize, usize), Box<Error>> {
        let setting = Regex::new(r"x = ([0-9]+), y = ([0-9]+)")?;

        let cap = match setting.captures(line) {
            Some(c) => c,
            None => return Err(Box::new(StandardError::new("invalid")))
        };

        match (usize::from_str(&cap[1]), usize::from_str(&cap[2])) {
            (Ok(l), Ok(r)) => Ok((l, r)),
            _ => Err(Box::new(StandardError::new("invalid")))
        }
    }

    fn parse_map(life_map: String) {
        let mut num = "".to_string();
        let mut rows = 0;

        for c in life_map.chars() {
            match c {
                c if c.is_digit(10) => {
                    println!("{}", c);
                    num.push(c);
                },
                'b' => {
                    let n = usize::from_str(&num).unwrap_or(1);
                    println!("dead cell: {}", n);
                    num.clear();
                    rows += n;
                },
                'o' => {
                    let n = usize::from_str(&num).unwrap_or(1);
                    println!("alive cell: {}", n);
                    num.clear();
                    rows += n;
                },
                '$' => {
                    println!("end of line: {}", rows);
                    num.clear();
                    rows = 0;
                },
                _ => {}
            }
        }
    }
}

#[test]
fn test_parse() {
    assert!(Rle::parse("fixtures/sample.rl").is_err());
    Rle::parse("fixtures/sample.rle").unwrap();
}

#[test]
fn test_parse_setting() {
    assert!(Rle::parse_setting("x = 1, y = x").is_err());
    assert_eq!(Rle::parse_setting("x = 1, y = 23").unwrap(), (1, 23));
}

#[test]
fn test_parse_map() {
    Rle::parse_map("13b2o7bo57b2o4bo$5bo7b2o5b3o6b2o49bobo2bobo$5b3o11bo9bo51bo4bo$8bo10b2o6bobo$7b2o18b2o$16b3o50b3o7b2o$15bo3bo50bob2o4bo2b2o$14bo5bo47bobob2o4b2ob2o$2b2ob2o7bo5bo47bo3bo$o2bob2o7bo5bo14b2o30b3o2bo4bob2o$2obo11bo3bo10b2o2bo2bo32b2o$3bo12b3o11bobo2b2o30b2o10b2o$3b2o28b2o30b2o10b2o$b2o2bobo26bo39b2o$o2bo2b2o26bob2o27b2obo4bo2b3o$b2o28b2obo2bo35bo3bo$31b2ob2o27b2ob2o4b2obobo$63b2o2bo4b2obo$65b2o7b3o2$29b2o$17b2o10bo29bo4bo$18bo11b3o25bobo2bobo$15b3o5b2o7bo26bo4b2o$5bobo7bo7b2o$5b2o$6bo!".to_string());
}

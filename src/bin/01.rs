use anyhow::*;
use const_format::concatcp;
use std::io::{BufRead, BufReader};

const DAY: &str = "01";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

fn input_lines() -> Result<impl Iterator<Item = Result<String, std::io::Error>>> {
    Ok(BufReader::new(std::fs::File::open(INPUT_FILE)?).lines())
}

const STARTING_POSITION: isize = 50;

struct Dial {
    position: isize,
    positions: Vec<isize>,
    zeros: isize,
}

impl Dial {
    fn new() -> Self {
        Dial {
            position: STARTING_POSITION,
            positions: Vec::new(),
            zeros: 0,
        }
    }

    fn move_dial(&mut self, amount: isize) {
        // any number MORE than 99 starts again from 0
        // any number LESS than 0 starts again from 99
        let new_position = (self.position + amount).rem_euclid(100);
        if new_position == 0 {
            self.zeros += 1;
        }
        self.position = new_position;
        self.positions.push(new_position)
    }

    fn amount_from_line(line: &String) -> Result<isize> {
        let is_negative = line.starts_with('L');
        let num_str = line.trim_start_matches(['R', 'L']);
        // it doesn't matter how big the number is as we only care about
        // how many times the dial LANDS on 0, not how many times
        // zero is passed
        let num_str = if num_str.len() > 2 {
            &num_str[num_str.len() - 2..]
        } else {
            num_str
        };
        let val = num_str
            .parse::<isize>()
            .context("Failed to parse number from string")?;
        let val = if is_negative { -val } else { val };
        Ok(val)
    }

    fn position(&self) -> isize {
        self.position
    }
}

fn main() -> Result<()> {
    let mut dial = Dial::new();

    for line in input_lines()? {
        let amount = Dial::amount_from_line(&line?);
        dial.move_dial(amount?);
        println!("{}", dial.position());
    }

    println!("Total Zeros: {}", dial.zeros);

    // i'm so used to working with usize that I chose that by default and hit
    // a "attempt to add with overflow" panic... obviously usize can't be negative
    Ok(())
}

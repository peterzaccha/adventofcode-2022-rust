use std::{num::ParseIntError, str::FromStr};

static INPUT: &'static str = include_str!("day04.input.txt");

#[derive(Debug)]
struct Range {
    from: u32,
    to: u32,
}

impl Range {
    fn contains(&self, other: &Self) -> bool {
        if self.from <= other.from && self.to >= other.to {
            true
        } else {
            false
        }
    }
    fn inside(&self, other: &Self) -> bool {
        if self.from >= other.from && self.to <= other.to {
            true
        } else {
            false
        }
    }
}

impl FromStr for Range {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split("-");
        Ok(Self {
            from: tokens.next().unwrap().parse()?,
            to: tokens.next().unwrap().parse()?,
        })
    }
}

impl FromStr for Pair {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split(",");
        Ok(Self {
            range1: tokens.next().unwrap().parse().unwrap(),
            range2: tokens.next().unwrap().parse().unwrap(),
        })
    }
}

#[derive(Debug)]
struct Pair {
    range1: Range,
    range2: Range,
}

impl Pair {
    fn inside_each(&self) -> bool {
        self.range1.contains(&self.range2) || self.range2.contains(&self.range1)
    }
    fn overlap(&self) -> bool {
        if (self.range1.to >= self.range2.from && self.range1.to <= self.range2.to)
            || (self.range2.to >= self.range1.from && self.range2.to <= self.range1.to)
        {
            true
        } else {
            false
        }
    }
}

fn parse_input() -> Vec<Pair> {
    INPUT
        .lines()
        .map(|line| line.parse::<Pair>().unwrap())
        .collect()
}

#[test]
fn part1() {
    let pairs = parse_input();
    let count = pairs.iter().filter(|&a| a.inside_each()).count();
    println!("{:?}", count)
}

#[test]
fn part2() {
    let pairs = parse_input();
    let count = pairs.iter().filter(|&a| a.overlap()).count();
    println!("{:?}", count)
}

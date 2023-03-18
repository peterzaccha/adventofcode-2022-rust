use std::collections::{HashMap, HashSet};

#[allow(dead_code)]

static INPUT: &'static str = include_str!("day03.input.txt");

#[derive(Debug, Clone)]
struct Rucksack {
    items: &'static str,
    compartment1: &'static str,
    compartment2: &'static str,
}

impl Rucksack {
    fn get_wrong_item(&self) -> Option<char> {
        for x in self.compartment1.chars() {
            for y in self.compartment2.chars() {
                if x == y {
                    return Some(x);
                }
            }
        }
        println!(
            "{:?}:{},{:?}:{}",
            self.compartment1,
            self.compartment1.len(),
            self.compartment2,
            self.compartment2.len()
        );
        None
    }
}

fn parse_input() -> Vec<Rucksack> {
    return INPUT
        .lines()
        .map(|line| {
            let len = line.len();
            let mid = len / 2;
            let comp1 = &line[..mid];
            let comp2 = &line[mid..];
            Rucksack {
                items: &line,
                compartment1: &comp1,
                compartment2: &comp2,
            }
        })
        .collect();
}

#[test]
fn part1() {
    let scores: Vec<(char, u32)> = parse_input()
        .iter()
        .map(|ruck| match ruck.get_wrong_item() {
            Some(c) => match c.is_lowercase() {
                true => (c, (c as u32) - 96),
                false => (c, (c as u32) - 64 + 26),
            },
            None => ('*', 0),
        })
        .collect();

    println!("{:?}", scores.iter().map(|a| a.1).sum::<u32>())
}

#[derive(Debug)]
struct Group {
    group: (Rucksack, Rucksack, Rucksack),
}

impl Group {
    fn find_item(&self) -> char {
        let mut freq: HashMap<char, HashSet<usize>> = HashMap::from([]);
        for (i, r) in [&self.group.0, &self.group.1, &self.group.2]
            .iter()
            .enumerate()
        {
            for c in r.items.chars() {
                freq.entry(c)
                    .and_modify(|f| {
                        f.insert(i);
                    })
                    .or_insert(HashSet::from([i]));
            }
        }
        println!("{:?}", freq);
        let a = freq
            .iter()
            .max_by(|a, b| a.1.len().cmp(&b.1.len()))
            .unwrap();

        if a.1.len() != 3 {
            panic!("a")
        }
        a.0.clone()
    }
}

#[test]
fn part2() {
    let groups: Vec<Group> = parse_input()
        .chunks(3)
        .map(|c| Group {
            group: (c[0].clone(), c[1].clone(), c[2].clone()),
        })
        .collect();

    let chars: Vec<char> = groups.iter().map(|g| g.find_item()).collect();
    let scores: u32 = chars
        .iter()
        .map(|&c| match c.is_lowercase() {
            true => (c as u32) - 96,
            false => (c as u32) - 64 + 26,
        })
        .sum::<u32>();

    println!("{:?}", scores)
}

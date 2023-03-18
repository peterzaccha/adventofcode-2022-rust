use std::collections::HashMap;

static INPUT: &'static str = include_str!("./day02.input.txt");

#[derive(PartialEq, Eq, Hash, Clone)]
enum State {
    Rock,
    Paper,
    Scissors,
}

fn get_lose() -> HashMap<State, State> {
    let beats: HashMap<State, State> = HashMap::from([
        (State::Scissors, State::Rock),
        (State::Rock, State::Paper),
        (State::Paper, State::Scissors),
    ]);
    beats
}
fn get_beats() -> HashMap<State, State> {
    HashMap::from([
        (State::Rock, State::Scissors),
        (State::Paper, State::Rock),
        (State::Scissors, State::Paper),
    ])
}

impl State {
    fn beats(&self, other: &Self) -> bool {
        let beats = get_beats();
        return match beats.get(self) {
            Some(state) => state == other,
            None => panic!(),
        };
    }
    fn score(&self) -> u32 {
        match self {
            State::Rock => 1,
            State::Paper => 2,
            State::Scissors => 3,
        }
    }
}

enum Outcome {
    Win,
    Lose,
    Tie,
}

struct Game {
    me: State,
    other: State,
}

impl Game {
    fn outcome(&self) -> Outcome {
        if self.me == self.other {
            return Outcome::Tie;
        } else if self.me.beats(&self.other) {
            return Outcome::Win;
        }
        Outcome::Lose
    }
    fn score(&self) -> u32 {
        let outcome_score = match self.outcome() {
            Outcome::Lose => 0,
            Outcome::Tie => 3,
            Outcome::Win => 6,
        };
        self.me.score() + outcome_score
    }
}

#[test]
fn part1() {
    let score: u32 = INPUT
        .lines()
        .map(|line| {
            let game_str: Vec<&str> = line.split(" ").collect();
            let other = match game_str[0] {
                "A" => State::Rock,
                "B" => State::Paper,
                "C" => State::Scissors,
                _ => panic!(),
            };
            let me = match game_str[1] {
                "X" => State::Rock,
                "Y" => State::Paper,
                "Z" => State::Scissors,
                _ => panic!(),
            };
            return Game { me, other };
        })
        .map(|a| a.score())
        .sum();
    println!("{}", score)
}

#[test]
fn part2() {
    let score: u32 = INPUT
        .lines()
        .map(|line| {
            let beats = get_beats();
            let loses = get_lose();
            let game_str: Vec<&str> = line.split(" ").collect();
            let other = match game_str[0] {
                "A" => State::Rock,
                "B" => State::Paper,
                "C" => State::Scissors,
                _ => panic!(),
            };
            let outcome = match game_str[1] {
                "X" => Outcome::Lose,
                "Y" => Outcome::Tie,
                "Z" => Outcome::Win,
                _ => panic!(),
            };
            let me = match outcome {
                Outcome::Win => loses.get(&other).unwrap().clone(),
                Outcome::Lose => beats.get(&other).unwrap().clone(),
                Outcome::Tie => other.clone(),
            };
            return Game { me, other };
        })
        .map(|a| a.score())
        .sum();
    println!("{}", score)
}

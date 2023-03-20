use std::collections::VecDeque;

static INPUT: &'static str = include_str!("day06.input.txt");

#[derive(Debug)]
struct Buffer {
    buffer: VecDeque<char>,
    size: usize,
}

impl Buffer {
    fn is_diffrent_chars(&self) -> bool {
        let mut exists: Vec<char> = vec![];
        println!("{exists:?}, {self:?}");
        if self.buffer.len() < self.size {
            return false;
        }
        for &c in self.buffer.iter() {
            if exists.contains(&c) {
                println!("{exists:?}, {self:?}");
                return false;
            } else {
                exists.push(c);
            }
        }
        true
    }
    fn push(&mut self, c: char) -> () {
        self.buffer.push_front(c);
        if self.buffer.len() > self.size {
            self.buffer.pop_back();
        }
    }

    fn new(size: usize) -> Self {
        Self {
            buffer: VecDeque::new(),
            size,
        }
    }
}

#[test]
fn part1() {
    let mut buffer: Buffer = Buffer::new(4);

    for (i, char) in INPUT.chars().enumerate() {
        buffer.push(char);
        if buffer.is_diffrent_chars() {
            println!("{}", i + 1);
            return;
        }
    }
}

#[test]
fn part2() {
    let mut buffer: Buffer = Buffer::new(14);

    for (i, char) in INPUT.chars().enumerate() {
        buffer.push(char);
        if buffer.is_diffrent_chars() {
            println!("{}", i + 1);
            return;
        }
    }
}

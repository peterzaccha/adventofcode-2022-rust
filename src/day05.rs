static INPUT: &'static str = include_str!("day05.input.txt");

#[derive(Debug, Clone)]
struct Cargo {
    stacks: Vec<Vec<char>>,
}

#[derive(Debug, Clone)]
struct Move {
    count: usize,
    from: usize,
    to: usize,
}

impl Cargo {
    fn execute(&mut self, mv: &Move) {
        for _ in 0..mv.count {
            let a = self.stacks[mv.from - 1].pop();
            match a {
                Some(c) => self.stacks[mv.to - 1].push(c),
                None => println!("Empty "),
            }
        }
    }
    fn new_execute(&mut self, mv: &Move) {
        let len_from = self.stacks[mv.from - 1].len();
        let mut moving: Vec<char> = vec![];
        moving.extend(
            self.stacks[mv.from - 1][len_from - mv.count..]
                .iter()
                .cloned(),
        );
        self.stacks[mv.from - 1] = self.stacks[mv.from - 1][..len_from - mv.count]
            .iter()
            .cloned()
            .collect();
        self.stacks[mv.to - 1].extend(moving.iter())
        // match a {
        //     Some(c) => self.stacks[mv.to - 1].push(c),
        //     None => println!("Empty "),
        // }
    }
}

fn parse_input() -> (Cargo, Vec<Move>) {
    let mut tokens = INPUT.split("\n\n");
    let raw_cargo = tokens.next().unwrap();
    let raw_operations = tokens.next().unwrap();
    let mut cargo = Cargo {
        stacks: vec![vec![]; 9],
    };

    for line in raw_cargo
        .lines()
        .collect::<Vec<&str>>()
        .iter()
        .take(8)
        .rev()
    {
        for (i, col) in line.chars().collect::<Vec<char>>().chunks(4).enumerate() {
            match col[1] {
                token if token.is_alphabetic() => cargo.stacks[i].push(token),
                _ => {}
            }
        }
    }

    let operations: Vec<Move> = raw_operations
        .lines()
        .map(|line| {
            let mut tokens = line.split(" ");
            tokens.next(); //move
            let count: usize = tokens.next().unwrap().parse().unwrap();
            tokens.next(); //from
            let from: usize = tokens.next().unwrap().parse().unwrap();
            tokens.next(); //to
            let to: usize = tokens.next().unwrap().parse().unwrap();
            Move { count, from, to }
        })
        .collect();

    // println!("{:?}", cargo);
    // cargo.stacks[0].pop();
    // println!("{:?}", cargo);

    (cargo, operations)
}

#[test]
fn part1() {
    let (mut cargo, moves) = parse_input();

    for mv in &moves {
        cargo.execute(mv);
    }
    println!(
        "{:?}, {:?}",
        cargo
            .stacks
            .iter()
            .map(|stack| stack.last().unwrap().clone())
            .collect::<String>(),
        cargo
    )
}

#[test]
fn part2() {
    let (mut cargo, moves) = parse_input();

    for mv in &moves {
        cargo.new_execute(mv);
    }

    println!(
        "{:?}, {:?}",
        cargo
            .stacks
            .iter()
            .map(|stack| stack.last().unwrap().clone())
            .collect::<String>(),
        cargo
    )
}

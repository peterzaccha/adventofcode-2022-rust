use std::{
    borrow::{Borrow, BorrowMut},
    cell::RefCell,
    cmp::Ordering,
    fmt::Debug,
    ops::{Deref, DerefMut},
    rc::Rc,
};

static INPUT: &'static str = include_str!("day07.input.txt");

struct Node {
    name: String,
    size: u64,
    children: Vec<Rc<RefCell<Node>>>,
    is_dir: bool,
    parent: Option<Rc<RefCell<Node>>>,
}

struct DirTree {
    root: Rc<RefCell<Node>>,
    current_node: Rc<RefCell<Node>>,
}

impl Debug for DirTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DirTree").field("root", &self.root).finish()
    }
}
impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Node")
            .field("name", &self.name)
            .field("size", &self.size)
            .field("children", &self.children)
            .field("is_dir", &self.is_dir)
            .finish()
    }
}

impl Node {
    fn size(&mut self) -> u64 {
        if self.size > 0 {
            return self.size;
        }
        let mut total: u64 = 0;
        let mut childs: Vec<Rc<RefCell<Node>>> = vec![];
        childs.extend(self.children.iter().map(|a| Rc::clone(a)));
        for child in &self.children {
            let current = Rc::clone(child);
            let size = current.as_ref().borrow_mut().size();
            total += size;
        }
        self.size = total;
        total
    }
}

impl DirTree {
    fn new() -> Self {
        let root = Rc::new(RefCell::new(Node {
            name: "/".to_string(),
            size: 0,
            children: vec![],
            is_dir: true,
            parent: None,
        }));

        Self {
            root: Rc::clone(&root),
            current_node: Rc::clone(&root),
        }
    }
    fn apply(&mut self, command: Command) {
        match command {
            Command::CD(target) => match target.as_str() {
                "/" => self.current_node = Rc::clone(&self.root),
                ".." => {
                    let parent = self.current_node.as_ref().borrow().parent.clone().unwrap();
                    self.current_node = Rc::clone(&parent);
                }
                x => {
                    self.current_node = self.find_dir(x.to_string()).unwrap();
                }
            },
            Command::LS(childs) => {
                for line in childs.lines() {
                    let mut tokens = line.split(" ");
                    let first_word = tokens.next().unwrap();
                    let name = tokens.next().unwrap();

                    match first_word {
                        "dir" => {
                            self.current_node
                                .as_ref()
                                .borrow_mut()
                                .children
                                .push(Rc::new(RefCell::new(Node {
                                    name: name.to_string(),
                                    size: 0,
                                    children: vec![],
                                    is_dir: true,
                                    parent: Some(Rc::clone(&self.current_node)),
                                })));
                        }
                        size => {
                            let file_size: u64 = size.parse().unwrap();
                            self.current_node
                                .as_ref()
                                .borrow_mut()
                                .children
                                .push(Rc::new(RefCell::new(Node {
                                    name: name.to_string(),
                                    size: file_size,
                                    children: vec![],
                                    is_dir: false,
                                    parent: Some(Rc::clone(&self.current_node)),
                                })));
                        }
                    }
                }
            }
        }
    }

    fn find_dir(&self, name: String) -> Option<Rc<RefCell<Node>>> {
        for node in &self.current_node.as_ref().borrow().children {
            if node.as_ref().borrow().name == name {
                return Some(Rc::clone(&node));
            }
        }
        None
    }
}

#[derive(Debug)]
enum Command {
    CD(String),
    LS(String),
}

fn parse_input() -> Vec<Command> {
    let mut commands: Vec<Command> = vec![];
    let mut buffer: Vec<&str> = vec![];
    let mut current_command: Option<Command> = None;
    for line in INPUT.lines() {
        let mut tokens = line.split(" ");
        let first_token = tokens.next().unwrap();
        if let Ordering::Equal = first_token.cmp("$") {
            // if command
            match current_command {
                Some(mut command) => {
                    if let Command::LS(ref mut result) = command {
                        *result = buffer.join("\n");
                    }
                    commands.push(command);
                    buffer = vec![];
                }
                None => {}
            }
            let command_str = tokens.next().unwrap();

            match command_str {
                "ls" => {
                    current_command = Some(Command::LS("".to_string()));
                }
                "cd" => {
                    let targer_dir = tokens.next().unwrap();
                    current_command = Some(Command::CD(targer_dir.to_string()))
                }
                x => panic!("Command is not supported {}", x),
            }
        } else {
            buffer.push(line)
        }
    }
    match current_command {
        Some(mut command) => {
            if let Command::LS(ref mut result) = command {
                *result = buffer.join("\n");
            }
            commands.push(command);
            buffer = vec![];
        }
        None => {}
    }

    commands
}

#[test]
fn part1() {
    let commands = parse_input();
    let mut tree = DirTree::new();

    // println!("{commands:?}");
    for command in commands {
        tree.apply(command)
    }

    let mut sum: u64 = 0;

    let mut child: Vec<Rc<RefCell<Node>>> = vec![Rc::clone(&tree.root)];

    loop {
        let current = child.pop();
        if let None = current {
            break;
        }
        let current = Rc::clone(&current.unwrap());
        if !current.as_ref().borrow().is_dir {
            continue;
        }
        let size = current.as_ref().borrow_mut().size();
        if size < 100000 {
            sum += size;
        }
        let new = &(current.as_ref().borrow().children);
        child.extend(new.iter().map(|a| Rc::clone(a)));
    }

    println!("{sum:?},")
}

fn part2() {}

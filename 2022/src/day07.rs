use std::fmt::{Display, Formatter, Result};

#[derive(Eq, PartialEq)]
enum Command {
    CdBack,
    CdRoot,
    CdDir(String),
    Ls
}

#[derive(Eq, PartialEq)]
enum FsEntry {
    Directory(String),
    File(String, usize)
}

impl Display for FsEntry {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
        match self {
            Self::Directory(name) => write!(fmt, "{} (dir)", name),
            Self::File(name, size) => write!(fmt, "{} (file, size={})", name, size),
        }
    }
}

#[derive(Debug)]
struct Tree<T> where T: PartialEq {
    arena: Vec<Node<T>>,
}

impl<T> Tree<T> where T: PartialEq {
    fn new() -> Tree<T> {
        Tree { arena: vec![] }
    }

    fn insert(&mut self, value: T, parent: Option<usize>) -> usize {
        let index = self.arena.len();
        self.arena.push(Node::new(self.arena.len(), value, parent));

        if let Some(p) = parent {
            self.arena[p].children.push(index);
        }

        index
    }

    fn find(&mut self, value: T, parent: Option<usize>) -> Option<usize> {
        for node in &self.arena {
            if node.value == value && (parent.is_some() && node.parent.is_some() && parent.unwrap() == node.parent.unwrap()) {
                return Some(node.index);
            }
        }

        None
    }

    fn depth(&self, idx: usize) -> usize {
        match self.arena[idx].parent {
            Some(id) => 1 + self.depth(id),
            None => 0,
        }
    }
}

impl<T> Tree<T> where T: PartialEq + Display {
    fn rec_print(&self, index: usize, fmt: &mut Formatter<'_>) -> Result {
        if self.arena[index].children.is_empty() {
            Ok(())
        } else {
            self.arena[index].children
                .iter()
                .fold(Ok(()), |result, index| {
                    result
                        .and(writeln!(fmt, "{}- {}", "  ".repeat(self.depth(*index)), self.arena[*index].value))
                        .and(self.rec_print(*index, fmt))
                })
        }
    }
}

impl<T> Display for Tree<T> where T: PartialEq + Display {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
        self.rec_print(0, fmt)
    }
}

#[derive(Debug)]
struct Node<T> where T: PartialEq {
    index: usize,
    value: T,
    parent: Option<usize>,
    children: Vec<usize>,
}

impl<T> Node<T> where T: PartialEq {
    fn new(index: usize, value: T, parent: Option<usize>) -> Self {
        Self {
            index,
            value,
            parent,
            children: vec![],
        }
    }
}

fn main() {
    let input = include_str!("../input/day07.input");

    // Part one
    let size_sum = first_puzzle(input);
    println!("Directories of size <= 100000 add up to {}.", size_sum);

    // Part two
    let dir_size = second_puzzle(input);
    println!("Directory of size {} should be deleted.", dir_size);
}

#[test]
fn sample() {
    let sample = include_str!("../sample/day07.input");

    let tree = build_tree(sample);
    println!("{}", tree);

    assert_eq!(first_puzzle(sample), 95437);
    assert_eq!(second_puzzle(sample), 24933642);
}

fn first_puzzle(source: &str) -> usize {
    let tree = build_tree(source);

    tree.arena
        .iter()
        .enumerate()
        .filter(|(_, node)| matches!(node.value, FsEntry::Directory(_)))
        .map(|(index, _)| calc_size(&tree, index))
        .filter(|dir_size| *dir_size <= 100000)
        .sum()
}

fn second_puzzle(source: &str) -> usize {
    let total_size_available = 70000000;
    let unused_size_target = 30000000;

    let tree = build_tree(source);
    let used_size = calc_size(&tree, 0);

    let mut candidates: Vec<usize> = tree.arena
        .iter()
        .enumerate()
        .filter(|(_, node)| matches!(node.value, FsEntry::Directory(_)))
        .map(|(index, _)| calc_size(&tree, index))
        .filter(|dir_size| (total_size_available - used_size) + dir_size >= unused_size_target)
        .collect();

    candidates.sort();
    *candidates.first().unwrap()
}

fn build_tree(source: &str) -> Tree<FsEntry> {
    let mut tree = Tree::new();
    let mut index = 0;
    let mut ls_called = false;

    let root = tree.insert(FsEntry::Directory("/".to_string()), None);

    source
        .lines()
        .for_each(|line| {
            if line.starts_with("$ ") {
                let command = parse_command(line);

                ls_called = false;
                match command {
                    Command::CdBack => index = tree.arena[index].parent.expect("Directory doesn't have a parent."),
                    Command::CdRoot => index = root,
                    Command::Ls => ls_called = true,
                    Command::CdDir(name) => {
                        let dir_index = tree
                            .find(FsEntry::Directory(name), Some(index))
                            .expect("Directory does not contain this directory");
            
                        index = dir_index;
                    }
                }
            } else {
                if !ls_called {
                    panic!("Tried to parse 'ls' results before an 'ls' call.");
                }

                let result = parse_result(line);
                tree.insert(result, Some(index));
            }
        });

    tree
}

fn calc_size(tree: &Tree<FsEntry>, index: usize) -> usize {
    let node = &tree.arena[index];

    match node.value {
        FsEntry::File(_, size) => size,
        FsEntry::Directory(_) => {
            node
                .children
                .iter()
                .map(|index| calc_size(tree, *index))
                .sum()
        }
    }
}

fn parse_command(source: &str) -> Command {
    if source.starts_with("$ ") {
        let statement = &mut source.strip_prefix("$ ").unwrap().split(' ');
        let command = statement.next().expect("Could not find a valid command (must be 'cd' or 'ls').");

        match command {
            "ls" => return Command::Ls,
            "cd" => {
                let argument = statement.next().expect("Could not find a valid 'cd' argument.");
                
                match argument {
                    "/" => return Command::CdRoot,
                    ".." => return Command::CdBack,
                    _ => return Command::CdDir(argument.to_string())
                }
            }
            _ => panic!("Could not find a valid command (must be 'cd' or 'ls').")
        }
    }

    panic!("Not a valid command statement.");
}

fn parse_result(source: &str) -> FsEntry {
    if source.starts_with("dir ") {
        return FsEntry::Directory(source.strip_prefix("dir ").expect("Invalid format for directory result.").to_string());
    }

    let mut size_name = source.split(' ');

    let size = size_name.next().expect("Could not find file size.").parse::<usize>().expect("Invalid file size.");
    let name = size_name.next().expect("Could not find file name.").to_string();

    FsEntry::File(name, size)
}
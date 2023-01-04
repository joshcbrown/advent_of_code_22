use crate::Solution;
use regex::Regex;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::rc::Rc;

#[derive(PartialEq, Eq, Debug, Hash)]
struct File {
    filename: String,
    size: u32,
}

impl File {
    fn set_from_content(content: &str) -> HashSet<Self> {
        let mut result = HashSet::new();
        let re = Regex::new(r"(?P<size>\d+) (?P<filename>.+)").unwrap();
        re.captures_iter(content).for_each(|cap| {
            result.insert(Self {
                filename: cap["filename"].parse().unwrap(),
                size: cap["size"].parse().unwrap(),
            });
        });
        result
    }
}

#[derive(Debug)]
struct Directory {
    name: String,
    files: HashSet<File>,
    children: HashMap<String, Rc<RefCell<Directory>>>,
    parent: Option<Rc<RefCell<Directory>>>,
}

impl Directory {
    fn direct_size(&self) -> u32 {
        self.files.iter().map(|file| file.size).sum()
    }

    fn size(&self) -> u32 {
        let direct_size: u32 = self.direct_size();
        let indirect_size: u32 = self
            .children
            .iter()
            .map(|(_, child)| child.borrow().size())
            .sum();
        direct_size + indirect_size
    }

    // adds child directory to parent directory, returns reference to child
    fn add_child_to_parent(parent: Rc<RefCell<Self>>, name: String) -> Rc<RefCell<Self>> {
        let child = Self {
            name: name.clone(),
            files: HashSet::new(),
            children: HashMap::new(),
            parent: Some(Rc::clone(&parent)),
        };
        let child = Rc::new(RefCell::new(child));
        parent.borrow_mut().children.insert(name, Rc::clone(&child));
        child
    }

    fn add_files(&mut self, content: &str) {
        self.files.extend(File::set_from_content(content))
    }

    // insanely inefficient
    fn whole_size_less_than(&self, n: u32) -> u32 {
        let mut size = match self.size_less_than(n) {
            Some(s) => s,
            None => 0,
        };
        for (_, child) in &self.children {
            size += child.borrow().whole_size_less_than(n)
        }
        size
    }

    fn size_less_than(&self, n: u32) -> Option<u32> {
        let direct_size: u32 = self.files.iter().map(|file| file.size).sum();
        let mut size = if direct_size < n {
            Some(direct_size)
        } else {
            None
        }?;
        for (_, child) in &self.children {
            let child_size = child.borrow().size_less_than(n);
            match child_size {
                Some(m) => size += m,
                None => return None,
            }
        }
        if size < n {
            Some(size)
        } else {
            None
        }
    }
}

pub(crate) struct Day7 {
    root: Rc<RefCell<Directory>>,
}

impl Day7 {
    fn handle_ls(command: &str, current: &mut Rc<RefCell<Directory>>) {
        let (_, output) = command.split_once('\n').unwrap();
        current.borrow_mut().add_files(output);
    }

    fn handle_cd(
        command: &str,
        current: Rc<RefCell<Directory>>,
        root: &Rc<RefCell<Directory>>,
    ) -> Rc<RefCell<Directory>> {
        let mut lines = command.lines();
        let dir_name = &lines.next().unwrap()[3..];
        match dir_name {
            ".." => Rc::clone(current.borrow().parent.as_ref().unwrap()),
            "/" => Rc::clone(root),
            _ => {
                if !current.borrow().children.contains_key(dir_name) {
                    Directory::add_child_to_parent(current, dir_name.to_string())
                } else {
                    Rc::clone(&current.borrow().children[dir_name])
                }
            }
        }
    }
}

impl Solution for Day7 {
    fn new(content: String) -> Self {
        let commands: Vec<_> = content.split("$ ").skip(1).map(str::trim).collect();
        let root = Directory {
            name: "/".to_string(),
            files: HashSet::new(),
            children: HashMap::new(),
            parent: None,
        };
        let root = Rc::new(RefCell::new(root));
        let mut current = Rc::clone(&root);
        for command in commands {
            match &command[0..2] {
                "ls" => {
                    Day7::handle_ls(command, &mut current);
                }
                "cd" => current = Day7::handle_cd(command, current, &root),
                _ => {
                    panic!("invalid command: {command}")
                }
            }
        }
        Self { root }
    }
    fn solve1(&self) -> String {
        self.root.borrow().whole_size_less_than(100_000).to_string()
    }
}

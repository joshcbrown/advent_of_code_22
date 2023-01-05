use crate::Solution;
use regex::Regex;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
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
    fn cum_size_less_than(&self, n: u32) -> u32 {
        let mut size = self.size_less_than(n).unwrap_or(0);
        for (_, child) in &self.children {
            size += child.borrow().cum_size_less_than(n)
        }
        size
    }

    // good candidate for monadic stuff
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

    // not super general, wanted to make this accept a closure and work like a filter
    // but ran into recursion problems
    fn dirs_bigger_than(source_dir: Rc<RefCell<Self>>, n: u32) -> Vec<Rc<RefCell<Self>>> {
        let mut self_vec = if source_dir.borrow().size() > n {
            vec![Rc::clone(&source_dir)]
        } else {
            vec![]
        };
        for (_, child) in &source_dir.borrow().children {
            self_vec.append(&mut Directory::dirs_bigger_than(Rc::clone(&child), n));
        }
        self_vec
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

const LARGEST_SIZE: u32 = 100_000;
const REQUIRED_SPACE: u32 = 30_000_000;
const TOTAL_SPACE: u32 = 70_000_000;

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
        self.root
            .borrow()
            .cum_size_less_than(LARGEST_SIZE)
            .to_string()
    }

    fn solve2(&self) -> String {
        let used_space = self.root.borrow().size();
        println!("{}", used_space);
        let required_to_delete = REQUIRED_SPACE - (TOTAL_SPACE - used_space);

        let potential_deletions =
            Directory::dirs_bigger_than(Rc::clone(&self.root), required_to_delete);

        let mut sizes: Vec<u32> = potential_deletions
            .iter()
            .map(|dir| dir.borrow().size())
            .collect();
        sizes.sort();

        sizes[0].to_string()
    }
}

use std::collections::HashSet;

use crate::Solution;

type Item = u8;

trait Valuable {
    fn value(self) -> i32;
}

impl Valuable for Item {
    fn value(self) -> i32 {
        match self {
            97..=122 => (self - 96).into(),
            65..=90 => (self - 38).into(),
            _ => panic!("invalid character supplied: {}", self as char),
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Rucksack {
    compart1: Vec<Item>,
    compart2: Vec<Item>,
}

impl Rucksack {
    fn new(line: &str) -> Self {
        let (a, b) = line.split_at(line.len() / 2);
        Self {
            compart1: a.as_bytes().into(),
            compart2: b.as_bytes().into(),
        }
    }
    fn common(&self) -> u8 {
        let compart1_items: HashSet<u8> = HashSet::from_iter(self.compart1.clone());
        for c in &self.compart2 {
            if compart1_items.contains(c) {
                return *c;
            }
        }
        panic!("no matching characters");
    }
}

pub(crate) struct Day3 {
    rucksacks: Vec<Rucksack>,
}

impl Solution for Day3 {
    fn new(content: String) -> Self {
        let rucksacks = content.lines().map(|line| Rucksack::new(line)).collect();
        Self { rucksacks }
    }

    fn solve1(&self) -> String {
        self.rucksacks
            .iter()
            .map(|sack| sack.common().value())
            .sum::<i32>()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rucksack_new_splits_correctly() {
        let line = "vJrwpWtwJgWrhcsFMMfFFhFp";
        let expected = Rucksack {
            compart1: "vJrwpWtwJgWr".as_bytes().into(),
            compart2: "hcsFMMfFFhFp".as_bytes().into(),
        };
        assert_eq!(expected, Rucksack::new(line));
    }

    #[test]
    fn solve1_solves() {
        let content = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"
            .to_string();
        assert_eq!("157".to_string(), Day3::new(content).solve1())
    }
}

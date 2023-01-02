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
    items: Vec<Item>,
}

impl Rucksack {
    fn new(line: &str) -> Self {
        Self {
            items: line.as_bytes().into(),
        }
    }

    fn comparts(&self) -> (Vec<Item>, Vec<Item>) {
        let (a, b) = self.items.split_at(self.items.len() / 2);
        (a.into(), b.into())
    }

    fn common(&self) -> u8 {
        let (compart1, compart2) = self.comparts();
        let compart1_items: HashSet<u8> = HashSet::from_iter(compart1);
        for c in &compart2 {
            if compart1_items.contains(c) {
                return *c;
            }
        }
        panic!("no matching characters");
    }

    fn intersects(&self, other: &Self) -> Self {
        let a: HashSet<Item> = HashSet::from_iter(self.items.clone());
        let b: HashSet<Item> = HashSet::from_iter(other.items.clone());
        let items: Vec<Item> = a.into_iter().filter(|item| b.contains(item)).collect();
        Self { items }
    }
}

pub(crate) struct Day3 {
    rucksacks: Vec<Rucksack>,
}

impl Solution for Day3 {
    fn new(content: String) -> Self {
        let rucksacks = content.lines().map(Rucksack::new).collect();
        Self { rucksacks }
    }

    fn solve1(&self) -> String {
        self.rucksacks
            .iter()
            .map(|sack| sack.common().value())
            .sum::<i32>()
            .to_string()
    }

    fn solve2(&self) -> String {
        self.rucksacks
            .chunks(3)
            .map(|chunk| {
                if let [a, b, c] = chunk {
                    a.intersects(b).intersects(c)
                } else {
                    panic!("number of elves not a multiple of 3")
                }
            })
            .map(|intersection| {
                let potential_items = &intersection.items[..];
                if let [i] = potential_items {
                    (*i as Item).value()
                } else {
                    panic!("no common element among three elves")
                }
            })
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
        let expected = (
            "vJrwpWtwJgWr".as_bytes().into(),
            "hcsFMMfFFhFp".as_bytes().into(),
        );
        assert_eq!(expected, Rucksack::new(line).comparts());
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

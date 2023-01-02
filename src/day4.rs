use crate::Solution;

struct Assignment {
    start: i32,
    end: i32,
}

impl Assignment {
    fn new(s: &str) -> Self {
        let (start, end) = s.split_once('-').unwrap();
        let (start, end): (i32, i32) = (start.parse().unwrap(), end.parse().unwrap());
        Self { start, end }
    }

    fn overlaps_entirely(&self, other: &Self) -> bool {
        (self.start >= other.start && self.end <= other.end)
            || (self.start <= other.start && self.end >= other.end)
    }

    fn overlaps_at_all(&self, other: &Self) -> bool {
        self.start <= other.end && self.end >= other.start
    }
}

struct ElfPair {
    elf1: Assignment,
    elf2: Assignment,
}

impl ElfPair {
    fn new(line: &str) -> Self {
        let (elf1_str, elf2_str) = line.split_once(',').unwrap();
        let (elf1, elf2) = (Assignment::new(elf1_str), Assignment::new(elf2_str));
        Self { elf1, elf2 }
    }
}

pub(crate) struct Day4 {
    pairings: Vec<ElfPair>,
}

impl Solution for Day4 {
    fn new(content: String) -> Self {
        Self {
            pairings: content.lines().map(ElfPair::new).collect(),
        }
    }

    fn solve1(&self) -> String {
        self.solve(|pair| pair.elf1.overlaps_entirely(&pair.elf2))
    }

    fn solve2(&self) -> String {
        self.solve(|pair| pair.elf1.overlaps_at_all(&pair.elf2))
    }
}

impl Day4 {
    fn solve<F>(&self, f: F) -> String
    where
        F: Fn(&ElfPair) -> bool,
    {
        self.pairings
            .iter()
            .filter(|pair| f(pair))
            .count()
            .to_string()
    }
}

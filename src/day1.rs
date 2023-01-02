use crate::Solution;

pub(crate) struct Day1 {
    elf_cals: Vec<i32>,
}

impl Solution for Day1 {
    fn new(content: String) -> Self {
        Day1 {
            elf_cals: content
                .split("\n\n")
                .map(|elf| elf.lines().map(|int| int.parse::<i32>().unwrap()).sum())
                .collect(),
        }
    }

    fn solve1(&self) -> String {
        let result = self.elf_cals.iter().max();
        match result {
            Some(int) => int.to_string(),
            None => panic!("empty vector"),
        }
    }

    fn solve2(&self) -> String {
        let mut elf_cals = self.elf_cals.clone();
        elf_cals.sort_by(|a, b| b.cmp(a));
        // TODO: learn and implement partition algorithm for this
        elf_cals[..3].iter().sum::<i32>().to_string()
    }
}

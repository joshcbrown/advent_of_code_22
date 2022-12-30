use crate::Solution;

pub(crate) struct Day1 {
    pub content: String,
}

impl Day1 {
    fn elf_cals(&self) -> Vec<i32> {
        self.content
            .split("\n\n")
            .map(|elf| elf.lines().map(|int| int.parse::<i32>().unwrap()).sum())
            .collect()
    }
}

impl Solution for Day1 {
    fn solve1(&self) -> String {
        let elf_cals = self.elf_cals();
        let result = elf_cals.iter().max();
        match result {
            Some(int) => int.to_string(),
            None => panic!("empty vector"),
        }
    }

    fn solve2(&self) -> String {
        let mut elf_cals = self.elf_cals();
        elf_cals.sort_by(|a, b| b.cmp(a));
        // format!("{:#?}", elf_cals)
        elf_cals[..3].into_iter().sum::<i32>().to_string()
    }
}

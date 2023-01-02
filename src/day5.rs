use crate::Solution;
use regex::Regex;

type Crate = char;

#[derive(Debug)]
struct Instruction {
    from: usize,
    to: usize,
    qty: usize,
}

impl Instruction {
    fn from_language(line: &str) -> Self {
        // this is wildly inefficient, but wanted to learn a bit of regex
        // in rust so chose to do it this way
        let re = Regex::new(r"move (?P<qty>\d+) from (?P<from>\d+) to (?P<to>\d+)").unwrap();
        let cap = re.captures(line).unwrap();
        Self {
            from: cap["from"].parse().unwrap(),
            to: cap["to"].parse().unwrap(),
            qty: cap["qty"].parse().unwrap(),
        }
    }
}

pub(crate) struct Day5 {
    stacks: Vec<Vec<Crate>>,
    instructions: Vec<Instruction>,
}

impl Solution for Day5 {
    fn new(content: String) -> Self {
        let num_stacks = (content.lines().next().unwrap().len() + 1) / 4;
        let mut stacks: Vec<Vec<Crate>> = Vec::with_capacity(num_stacks);
        (0..num_stacks).for_each(|_| stacks.push(Vec::new()));

        let (stack_info, instructions) = content.split_once("\n\n").unwrap();

        let mut lines = stack_info.lines();
        // consume last line (the number allocated to each stack)
        lines.next_back();

        for line in lines {
            for (i, new_crate) in line.chars().into_iter().skip(1).step_by(4).enumerate() {
                if new_crate != ' ' {
                    stacks[i].push(new_crate);
                }
            }
        }

        // reverse each array so that pop() is an O(1) operation
        for stack in &mut stacks {
            stack.reverse();
        }

        let instructions: Vec<_> = instructions
            .lines()
            .map(|line| Instruction::from_language(line))
            .collect();

        Self {
            stacks,
            instructions,
        }
    }

    fn solve1(&self) -> String {
        self.solve(|&Instruction { from, to, qty }, stacks| {
            (0..qty).for_each(|_| {
                let ele = stacks[from - 1].pop().unwrap();
                stacks[to - 1].push(ele);
            })
        })
    }

    fn solve2(&self) -> String {
        self.solve(|&Instruction { from, to, qty }, stacks| {
            let from_stack = &mut stacks[from - 1];
            let from_len = from_stack.iter().len();
            let mut moving: Vec<_> = from_stack.drain((from_len - qty)..).collect();
            stacks[to - 1].append(&mut moving);
        })
    }
}

impl Day5 {
    fn solve<F>(&self, f: F) -> String
    where
        F: Fn(&Instruction, &mut Vec<Vec<Crate>>),
    {
        let mut stacks = self.stacks.clone();
        self.instructions
            .iter()
            .for_each(|instruction| f(instruction, &mut stacks));
        stacks
            .into_iter()
            .map(|mut stack| stack.pop().unwrap())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve1_solves() {
        let content = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"
            .to_string();
        let expected = "CMZ".to_string();
        assert_eq!(expected, Day5::new(content).solve1());
    }
    #[test]
    fn solve2_solves() {
        let content = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"
            .to_string();
        let expected = "MCD".to_string();
        assert_eq!(expected, Day5::new(content).solve2());
    }
}

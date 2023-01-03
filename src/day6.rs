use std::collections::HashSet;

use crate::Solution;

// possible optimisation to skip redundant windows, cbs implementing for right
// now
// struct WindowQueue {
//     start_idx: usize,
//     window: HashMap<char, usize>,
//     idx_to_char: HashMap<usize, char>,
//     max_elements: usize,
// }

pub(crate) struct Day6 {
    stream: String,
}

impl Solution for Day6 {
    fn new(content: String) -> Self {
        Self { stream: content }
    }

    fn solve1(&self) -> String {
        self.solve(4)
    }

    fn solve2(&self) -> String {
        self.solve(14)
    }
}

impl Day6 {
    fn solve(&self, needed_distinct: usize) -> String {
        for (i, w) in self.stream.as_bytes().windows(needed_distinct).enumerate() {
            let window_els: HashSet<u8> = HashSet::from_iter(w.iter().cloned());
            if window_els.len() == needed_distinct {
                return (i + needed_distinct).to_string();
            }
        }
        "didn't find appropriate window".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solve1_solves() {
        let input1 = "bvwbjplbgvbhsrlpgdmjqwftvncz".to_string();
        let input2 = "nppdvjthqldpwncqszvftbrmjlhg".to_string();
        let input3 = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string();
        let input4 = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string();
        let (e1, e2, e3, e4) = (5.to_string(), 6.to_string(), 10.to_string(), 11.to_string());

        assert_eq!(e1, Day6::new(input1).solve1());
        assert_eq!(e2, Day6::new(input2).solve1());
        assert_eq!(e3, Day6::new(input3).solve1());
        assert_eq!(e4, Day6::new(input4).solve1());
    }
}

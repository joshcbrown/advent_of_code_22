use std::iter::repeat;

use crate::Solution;

pub(crate) struct Day8 {
    content: String,
}

impl Solution for Day8 {
    fn new(content: String) -> Self {
        Self { content }
    }

    fn solve1(&self) -> String {
        let mut visible: Vec<Vec<bool>> = Vec::new();
        let lines: Vec<Vec<u8>> = self
            .content
            .lines()
            .map(|line| line.as_bytes().to_vec())
            .collect();
        println!("{:#?}", lines);
        let n_rows = lines.len();
        let n_cols = lines[0].len();
        for (i, line) in lines.iter().enumerate() {
            visible.push(repeat(false).take(line.len()).collect());
            do_the_thing(&mut visible[i], line.iter(), false);
            do_the_thing(&mut visible[i], line.iter().rev(), true);
        }

        for j in 0..n_cols {
            let mut max_so_far = b'0' - 1;

            for i in 0..n_rows {
                if lines[i][j] > max_so_far {
                    max_so_far = lines[i][j];
                    visible[i][j] = true;
                }
            }
            max_so_far = b'0' - 1;
            for i in (0..n_rows).rev() {
                println!("{i}: {}", lines[i][0]);
                if lines[i][j] > max_so_far {
                    max_so_far = lines[i][j];
                    visible[i][j] = true;
                }
            }
        }
        println!("{:#?}", visible);
        visible
            .concat()
            .iter()
            .filter(|val| **val)
            .count()
            .to_string()
    }
}

fn do_the_thing<'a, I>(vec: &mut Vec<bool>, line: I, reverse: bool)
where
    I: ExactSizeIterator<Item = &'a u8>,
{
    let mut max_so_far: u8 = b'0' - 1;
    let max_index = line.len() - 1;
    for (j, &num) in line.enumerate() {
        if num > max_so_far {
            max_so_far = num;
            if reverse {
                vec[max_index - j] = true;
            } else {
                vec[j] = true;
            }
            continue;
        }
    }
}

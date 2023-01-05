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
        let n_rows = lines.len();
        let n_cols = lines[0].len();

        for (i, line) in lines.iter().enumerate() {
            visible.push(repeat(false).take(line.len()).collect());
            fill_horizontal(&mut visible[i], line.iter(), false);
            fill_horizontal(&mut visible[i], line.iter().rev(), true);
        }

        for j in 0..n_cols {
            fill_vertical(&mut visible, &lines, j, n_rows, false);
            fill_vertical(&mut visible, &lines, j, n_rows, true);
        }

        visible
            .concat()
            .iter()
            .filter(|val| **val)
            .count()
            .to_string()
    }
}

fn fill_vertical(
    vec: &mut Vec<Vec<bool>>,
    lines: &Vec<Vec<u8>>,
    j: usize,
    n_rows: usize,
    reverse: bool,
) {
    let mut max_so_far = b'0' - 1;

    for i in 0..n_rows {
        let (comp, to_change) = if reverse {
            (lines[i][j], &mut vec[i][j])
        } else {
            (lines[n_rows - i][j], &mut vec[i][j])
        };
        if comp > max_so_far {
            max_so_far = comp;
            *to_change = true;
        }
    }
}

fn fill_horizontal<'a, I>(vec: &mut Vec<bool>, line: I, reverse: bool)
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

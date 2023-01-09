use std::iter::repeat;

use crate::Solution;

pub(crate) struct Day8 {
    lines: Vec<Vec<u8>>,
}

impl Solution for Day8 {
    fn new(content: String) -> Self {
        Self {
            lines: content
                .lines()
                .map(|line| line.as_bytes().to_vec())
                .collect(),
        }
    }

    fn solve1(&self) -> String {
        let mut visible: Vec<Vec<bool>> = Vec::new();
        let lines = &self.lines;
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

    fn solve2(&self) -> String {
        let mut scores: Vec<Vec<u32>> = Vec::new();
        let lines = &self.lines;
        let n_rows = lines.len();
        let n_cols = lines[0].len();

        for (i, line) in lines.iter().enumerate() {
            scores.push(repeat(1).take(n_cols).collect());
            scores[i][0] = 0;
            let mut line_iter = line.iter();
            let mut prev_height: u8 = *line_iter.next().unwrap();
            let mut trees_since_decrease = 0;
            for (height, j) in line_iter.zip(1..) {
                if *height <= prev_height {
                    trees_since_decrease = 0;
                }
                trees_since_decrease += 1;
                println!(
                    "from left: {i}, {j}, {} compared to {}, {trees_since_decrease}",
                    *height as char, prev_height as char
                );
                scores[i][j] *= trees_since_decrease;
                prev_height = *height;
            }

            let mut rev_line_iter = line.iter().rev();
            (prev_height, trees_since_decrease) = (*rev_line_iter.next().unwrap(), 0);
            scores[i][n_cols - 1] = 0;
            for (height, j) in rev_line_iter.zip(1..) {
                if *height <= prev_height {
                    trees_since_decrease = 0;
                }
                trees_since_decrease += 1;
                println!(
                    "from right: {i}, {j}, {} compared to {}, {trees_since_decrease}",
                    *height as char, prev_height as char
                );
                scores[i][n_cols - 1 - j] *= trees_since_decrease;
                prev_height = *height;
            }
        }
        println!("{:#?}", scores);

        for j in 0..n_cols {
            let mut prev_height: u8 = lines[0][j];
            scores[0][j] = 0;
            let mut trees_since_decrease = 0;
            for i in 1..n_rows {
                let height = lines[i][j];
                if height <= prev_height {
                    trees_since_decrease = 0;
                }
                trees_since_decrease += 1;
                println!(
                    "from up: {i}, {j}, {} compared to {}, {trees_since_decrease}",
                    height as char, prev_height as char
                );
                scores[i][j] *= trees_since_decrease;
                prev_height = height;
            }

            (prev_height, trees_since_decrease) = (lines[n_rows - 1][j], 0);
            scores[n_rows - 1][j] = 0;
            for i in 1..n_rows {
                let height = lines[n_rows - 1 - i][j];
                if height <= prev_height {
                    trees_since_decrease = 0;
                }
                println!(
                    "from down: {i}, {j}, {} compared to {}, {trees_since_decrease}",
                    height as char, prev_height as char
                );
                trees_since_decrease += 1;
                scores[n_rows - 1 - i][j] *= trees_since_decrease;
                prev_height = height;
            }
        }

        println!("{:#?}", scores);
        todo!();
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
            (lines[n_rows - 1 - i][j], &mut vec[n_rows - 1 - i][j])
        } else {
            (lines[i][j], &mut vec[i][j])
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

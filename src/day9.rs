use std::{
    collections::HashSet,
    ops::{AddAssign, Sub},
};

use crate::Solution;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn l1_distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Point {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

struct Rope {
    head: Point,
    tail: Point,
    visited: HashSet<Point>,
}

impl Rope {
    fn new() -> Self {
        let origin = Point { x: 0, y: 0 };
        Self {
            head: origin,
            tail: origin,
            visited: HashSet::from_iter([origin]),
        }
    }

    fn move_head(&mut self, to_add: Point) {
        self.head += to_add;
        let rope_vec = self.head - self.tail;
        let rope_length = rope_vec.l1_distance();
        if (self.head.x != self.tail.x && self.head.y != self.tail.y && rope_length > 2)
            || (self.head.x != self.tail.x && self.head.y == self.tail.y && rope_length > 1)
            || (self.head.y != self.tail.y && self.head.x == self.tail.x && rope_length > 1)
        {
            self.tail += rope_vec - to_add;
        }
        self.visited.insert(self.tail);
    }
}

struct Instruction {
    direction: char,
    num_steps: i32,
}

impl Instruction {
    fn from_line(line: &str) -> Self {
        let (direction, num_str) = line.split_once(' ').unwrap();
        let direction = direction.chars().nth(0).unwrap();
        let num_steps = num_str.parse().unwrap();
        Self {
            direction,
            num_steps,
        }
    }

    fn to_point(&self) -> Point {
        match self.direction {
            'U' => Point { x: 0, y: 1 },
            'D' => Point { x: 0, y: -1 },
            'L' => Point { x: -1, y: 0 },
            'R' => Point { x: 1, y: 0 },
            _ => panic!("unexpected dir"),
        }
    }
}

pub(crate) struct Day9 {
    instructions: Vec<Instruction>,
}

impl Solution for Day9 {
    fn new(content: String) -> Self {
        Self {
            instructions: content.lines().map(Instruction::from_line).collect(),
        }
    }

    fn solve1(&self) -> String {
        let mut rope = Rope::new();
        self.instructions.iter().for_each(|i| {
            for _ in 1..=i.num_steps {
                rope.move_head(i.to_point())
            }
        });
        rope.visited.len().to_string()
    }
}

use std::{
    cmp,
    collections::{HashSet, VecDeque},
};

use advent::AdventRunnable;
use itertools::Itertools;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn value(&self) -> (i32, i32) {
        match *self {
            Direction::Down => (1, 0),
            Direction::Up => (-1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }

    fn angle(&self, back: bool) -> Direction {
        if back {
            match *self {
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Up,
                Direction::Right => Direction::Down,
                Direction::Up => Direction::Left,
            }
        } else {
            match *self {
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Down,
                Direction::Right => Direction::Up,
                Direction::Up => Direction::Right,
            }
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
    direction: Direction,
}

impl Position {
    fn go(&self) -> (i32, i32) {
        let (ix, iy) = self.direction.value();

        (self.x + ix, self.y + iy)
    }
}

pub struct Day16Runnable;

fn calc_enegized_tiles(initial: Position, file_contents: &String) -> i32 {
    let lines = file_contents.lines().map(|x| x.as_bytes()).collect_vec();
    let mut q: VecDeque<Position> = VecDeque::new();
    let mut seen: HashSet<Position> = HashSet::new();
    let mut seen_pos: HashSet<(i32, i32)> = HashSet::new();

    q.push_back(initial);

    while let Some(curr) = q.pop_front() {
        let ch_pos = curr.go();

        if ch_pos.0 < 0
            || ch_pos.0 >= lines.len() as i32
            || ch_pos.1 < 0
            || ch_pos.1 >= lines[0].len() as i32
        {
            continue;
        }

        seen_pos.insert((ch_pos.0, ch_pos.1));

        let ch = lines[ch_pos.0 as usize][ch_pos.1 as usize];

        let new_pos: Option<Vec<Position>> = match (ch, &curr.direction) {
            (b'.', _)
            | (b'-', Direction::Left | Direction::Right)
            | (b'|', Direction::Up | Direction::Down) => Some(vec![Position {
                x: ch_pos.0,
                y: ch_pos.1,
                direction: curr.direction,
            }]),
            (b'\\', _) => Some(vec![Position {
                x: ch_pos.0,
                y: ch_pos.1,
                direction: curr.direction.angle(true),
            }]),
            (b'/', _) => Some(vec![Position {
                x: ch_pos.0,
                y: ch_pos.1,
                direction: curr.direction.angle(false),
            }]),
            (b'-', _) => Some(vec![
                Position {
                    x: ch_pos.0,
                    y: ch_pos.1,
                    direction: Direction::Left,
                },
                Position {
                    x: ch_pos.0,
                    y: ch_pos.1,
                    direction: Direction::Right,
                },
            ]),
            (b'|', _) => Some(vec![
                Position {
                    x: ch_pos.0,
                    y: ch_pos.1,
                    direction: Direction::Up,
                },
                Position {
                    x: ch_pos.0,
                    y: ch_pos.1,
                    direction: Direction::Down,
                },
            ]),
            _ => None,
        };

        match new_pos {
            Some(new_positions) => {
                for p in new_positions {
                    if !seen.contains(&p) {
                        q.push_back(p);
                        seen.insert(p);
                    }
                }
            }
            None => continue,
        }
    }

    return seen_pos.len() as i32;
}

impl AdventRunnable for Day16Runnable {
    fn run_part1(&self, file_contents: String) -> String {
        calc_enegized_tiles(
            Position {
                x: 0,
                y: -1,
                direction: Direction::Right,
            },
            &file_contents,
        )
        .to_string()
    }

    fn run_part2(&self, file_contents: String) -> String {
        let lines = file_contents.lines().collect_vec();
        let mut max = 0;

        for i in 0..lines.len() {
            max = cmp::max(
                max,
                calc_enegized_tiles(
                    Position {
                        x: i as i32,
                        y: -1,
                        direction: Direction::Right,
                    },
                    &file_contents,
                ),
            );
            max = cmp::max(
                max,
                calc_enegized_tiles(
                    Position {
                        x: i as i32,
                        y: lines[0].len() as i32,
                        direction: Direction::Left,
                    },
                    &file_contents,
                ),
            )
        }

        for i in 0..lines[0].len() {
            max = cmp::max(
                max,
                calc_enegized_tiles(
                    Position {
                        x: -1,
                        y: i as i32,
                        direction: Direction::Down,
                    },
                    &file_contents,
                ),
            );
            max = cmp::max(
                max,
                calc_enegized_tiles(
                    Position {
                        x: lines.len() as i32,
                        y: i as i32,
                        direction: Direction::Up,
                    },
                    &file_contents,
                ),
            )
        }

        max.to_string()
    }
}

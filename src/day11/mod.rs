use std::collections::HashSet;

use advent::AdventRunnable;

// #[macro_use]
use itertools::{self, iproduct};

pub struct Day11Runnable;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Coordinate(pub i32, pub i32);

#[derive(Hash, Debug)]
struct CoordPair {
    x: Coordinate,
    y: Coordinate
}

impl PartialEq for CoordPair {
    fn eq(&self, other: &Self) -> bool {
        (self.x == other.x && self.y == other.y) || (self.x == other.y && self.y == other.x)
    }
}

impl Eq for CoordPair {}


impl Day11Runnable {
    fn empty_rows(universe: &mut Vec<Vec<char>>) -> Vec<usize> {
        let mut rows_empty:Vec<usize> = Vec::new();

        for i in 0..universe.len() {
            let mut empty = true;
            for j in 0..universe[i].len() {
                if universe[i][j] != '.' {
                    empty = false;
                    break
                }
            }

            if empty {
                rows_empty.push(i);
            }
        }

        return rows_empty
    }

    fn empty_cols(universe: &mut Vec<Vec<char>>) -> Vec<usize> {
        let mut columns_empty:Vec<usize> = Vec::new();

        for j in 0..universe[0].len() {
            let mut empty = true;
            for i in 0..universe.len() {
                if universe[i][j] != '.' {
                    empty=false;
                    break;
                }
            }

            if empty {
                columns_empty.push(j)
            }
        }

        return columns_empty;
    }

    fn expand_universe(universe: &mut Vec<Vec<char>>) {
        let mut rows_empty:Vec<usize> = Self::empty_rows(universe);
        let mut columns_empty:Vec<usize> = Self::empty_cols(universe);


        rows_empty.reverse();

        for i in rows_empty {
            universe.insert(i, vec!['.'; universe[0].len()]);

        }

        columns_empty.reverse();
        for j in columns_empty {
            for u in universe.iter_mut() {
                u.insert(j, '.');
            }
        }

    }
}

impl AdventRunnable for Day11Runnable {

    fn run_part1(&self, file_contents: String) -> String {
        let mut galaxy_coords:Vec<Coordinate> = Vec::new();
        let mut grid:Vec<Vec<char>> = Vec::new();

        for l in file_contents.lines() {
            grid.push(l.chars().collect());
        }

        // println!("Size of the Universe before expanding {:?}x{:?}", grid.len(), grid[0].len());
        Self::expand_universe(&mut grid);
        // println!("Size of the Universe before expanding {:?}x{:?}", grid.len(), grid[0].len());

        for (r, l) in grid.iter().enumerate() {
            for (c, _) in l.iter().enumerate() {
                if grid[r][c] == '#' {
                    galaxy_coords.push(Coordinate(r as i32, c as i32));
                }
            }
        }

        // println!("Location of galaxies {:?} (#{:?})", galaxy_coords, galaxy_coords.len());
        // println!("Product of galaxies coords {:?}", iproduct!(&galaxy_coords, &galaxy_coords));

        let mut set_of_coords:HashSet<CoordPair> = HashSet::new();
        
        for (x, y) in iproduct!(&galaxy_coords, &galaxy_coords) {
            if x != y {
                // println!("{:?} - {:?}", x, y);
                let pair = CoordPair{x: x.clone(), y: y.clone()};
                let reverse_pair = CoordPair{y: x.clone(), x: y.clone()};


                if set_of_coords.contains(&pair)  || set_of_coords.contains(&reverse_pair) {
                    continue;
                }

                set_of_coords.insert(pair);
            }
        }

        // println!("Set of galaxies coords {:?} (#{:?})", set_of_coords, set_of_coords.len());

        let mut sum : i32 = 0;
        for x in set_of_coords {
            let x_diff = (x.x.0 - x.y.0).abs();
            let y_diff = (x.x.1 - x.y.1).abs();

            sum += x_diff+y_diff;
        }


        sum.to_string()
    }

    fn run_part2(&self, file_contents: String) -> String {
        let mut galaxy_coords:Vec<Coordinate> = Vec::new();
        let mut grid:Vec<Vec<char>> = Vec::new();

        for l in file_contents.lines() {
            grid.push(l.chars().collect());
        }

        let rows_empty = Self::empty_rows(&mut grid);
        let cols_empty = Self::empty_cols(&mut grid);

        // println!("Size of the Universe before expanding {:?}x{:?}", grid.len(), grid[0].len());
        // Self::expand_universe(&mut grid);
        // println!("Size of the Universe before expanding {:?}x{:?}", grid.len(), grid[0].len());

        for (r, l) in grid.iter().enumerate() {
            for (c, _) in l.iter().enumerate() {
                if grid[r][c] == '#' {
                    galaxy_coords.push(Coordinate(r as i32, c as i32));
                }
            }
        }

        // println!("Location of galaxies {:?} (#{:?})", galaxy_coords, galaxy_coords.len());
        // println!("Product of galaxies coords {:?}", iproduct!(&galaxy_coords, &galaxy_coords));

        let mut set_of_coords:HashSet<CoordPair> = HashSet::new();
        
        for (x, y) in iproduct!(&galaxy_coords, &galaxy_coords) {
            if x != y {
                // println!("{:?} - {:?}", x, y);
                let pair = CoordPair{x: x.clone(), y: y.clone()};
                let reverse_pair = CoordPair{y: x.clone(), x: y.clone()};


                if set_of_coords.contains(&pair)  || set_of_coords.contains(&reverse_pair) {
                    continue;
                }

                set_of_coords.insert(pair);
            }
        }

        // println!("Set of galaxies coords {:?} (#{:?})", set_of_coords, set_of_coords.len());

        let mut sum : i64 = 0;
        for x in set_of_coords {
            //Rows
            let (min_row, max_row) = match x.x.0 < x.y.0 {
                true => (x.x.0, x.y.0),
                false => (x.y.0, x.x.0)
            };

            let mut row_diff = max_row as i64 - min_row as i64;

            for empty in &rows_empty {
                if min_row < *empty as i32 && max_row > *empty as i32 {
                    row_diff += 999999;
                }

            }

            //Rows
            let (min_col, max_col) = match x.x.1 < x.y.1 {
                true => (x.x.1, x.y.1),
                false => (x.y.1, x.x.1)
            };

            let mut col_diff = max_col as i64 - min_col as i64;

            for empty in &cols_empty {
                if min_col < *empty as i32 && max_col > *empty as i32 {
                    col_diff += 999999;
                }

            }

            sum += row_diff + col_diff;

        }


        sum.to_string()
    }
}
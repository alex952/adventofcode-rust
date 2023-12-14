use std::collections::VecDeque;

use advent::AdventRunnable;
use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord, Copy)]
pub struct Coordinate(pub usize, pub usize);



pub struct Day10Runnable;

impl Day10Runnable {
    fn find_loop(&self, maze: &Vec<Vec<char>>) -> Vec<Coordinate> {
        let mut start: Coordinate = Coordinate(0, 0);
        let mut our_loop: Vec<Coordinate> = Vec::new();

        'out_loop: for(r, l) in maze.iter().enumerate() {
            for (c, ch) in l.iter().enumerate() {
                if *ch == 'S' {
                    start = Coordinate(r, c);
                    break 'out_loop;
                }
            }
        }
        println!("Our start is {:?}", start);

        let mut q:VecDeque<Coordinate> = VecDeque::new();

        let source_up = vec!['S', '|', 'J', 'L'];
        let target_up = vec!['|', '7', 'F'];

        let source_down = vec!['S', '|', '7', 'F'];
        let target_down = vec!['|', 'J', 'L'];

        let source_west = vec!['S', '-', 'J', '7'];
        let target_west = vec!['-', 'L', 'F'];
        
        let source_east = vec!['S', '-', 'L', 'F'];
        let target_east = vec!['-', 'J', '7'];

        our_loop.push(start.clone());
        q.push_back(start.clone());

        while let Some(current) = q.pop_front() {
            let ch = maze[current.0][current.1];

            if current.0 > 0 && source_up.contains(&ch) && target_up.contains(&maze[current.0-1][current.1]) && !our_loop.contains(&Coordinate(current.0-1, current.1)) {
                our_loop.push(Coordinate(current.0-1, current.1));
                q.push_back(Coordinate(current.0-1, current.1));
                //Remove if you don't want an ordered loop vec
                continue;
            }

            if current.0 < maze.len() -1 && source_down.contains(&ch) && target_down.contains(&maze[current.0+1][current.1]) && !our_loop.contains(&Coordinate(current.0+1, current.1)) {
                our_loop.push(Coordinate(current.0+1, current.1));
                q.push_back(Coordinate(current.0+1, current.1));
                //Remove if you don't want an ordered loop vec
                continue;
            }

            if current.1 > 0 && source_west.contains(&ch) && target_west.contains(&maze[current.0][current.1-1]) && !our_loop.contains(&Coordinate(current.0, current.1-1)) {
                our_loop.push(Coordinate(current.0, current.1-1));
                q.push_back(Coordinate(current.0, current.1-1));
                //Remove if you don't want an ordered loop vec
                continue;
            }

            if current.1 < (maze[current.0].len() -1) && source_east.contains(&ch) && target_east.contains(&maze[current.0][current.1+1]) && !our_loop.contains(&Coordinate(current.0, current.1+1)) {
                our_loop.push(Coordinate(current.0, current.1+1));
                q.push_back(Coordinate(current.0, current.1+1));
                //Remove if you don't want an ordered loop vec
                continue;
            }
        }


        return our_loop;
    }


}

impl AdventRunnable for Day10Runnable {
    fn run_part1(&self, file_contents: String) -> String {
        let mut maze:Vec<Vec<char>> = Vec::new();

        for l in file_contents.lines() {
            maze.push(l.chars().collect_vec());
        }

        let our_loop = self.find_loop(&maze);
        println!("Our loop is: {:?}", our_loop);
        (our_loop.len() / 2).to_string()
    }

    fn run_part2(&self, file_contents: String) -> String {
        let mut maze:Vec<Vec<char>> = Vec::new();

        for l in file_contents.lines() {
            maze.push(l.chars().collect_vec());
        }

        let mut our_loop = self.find_loop(&maze);
        our_loop.push(our_loop[0].clone());

        String::from("Need to port python code to find poly points")
    }
}
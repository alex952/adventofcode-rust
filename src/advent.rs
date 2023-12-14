use std::fs;

pub trait AdventRunnable {
    fn run_part1(&self, file_contents: String) -> String;
    fn run_part2(&self, file_contents: String) -> String;
    fn run(&self, filename: String, first: bool) -> String {

        let text = fs::read_to_string(filename).expect("Read file");
        
        match first {
            true => self.run_part1(text),
            false => self.run_part2(text),
        }
    }
}
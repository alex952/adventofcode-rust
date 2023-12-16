mod day10;
mod day11;
mod day12;
mod day15;
mod day16;

use std::env;

use advent::AdventRunnable;

use day10::Day10Runnable;
use day11::Day11Runnable;
use day12::Day12Runnable;
use day15::Day15Runnable;
use day16::Day16Runnable;


pub fn get_runnable(day: i32) -> Option<Box<dyn AdventRunnable>> {
    return match day {
        10 => Some(Box::new(Day10Runnable)),
        11 => Some(Box::new(Day11Runnable)),
        12 => Some(Box::new(Day12Runnable)),
        15 => Some(Box::new(Day15Runnable)),
        16 => Some(Box::new(Day16Runnable)),
        _ => None
    }
}

fn main() {

    let mut parser = clargs::ParsingConfig::new();
    parser.add_param(String::from("input"), true);
    parser.add_alias(String::from("i"), String::from("input"));
    parser.add_param(String::from("day"), true);
    parser.add_alias(String::from("d"), String::from("day"));
    parser.add_flag(String::from("first"));

    let args = env::args().into_iter();

    let results = match clargs::parse(args, &parser) {
        Ok(results) => results,
        Err(error) => {
            println!("{:?}", error);
            return;
        }
    };

    let input = results.get_param(&String::from("input")).unwrap();
    let day = match results.get_param_as::<i32>(&String::from("day")).unwrap() {
        Ok(day) => day,
        Err(_)  => {
            println!("Couldn't parse day");
            return;
        }
    };

    let first = results.has_flag("first");

    let result = match get_runnable(day) {
        Some(runnable) => runnable.run(String::from(input), first),
        None => String::from("No result implemented for that day")
    };

    println!("Result: {:?}", result);
}

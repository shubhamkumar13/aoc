mod aoc_2022;

use lazy_static::lazy_static;
use std::{fs::File, io::Read, path::Path};

use crate::aoc_2022::day1;
use crate::aoc_2022::day2;
use crate::aoc_2022::day3;

fn parse_input(source: &str) -> String {
    let path = Path::new(source);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {display}: {why}"),
        Ok(file) => file,
    };

    let mut buf = String::new();
    let _ = file.read_to_string(&mut buf);

    buf
}

fn generate_input_path(s: &str) -> String {
    let mut input_path = String::from("/home/sk/aoc/src/aoc_2022/input/");
    input_path.push_str(s);
    input_path
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day1_example_p1() {
        let path = "day1_example_input.txt";
        let s = parse_input(generate_input_path(path).as_str());
        let result = day1::p1::solution(s);
        assert_eq!(result, 24000);
    }

    #[test]
    fn day1_p1() {
        let path = "day1_input.txt";
        let s = parse_input(generate_input_path(path).as_str());
        let result = day1::p1::solution(s);
        assert_eq!(result, 70116);
    }

    #[test]
    fn day1_example_p2() {
        let path = "day1_example_input.txt";
        let s = parse_input(generate_input_path(path).as_str());
        let result = day1::p2::solution(s);
        assert_eq!(result, 45000);
    }

    #[test]
    fn day1_p2() {
        let path = "day1_input.txt";
        let s = parse_input(generate_input_path(path).as_str());
        let result = day1::p2::solution(s);
        let res = 206582;
        assert_eq!(result, res);
    }

    #[test]
    fn day2_example_p1() {
        let path = "day2_example_input.txt";
        let s = parse_input(generate_input_path(path).as_str());
        let result = day2::p1::solution(s);
        assert_eq!(result, 15);
    }

    #[test]
    fn day2_p1() {
        let path = "day2_input.txt";
        let s = parse_input(generate_input_path(path).as_str());
        let result = day2::p1::solution(s);
        assert_eq!(result, 14069);
    }

    #[test]
    fn day2_example_p2() {
        let path = "day2_example_input.txt";
        let s = parse_input(generate_input_path(path).as_str());
        let result = day2::p2::solution(s);
        assert_eq!(result, 12);
    }

    #[test]
    fn day2_p2() {
        let path = "day2_input.txt";
        let s = parse_input(generate_input_path(path).as_str());
        let result = day2::p2::solution(s);
        assert_eq!(result, 12411);
    }

    #[test]
    fn day3_example_p1() {
        let path = "day3_example_input.txt";
        let s = parse_input(generate_input_path(path).as_str());
        let result = day3::p1::solution(s);
        assert_eq!(result, 157)
    }

    #[test]
    fn day3_p1() {
        let path = "day3_input.txt";
        let s = parse_input(generate_input_path(path).as_str());
        let result = day3::p1::solution(s);
        assert_eq!(result, 7746);
    }

    #[test]
    fn day3_example_p2() {
        let path = "day3_example_input.txt";
        let s = parse_input(generate_input_path(path).as_str());
        let result = day3::p2::solution(s);
        // println!("{result}");
        assert_eq!(result, 70)
    }

    #[test]
    fn day3_p2() {
        let path = "day3_input.txt";
        let s = parse_input(generate_input_path(path).as_str());
        let result = day3::p2::solution(s);
        println!("{result}");
        assert_eq!(result, 2604)
    }
}

mod aoc_2022;

use std::{fs::File, io::Read, path::Path};

use crate::aoc_2022::day1;
use crate::aoc_2022::day2;

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

#[cfg(test)]
mod tests {
    use std::result;

    use super::*;

    #[test]
    fn day1_example_p1() {
        let s = parse_input("/home/sk/aoc/src/day1_example_input.txt");
        let result = day1::p1::solution(s);
        assert_eq!(result, 24000);
    }

    #[test]
    fn day1_real_p1() {
        let s = parse_input("/home/sk/aoc/src/day1_input.txt");
        let result = day1::p1::solution(s);
        assert_eq!(result, 70116);
    }

    #[test]
    fn day1_example_p2() {
        let s = parse_input("/home/sk/aoc/src/day1_example_input.txt");
        let result = day1::p2::solution(s);
        assert_eq!(result, 45000);
    }

    #[test]
    fn day1_real_p2() {
        let s = parse_input("/home/sk/aoc/src/day1_input.txt");
        let result = day1::p2::solution(s);
        let res = 206582;
        assert_eq!(result, res);
    }

    #[test]
    fn day2_example_p1() {
        let s = parse_input("/home/sk/aoc/src/day2_example_input.txt");
        let result = day2::p1::solution(s);
        assert_eq!(result, 15);
    }

    #[test]
    fn day2_real_p1() {
        let s = parse_input("/home/sk/aoc/src/day2_input.txt");
        let result = day2::p1::solution(s);
        assert_eq!(result, 14069);
    }

    #[test]
    fn day2_example_p2() {
        let s = parse_input("/home/sk/aoc/src/day2_example_input.txt");
        let result = day2::p2::solution(s);
        assert_eq!(result, 12);
    }

    #[test]
    fn day2_real_p2() {
        let s = parse_input("/home/sk/aoc/src/day2_input.txt");
        let result = day2::p2::solution(s);
        println!("{result}");
        assert!(true)
    }
}

mod aoc_2022;

use std::{fs::File, io::Read, path::Path};

use crate::aoc_2022::day1::p1::solution;

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
    use super::*;

    #[test]
    fn day1_example_p1() {
        let v = parse_input("/home/sk/aoc/src/p1_example_input.txt");
        let result = solution(v);
        assert_eq!(result, 24000);
    }

    #[test]
    fn day1_real_p1() {
        let v = parse_input("/home/sk/aoc/src/p1_input.txt");
        let result = solution(v);
        assert_eq!(result, 70116)
    }
}

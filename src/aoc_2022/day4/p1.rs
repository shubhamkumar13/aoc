use std::collections::HashSet;

use crate::aoc_2022::day4::common::parse_input;

pub fn solution(input: String) -> usize {
    let input = parse_input(input);
    input
        .into_iter()
        .filter(|v| {
            let intersection: HashSet<u64> = v[0].intersection(&v[1]).map(|x| x.clone()).collect();

            v[0] == intersection || v[1] == intersection
        })
        .count()
}

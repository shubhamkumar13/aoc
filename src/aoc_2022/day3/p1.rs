use crate::aoc_2022::day3::common::{parse_input, PRIORITY_MAP};
use std::collections::HashSet;

fn string_to_hashset_vec(s: String) -> HashSet<char> {
    HashSet::from_iter(s.chars())
}

pub fn solution(input: String) -> usize {
    let input = parse_input(input);

    input
        .into_iter()
        .map(|mut item| {
            let len = item.len();
            let r_item: HashSet<char> = string_to_hashset_vec(item.split_off(len / 2));
            let l_item: HashSet<char> = string_to_hashset_vec(item);

            let common_char_vec: Vec<char> =
                l_item.intersection(&r_item).map(|x| x.clone()).collect();

            common_char_vec
                .into_iter()
                .map(|x| PRIORITY_MAP.get(&x).unwrap().clone())
                .sum::<usize>()
        })
        .sum()
}

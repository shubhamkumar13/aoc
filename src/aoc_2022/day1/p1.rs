use crate::aoc_2022::day1::common::parse_input;

pub fn solution(input: String) -> u64 {
    let vec_of_vec_num = parse_input(input);

    vec_of_vec_num
        .into_iter()
        .filter_map(|x| x.into_iter().reduce(|a, b| a + b))
        .max()
        .unwrap()
}

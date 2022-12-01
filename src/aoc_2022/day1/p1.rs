use std::vec;

use crate::parse_input;

fn parse_input_p1(source: String) -> Vec<Vec<u64>> {
    source
        .split("\n\n")
        .map(|x| {
            x.to_string()
                .split("\n")
                .filter_map(|x| x.to_string().parse::<u64>().ok())
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<_>>()
}

pub fn solution(input: String) -> u64 {
    let vec_of_vec_num = parse_input_p1(input);

    vec_of_vec_num
        .into_iter()
        .filter_map(|x| x.into_iter().reduce(|a, b| a + b))
        .max()
        .unwrap()
}

use crate::aoc_2022::day1::common::parse_input;

pub fn solution(input: String) -> u64 {
    let vec_of_vec_num = parse_input(input);

    let mut vec_of_vec_num = vec_of_vec_num
        .into_iter()
        .filter_map(|x| x.into_iter().reduce(|a, b| a + b))
        .collect::<Vec<_>>();

    vec_of_vec_num.sort_by(|a, b| b.cmp(a));

    let (top_three, _) = vec_of_vec_num.split_at(3);
    top_three.to_vec().into_iter().reduce(|a, b| a + b).unwrap()
}

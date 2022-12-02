use crate::aoc_2022::day2::common::{parse_input, Decision, RPS_DECISION_MAP_P1};

pub fn solution(input: String) -> u64 {
    let input = parse_input(input);
    input
        .into_iter()
        .map(
            |(a, b)| match RPS_DECISION_MAP_P1.get(&(a.clone(), b.clone())) {
                Some(d) => match d {
                    Decision::Draw(x) => x.clone(),
                    Decision::Win(x) => x.clone(),
                    Decision::Loss(x) => x.clone(),
                },
                None => panic!("Pair ({a}, {b}) not found"),
            },
        )
        .sum()
}

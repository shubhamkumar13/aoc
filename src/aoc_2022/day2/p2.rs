use crate::aoc_2022::day2::common::{parse_input, Decision};
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    // X means you need to lose
    // Y means you need to draw
    // Z means you need to win
    static ref RPS_DECISION_MAP: HashMap<(String, String), Decision> = {
        HashMap::from([
            (("A".to_string(), "X".to_string()), Decision::Loss(3 + 0)),
            (("A".to_string(), "Y".to_string()), Decision::Draw(1 + 3)),
            (("A".to_string(), "Z".to_string()), Decision::Win(2 + 6)),
            (("B".to_string(), "X".to_string()), Decision::Loss(1 + 0)),
            (("B".to_string(), "Y".to_string()), Decision::Draw(2 + 3)),
            (("B".to_string(), "Z".to_string()), Decision::Win(3 + 6)),
            (("C".to_string(), "X".to_string()), Decision::Loss(2 + 0)),
            (("C".to_string(), "Y".to_string()), Decision::Draw(3 + 3)),
            (("C".to_string(), "Z".to_string()), Decision::Win(1 + 6))
        ])
    };
}

pub fn solution(input: String) -> u64 {
    let input = parse_input(input);
    input
        .into_iter()
        .map(
            |(a, b)| match RPS_DECISION_MAP.get(&(a.clone(), b.clone())) {
                Some(d) => match d {
                    Decision::Draw(x) => x.clone(),
                    Decision::Loss(x) => x.clone(),
                    Decision::Win(x) => x.clone(),
                },
                None => panic!("Pair ({a}, {b}) not found"),
            },
        )
        .sum()
}

use lazy_static::lazy_static;
use std::collections::HashMap;

pub enum Decision {
    Win(u64),
    Loss(u64),
    Draw(u64),
}

pub fn parse_input(source: String) -> Vec<(String, String)> {
    source
        .split("\n")
        .map(|x| {
            let s = x.to_string();
            let vec_s = s.split(' ').map(|x| x.to_string()).collect::<Vec<String>>();
            (vec_s[0].clone(), vec_s[1].clone())
        })
        .collect()
}

lazy_static! {
    // A and X Rock
    // B and Y Paper
    // C and Z Scissor
    pub static ref RPS_DECISION_MAP_P1: HashMap<(String, String), Decision> = {
        HashMap::from([
            (("A".to_string(), "X".to_string()), Decision::Draw(1 + 3)),
            (("A".to_string(), "Y".to_string()), Decision::Win(2 + 6)),
            (("A".to_string(), "Z".to_string()), Decision::Loss(3 + 0)),
            (("B".to_string(), "X".to_string()), Decision::Loss(1 + 0)),
            (("B".to_string(), "Y".to_string()), Decision::Draw(2 + 3)),
            (("B".to_string(), "Z".to_string()), Decision::Win(3 + 6)),
            (("C".to_string(), "X".to_string()), Decision::Win(1 + 6)),
            (("C".to_string(), "Y".to_string()), Decision::Loss(2 + 0)),
            (("C".to_string(), "Z".to_string()), Decision::Draw(3 + 3)),
        ])
    };
    // X means you need to lose
    // Y means you need to draw
    // Z means you need to win
    pub static ref RPS_DECISION_MAP_P2: HashMap<(String, String), Decision> = {
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

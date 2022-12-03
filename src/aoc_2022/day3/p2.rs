use crate::aoc_2022::day3::common::{parse_input, string_to_hashset_vec, PRIORITY_MAP};

fn chunker(vec: Vec<String>) -> Vec<Vec<String>> {
    let mut chunk_vec: Vec<Vec<String>> = Vec::new();
    let mut temp_vec: Vec<String> = Vec::new();

    for (index, s) in vec.into_iter().enumerate() {
        match (index + 1) % 3 {
            0 => {
                temp_vec.push(s);
                chunk_vec.push(temp_vec.clone());
                temp_vec.clear();
            }
            _ => {
                temp_vec.push(s);
            }
        }
    }

    chunk_vec
}

fn intersection(vec_str: Vec<String>) -> char {
    let intersection_set: Option<String> = vec_str
        .clone()
        .into_iter()
        .reduce(|a, b| {
            let set1 = string_to_hashset_vec(a);
            let set2 = string_to_hashset_vec(b);
            String::from_iter(set1.intersection(&set2))
        })
        .and_then(|x| if x.len() > 1 { None } else { Some(x) });

    match intersection_set {
        None => panic!("{vec_str:?} does not have anything in common"),
        Some(x) => x.chars().nth(0).unwrap().clone(),
    }
}

pub fn solution(input: String) -> usize {
    let input = parse_input(input);
    let input = chunker(input);

    input
        .into_iter()
        .map(|v| {
            let ch = intersection(v);
            PRIORITY_MAP.get(&ch).unwrap().clone()
        })
        .sum()
}

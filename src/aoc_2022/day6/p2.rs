use std::collections::HashSet;

use itertools::Itertools;

pub fn solution(input: String) -> usize {
    let input = input.chars().collect_vec();
    let windowed_input_vec = input.as_slice().windows(14).collect_vec();

    windowed_input_vec
        .iter()
        .enumerate()
        .map(|(i, x)| (i, x.to_vec()))
        .filter_map(|(i, x)| {
            let set = x
                .clone()
                .into_iter()
                .fold(HashSet::new(), |mut acc: HashSet<char>, char| {
                    acc.insert(char);
                    acc
                });
            if set.len() < x.len() {
                None
            } else {
                Some(i)
            }
        })
        .nth(0)
        .unwrap_or_default()
        + 14
}

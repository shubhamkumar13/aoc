use itertools::Itertools;

use crate::aoc_2022::day5::common::{parse_input, CharStack, Instr};

fn exec_instr(from_vec: CharStack, to_vec: CharStack, move_: usize) -> (CharStack, CharStack) {
    let mut from_vec = from_vec;
    let updated_from_vec = from_vec
        .drain(..(from_vec.len() - move_))
        .collect::<Vec<_>>();
    let mut to_vec = to_vec;
    to_vec.append(&mut from_vec);
    (updated_from_vec, to_vec)
}

pub fn solution(input: String) -> String {
    let (mut stack_map, instr_vec) = parse_input(input);

    instr_vec.iter().for_each(|instr| {
        let from = stack_map
            .get(&instr.from)
            .and_then(|x| Some(x.clone()))
            .unwrap_or_default();
        let to = stack_map
            .get(&instr.to)
            .and_then(|x| Some(x.clone()))
            .unwrap_or_default();
        let (updated_from, updated_to) = exec_instr(from, to, instr.move_);
        stack_map.insert(instr.from, updated_from);
        stack_map.insert(instr.to, updated_to);
    });

    stack_map
        .into_iter()
        .sorted_by(|a, b| a.0.cmp(&b.0))
        .filter_map(|(_, mut v)| v.pop())
        .fold("".to_owned(), |mut acc, ch| {
            acc.push(ch);
            acc
        })
}

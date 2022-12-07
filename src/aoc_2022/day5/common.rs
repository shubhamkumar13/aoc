use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Instr {
    pub move_: usize,
    pub from: usize,
    pub to: usize,
}

pub type CharStack = Vec<char>;
pub type InstrVec = Vec<Instr>;

fn stack_str_to_stack_map(stack_str: &str) -> HashMap<usize, CharStack> {
    // //     [D]
    // // [N] [C]
    // // [Z] [M] [P]
    // // 1   2   3
    // // ------- to ---------
    // // 1 => ['N', 'Z']
    // // 2 => ['M', 'C', 'D']
    // // 3 => ['P']

    let mut stack_str: Vec<_> = stack_str.lines().collect();
    // remove dummy
    let _ = stack_str.remove(0);
    // remove last line
    let _ = stack_str.pop();

    stack_str
        .iter_mut()
        // top most must be the last row to index
        .rev()
        .flat_map(|row| {
            row.chars()
                // skip the first character
                .skip(1)
                // only choose every 4th character which is a letter
                .step_by(4)
                // index every (4n + 1)th time
                .enumerate()
                .filter(|(_, ch)| !ch.is_ascii_whitespace())
        })
        .map(|(index, ch)| (index + 1, ch))
        .into_group_map()
}

fn line_to_instr(instr: &str) -> Instr {
    // move 1 from 2 to 1
    // to
    // Instr {
    //  move_ : 1,
    //  from : 2,
    //  to : 1
    // }

    let instr_vec: Vec<usize> = instr
        .split_ascii_whitespace()
        .filter(|x| x.chars().all(|x| x.is_ascii_digit()))
        .filter_map(|x| x.parse::<usize>().ok())
        .collect();
    // println!("{instr_vec:?}");
    Instr {
        move_: instr_vec[0],
        from: instr_vec[1],
        to: instr_vec[2],
    }
}

fn instr_str_to_instr_stack(instr_str: &str) -> InstrVec {
    // move 1 from 2 to 1\nmove 3 from 1 to 3\nmove 2 from 2 to 1\nmove 1 from 1 to 2
    // to
    // [
    //     Instr {
    //         move_ : 1,
    //         from : 2,
    //         to : 1
    //     },
    //     Instr {
    //         move_ : 3,
    //         from : 1,
    //         to : 3
    //     },
    //     Instr {
    //         move_ : 2,
    //         from : 2,
    //         to : 1
    //     },
    //     Instr {
    //         move_ : 1,
    //         from : 1,
    //         to : 2
    //     },
    // ]
    Vec::from_iter(instr_str.split("\n").map(line_to_instr))
}

pub fn parse_input(source: String) -> (HashMap<usize, Vec<char>>, InstrVec) {
    let (stack_str, instr_str) = source.split_once("\n\n").unwrap_or_default();
    let char_stack_map = stack_str_to_stack_map(stack_str);
    let instr_stack_map = instr_str_to_instr_stack(instr_str);
    (char_stack_map, instr_stack_map)
}

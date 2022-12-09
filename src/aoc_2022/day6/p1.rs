use itertools::Itertools;

pub fn solution(input: String) -> usize {
    input
        .chars()
        .enumerate()
        .tuple_windows::<((_, _), (_, _), (_, _), (_, _))>()
        .filter_map(|(a, b, c, d)| {
            let unique_a_exists = (a.1 != b.1) && (a.1 != c.1) && (a.1 != d.1);
            let unique_b_exists = (b.1 != c.1) && (b.1 != d.1);
            let unique_c_exists = c.1 != d.1;

            if unique_a_exists && unique_b_exists && unique_c_exists {
                Some(d.0 + 1)
            } else {
                None
            }
        })
        .nth(0)
        .unwrap_or_default()
}

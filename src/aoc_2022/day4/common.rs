use std::collections::HashSet;

pub fn parse_input(source: String) -> Vec<Vec<HashSet<u64>>> {
    source
        .split('\n')
        .map(|x| {
            x.split(',')
                .map(|x| {
                    let range: Vec<u64> = x.split('-').map(|x| x.parse::<u64>().unwrap()).collect();

                    HashSet::from_iter(range[0]..=range[1])
                })
                .collect::<Vec<HashSet<u64>>>()
        })
        .collect::<Vec<_>>()
}

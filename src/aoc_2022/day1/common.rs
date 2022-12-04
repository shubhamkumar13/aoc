pub fn parse_input(source: String) -> Vec<Vec<u64>> {
    source
        .split("\n\n")
        .map(|x| {
            x.to_string()
                .split("\n")
                .filter_map(|x| x.to_string().parse::<u64>().ok())
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<_>>()
}

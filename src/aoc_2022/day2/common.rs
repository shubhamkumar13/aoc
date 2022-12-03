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

pub fn parse_input(source: String) -> Vec<String> {
    source.split('\n').map(|x| x.to_string()).collect()
}

use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref PRIORITY_MAP: HashMap<char, usize> = {

        // generate a priority map where
        // a ... z => 1 ... 26
        // A ... Z => 27 ... 52
        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
            .chars().zip(1..).collect()
    };
}

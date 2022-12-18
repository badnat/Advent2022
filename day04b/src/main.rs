use std::cmp::{max, min};

fn main() {
    let mut count = 0;
    let pairs = include_str!("../input").split('\n').map(|a| a.split(','));
    for pair in pairs {
        let p: Vec<&str>= pair.map(|b| b).collect();
        let range0: Vec<u8> = p[0].split("-").map(|c| c.parse::<u8>().unwrap()).collect();
        let range1: Vec<u8> = p[1].split("-").map(|d| d.parse::<u8>().unwrap()).collect();
        if  max(range0[0], range1[0]) <= min(range0[1], range1[1]){ count += 1; }
    }
    println!("{count}");
}
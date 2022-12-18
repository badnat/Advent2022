use std::collections::HashMap;

fn main() {
    let mut sum: u32 = 0;
    let binding = include_bytes!("../input").split(|b| *b == b'\n').collect::<Vec<_>>();
    let groups = binding.chunks(3);

    for g in groups {
        let mut hm0 = HashMap::<u8, u8>::new();
        for g0 in g[0] {
            hm0.insert(*g0, *g0);
        }
        let mut hm1 = HashMap::<u8, u8>::new();
        for g1 in g[1] {
            hm1.insert(*g1, *g1);
        }
        for g2 in g[2] {
            if hm0.get(&(*g2)).is_some() && hm1.get(&(*g2)).is_some() {
                if *g2 > 0x5A {
                    sum += *g2 as u32 - 0x60;
                } else {
                    sum += *g2 as u32 - 0x26;
                }
                break;
            }
        }
    }
    println!("{sum}");
}
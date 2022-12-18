use std::collections::HashMap;

fn main() {
    let mut sum: u32 = 0;
    let s: Vec<&[u8]> = include_bytes!("../input").split(|b| *b == b'\n').collect();
    for v in &s {
        let c: Vec<&[u8]> = v.chunks(v.len()/2).collect();
        let mut hm = HashMap::<u8, u8>::new();
        for c0 in c[0] {
            hm.insert(*c0, *c0);
        }
        for c1 in c[1] {
            if hm.get(&(*c1)).is_some() {
                if *c1 > 0x5A {
                    sum += *c1 as u32 - 0x60;
                } else {
                    sum += *c1 as u32 - 0x26;
                }
                break;
            }
        }
    }
    println!("{sum}");
}

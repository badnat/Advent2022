fn main() {
    let mut sum: u32 = 0;
    let binding = include_bytes!("../input").split(|b| *b == b'\n').collect::<Vec<_>>();
    let mut groups = binding.chunks(3);

    for g in groups {
        for g0 in g[0] {
            if g[1].contains(g0) && g[2].contains(g0) {
                if *g0 > 0x5A {
                    sum += *g0 as u32 - 0x60;
                } else {
                    sum += *g0 as u32 - 0x26;
                }
                break;
            }
        }
    }
    println!("{sum}");
}
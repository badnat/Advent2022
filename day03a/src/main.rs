fn main() {
    let mut sum: u32 = 0;
    let s: Vec<&[u8]> = include_bytes!("../input").split(|b| *b == b'\n').collect();
    for v in &s {
        let c: Vec<&[u8]> = v.chunks(v.len()/2).collect();
        for c0 in c[0] {
            if c[1].contains(c0) {
                if *c0 > 0x5A {
                    sum += *c0 as u32 - 0x60;
                } else {
                    sum += *c0 as u32 - 0x26;
                }
                break;
            }
        }  
    }
    println!("{sum}") 
}

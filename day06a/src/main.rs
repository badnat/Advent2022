fn main() {
    let v: Vec<char> = include_str!("../input").chars().collect();
    for i in 3..v.len() {
        let mut b: bool = false;
        for j in i-3..i {
            for k in j+1..i+1 {
                b = b || v[k] == v[j];
            }
        }
        if !b {
            println!("{}", i+1);
            break;
        }
    }
}

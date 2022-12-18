
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {

    let mut total_calories: u32 = 0;
    let mut max_calories: u32 = 0;

    let mut iter = read_lines("input").expect("REASON").peekable();
    while let Some(line) = iter.next() {
        if line.as_ref().unwrap()=="" {
            if total_calories >= max_calories { max_calories = total_calories;}
            total_calories = 0;
        } else if iter.peek().is_none() {
            total_calories += line.unwrap().parse::<u32>().unwrap();
            if total_calories >= max_calories { max_calories = total_calories;}
        } else {
            total_calories += line.unwrap().parse::<u32>().unwrap();
        }
    }
    println!("{}", max_calories);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
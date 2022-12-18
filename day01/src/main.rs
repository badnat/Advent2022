
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {

    let mut total_calories: u32 = 0;
    let mut calories0: u32 = 0;
    let mut calories1: u32 = 0;
    let mut calories2: u32 = 0;

    let mut iter = read_lines("input").expect("REASON");
    while let Some(line) = iter.next() {
        if line.as_ref().unwrap()=="" {
            if total_calories >= calories0 {
                calories2 = calories1;
                calories1 = calories0; 
                calories0 = total_calories;
            } else if total_calories >= calories1 {
                calories2 = calories1; 
                calories1 = total_calories;
            } else if total_calories >= calories2 {
                calories2 = total_calories;
            }
            total_calories = 0;
        } else {
            total_calories += line.unwrap().parse::<u32>().unwrap();
        }
    }
    if total_calories >= calories0 {
        calories2 = calories1;
        calories1 = calories0; 
        calories0 = total_calories;
    } else if total_calories >= calories1 {
        calories2 = calories1; 
        calories1 = total_calories;
    } else if total_calories >= calories2 {
        calories2 = total_calories;
    }
    println!("{}", calories0 + calories1 + calories2);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
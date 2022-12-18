use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut score: u32 = 0;

    let lines = read_lines("input").expect("REASON");
    for line in lines {
        let vec: Vec<&str> = line.as_ref().unwrap().split(" ").collect();

        match vec[1] {
            "X"=> {
                score += 0;
                match vec[0] {
                    "A"=> score += 3,
                    "B"=> score += 1,
                    "C"=> score += 2,
                    _=> println!("uh oh"),
                }
            },
            "Y"=> {
                score += 3;
                match vec[0] {
                    "A"=> score += 1,
                    "B"=> score += 2,
                    "C"=> score += 3,
                    _=> println!("uh oh"),
                }
            },
            "Z"=> {
                score += 6;
                match vec[0] {
                    "A"=> score += 2,
                    "B"=> score += 3,
                    "C"=> score += 1,
                    _=> println!("uh oh"),
                }
            },
            _=> println!("uh oh"),
        }

        // part 1 grave yard
        // if num_rep[0] == num_rep[1] { // Draw
        //     score += 3 + num_rep[1];
        // } else if (num_rep[0] < num_rep[1] || (num_rep[0] == 3 && num_rep[1] == 1)) && !(num_rep[1] == 3 && num_rep[0] == 1) { // W
        //     score += 6 + num_rep[1];
        // } else { // L
        //     score += num_rep[1];
        // }
    }
    println!("{}", score);
}

// part 1 grave yard
// fn translate(vec: Vec<&str>) -> [u32; 2] {
//     let mut num_rep: [u32; 2] = [0; 2];
//     match vec[0] {
//         "A"=> num_rep[0] = 1,
//         "B"=> num_rep[0] = 2,
//         "C"=> num_rep[0] = 3,
//         _=> println!("translation mismatch"),
//     }
//     match vec[1] {
//         "X"=> num_rep[1] = 1,
//         "Y"=> num_rep[1] = 2,
//         "Z"=> num_rep[1] = 3,
//         _=> println!("translation mismatch"),
//     }
//     return num_rep;
// }

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

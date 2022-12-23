use core::str::Split;
use std::collections::VecDeque;

fn main() {
    let v: Vec<&str> = include_str!("../input").split("\n\n").collect();
    let (stacks, moves) = (v[0].split('\n'), v[1].split('\n'));
    let stacks = get_stacks(stacks);
    let moves = get_moves(moves);

    move_stacks(stacks, moves);
}

// transforms the split from reading input into Vec<Vec<char>>
fn get_stacks(stacks: Split<char>) -> Vec<Vec<char>>{
    // trim up the stack data and store into a Vec<Vec<char>>
    let mut vec: Vec<Vec<char>> = Vec::new();
    for stack in stacks {
        let stack = stack.replace(&['[', ']'][..], " ");
        let stack: Vec<char> = stack[1..stack.len()].chars().step_by(4).collect();
        vec.push(stack);
    }
    // trim a little more and rotate to get actual stacks of the crates
    let mut vec2: Vec<Vec<char>> = Vec::new();
    for i in 0..vec[0].len() {
        let mut v: Vec<char> = Vec::new();
        for j in (0..vec.len()).rev() {
            if vec[j][i] != ' ' {v.push(vec[j][i]);}
        }
        vec2.push(v);
    }
    return vec2;
}

fn get_moves(moves: Split<char>) -> Vec<Vec<usize>> {
    let mut vec: Vec<Vec<usize>> = Vec::new();
    for m in moves {
        let m = m.replace(&['m', 'o', 'v', 'e', 'f', 'r', 't'][..], "");
        let m: Vec<&str> = m[1..m.len()].split("  ").collect();
        let mut c:  Vec<usize> = Vec::new();
        for i in m {
            c.push(i.parse().unwrap());
        } 
        vec.push(c);
    }
    return vec;
}

fn move_stacks(mut stacks: Vec<Vec<char>>, moves: Vec<Vec<usize>>) {
    for mv in moves {
        let mut s: VecDeque<char> = VecDeque::new();
        for _i in 0..mv[0]{
            s.push_back(stacks[mv[1]-1].pop().unwrap());
        }
        for _i in 0..mv[0]{
            stacks[mv[2]-1].push(s.pop_back().unwrap());
        }
    }
    for stack in &stacks {
        print!("{}", &stack[stack.len()-1]);
        // println!("{:?}", &stack);
    }
    println!("");
}
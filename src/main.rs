use std::env;
use std::fmt;
#[macro_use] extern crate itertools;

const TARGETVALUE: usize = 8;
const NUMBEROFMOVES: i8 = 6;
//const ALLOWEDTURNS: u8 = 5;

#[derive(Debug)]
enum Moves {
    Investment,
    Blackmail,
    Donation,
    Orgy,
    ExtraTime,
    Teambuilding
}

impl fmt::Display for Moves {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

fn get_move_by_idx(idx: usize) -> Moves {
    match idx {
        0 => Moves::Investment,
        1 => Moves::Blackmail,
        2 => Moves::Donation,
        3 => Moves::Orgy,
        4 => Moves::ExtraTime,
        5 => Moves::Teambuilding,
        _ => panic!("{:?}", idx),
    }
}

fn moveset_to_names(moves: Vec<usize>) -> Vec<String>{
    moves.into_iter().map(|m| get_move_by_idx(m as usize).to_string()).collect()
}

fn get_moveset(m: Moves) -> Vec<i8> {
    match m {
        Moves::Investment => vec![3, 0, -1],
        Moves::Blackmail => vec![-3, -2, 3],
        Moves::Donation => vec![2, 0, -2],
        Moves::Orgy => vec![-2, 2, 0],
        Moves::ExtraTime => vec![0, -3, 2],
        Moves::Teambuilding => vec![0, 3, -2],
    }
}

fn get_score(vals: Vec<usize>) -> usize {
    vals.into_iter().map(|v| i32::abs(v as i32 - TARGETVALUE as i32) as usize).sum()
}

fn main() {
    println!("{:?}", solve(env::args().skip(1).map(|a| a.parse::<usize>().unwrap()).collect::<Vec<usize>>()))
}

fn solve(input_vals: Vec<usize>) -> Vec<String>{

    // set it to be the worst score to start with
    let mut best_score = 3 * TARGETVALUE;
    let mut current_score = 0;
    let mut best_moveset = Vec::new();
    // make this actually work if NUMBEROFMOVES changes
    for (i, j, k, l, m) in iproduct!(0..NUMBEROFMOVES, 0..NUMBEROFMOVES, 0..NUMBEROFMOVES, 0..NUMBEROFMOVES, 0..NUMBEROFMOVES){
        let mut tmp_vals = input_vals.clone();
        let curr_moves = vec![i as usize, j as usize, k as usize, l as usize, m as usize];
        //println!("{};{};{};{};{};", i, j, k, l, m)
        for m in &curr_moves {
            tmp_vals = calc_move(tmp_vals, &m)
        }
        if tmp_vals == vec![TARGETVALUE, TARGETVALUE, TARGETVALUE]{
            return moveset_to_names(curr_moves)
        }
        current_score = get_score(tmp_vals);
        if current_score < best_score {
            best_score = current_score;
            best_moveset = curr_moves
        }
    }

    moveset_to_names(best_moveset)
}

fn calc_move(vals: Vec<usize>, move_idx: &usize) -> Vec<usize>{
    let mut out = Vec::new();
    for idx in 0..vals.len() {
        let mut val = vals.get(idx).unwrap().clone() as i8;
        let m = get_moveset(get_move_by_idx(*move_idx));
        val += m.get(idx).unwrap();
        if val < 1 {
            val = 1
        }
        if val > 16 {
            val = 16
        }
        out.push(val as usize)
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_moves_investment() {
        assert_eq!(vec![11, 8, 7], calc_move(vec![8,8,8], &0));
    }
    #[test]
    fn test_moves_blackmail() {
        assert_eq!(vec![5, 6, 11], calc_move(vec![8,8,8], &1));
    }
    #[test]
    fn test_moves_donation() {
        assert_eq!(vec![10, 8, 6], calc_move(vec![8,8,8], &2));
    }
    #[test]
    fn test_moves_orgy() {
        assert_eq!(vec![6, 10, 8], calc_move(vec![8,8,8], &3));
    }
    #[test]
    fn test_moves_extra_time() {
        assert_eq!(vec![8, 5, 10], calc_move(vec![8,8,8], &4));
    }
    #[test]
    fn test_moves_teambuilding() {
        assert_eq!(vec![8, 11, 6], calc_move(vec![8,8,8], &5));
    }
}

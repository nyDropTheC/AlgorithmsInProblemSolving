use std::collections::VecDeque;
use std::io;
use std::io::BufRead;

fn main() {
    let mut input = String::new();
    
    io::stdin ( )
        .read_line ( &mut input )
        .expect ( "Failed to read line" );

    let mut collection = io::stdin ( )
        .lock ( )
        .lines ( )
        .next ( )
        .unwrap ( )
        .unwrap ( )
        .trim ( )
        .split_whitespace ( )
        .map ( |s| s.parse::<i64> ( ).unwrap ( ) )
        .collect::<VecDeque<i64>> ( );

    let mut score: i64 = 0;
    let mut turn = true;

    // Greedy doesn't actually work here, nevermind
    
    while collection.len() > 1 {
        let front = collection.front().unwrap().clone();
        let back = collection.back().unwrap().clone();
        
        let optimal = if front > back {
            collection.pop_front();
            front
        } else {
            collection.pop_back();
            back
        };

        if turn {
            score += optimal;
        }

        turn = !turn;
    }

    println!("{}", score + collection.pop_front().unwrap());
}

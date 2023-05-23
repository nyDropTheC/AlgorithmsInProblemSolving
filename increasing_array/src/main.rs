use std::io;
use std::io::BufRead;

fn main() {
    let mut input = String::new();
    
    io::stdin ( )
        .read_line ( &mut input )
        .expect ( "Failed to read line" );

    // literally useless, probably for the cniles doing this

    let mut list: Vec<i64> = io::stdin ( )
        .lock ( )
        .lines ( )
        .next ( )
        .unwrap ( )
        .unwrap ( )
        .trim ( )
        .split_whitespace ( )
        .map ( |s| s.parse::<i64> ( ).unwrap ( ) )
        .collect::<Vec<i64>> ( );

    // rust... uh...

    let mut moves: i64 = 0;

    for i in 1..list.len() {
       let prev = list.get ( i - 1 ).unwrap ( );
       let curr = list.get ( i ).unwrap ( );

        if curr - prev < 0 {
            let delta = prev - curr;
            *&mut list[i] += delta;
            moves += delta;
        }
    }

    println!("{}", moves);
}

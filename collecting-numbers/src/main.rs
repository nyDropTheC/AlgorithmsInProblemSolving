use std::collections::HashSet;

use std::io;
use std::io::BufRead;

fn solve ( list : Vec<usize> ) -> i64 {
    let mut transposed = vec![ 0; list.len ( ) + 1 ];
    let mut iterations : i64 = 1;

    for ( iter, &value ) in list.iter ( ).enumerate ( ) {
        *&mut transposed [ value ] = iter;
    }

    // Not quite sure on how I'd go about transposing it straight out of user input
    // Actually, I know how - just can't be bothered
 
    for i in 1..list.len ( ) {
        if transposed [ i + 1 ] < transposed [ i ] {
            iterations += 1;
        }
    }

    iterations
}

fn main() {
    let mut input = String::new();
    
    io::stdin ( )
        .read_line ( &mut input )
        .expect ( "Failed to read line" );

    
    let list: Vec<usize> = io::stdin ( )
        .lock ( )
        .lines ( )
        .next ( )
        .unwrap ( )
        .unwrap ( )
        .trim ( )
        .split_whitespace ( )
        .map ( |s| s.parse::<usize> ( ).unwrap ( ) )
        .collect::<Vec<usize>> ( );

    //let list: Vec<i64> = Vec::from_iter ( ( 1..200000 + 1 ).rev ( ) );

    println! ( "{}", solve ( list ) );
}

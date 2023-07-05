use std::io;
use std::io::BufRead;

fn main() {
    let mut input = String::new();
    
    io::stdin ( )
        .read_line ( &mut input )
        .expect ( "Failed to read line" );

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

    list.sort_unstable();

    println!("{}", list.iter().fold(1i64, |acc, x| if x <= &acc { &acc + x } else { acc }));
}

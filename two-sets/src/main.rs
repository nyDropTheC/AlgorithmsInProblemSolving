use std::collections::HashSet;
use std::io;
use std::iter::FromIterator;

// 1+2+3+4=10

// 1+4=5 2+3=5

// 1+2+3+4+5+6+7=28

// 1+2+4+7=14 3+5+6=14

// get a comb the sum of which is half of full sum

// 1+2+3+4+5=21

// no, sum isn't even



fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let n : i64 = input.trim ( ).parse ( ).expect ( "Input a valid number" );

    let sum = n * ( n + 1 ) / 2;

    if sum % 2 == 0 {
        println!("YES");

        let mut set: HashSet<i64> = HashSet::new ( );
        let mut running_sum = 0;
        for i in (1..n + 1).rev ( ) {
            if running_sum + i <= sum / 2 {
                set.insert ( i );
                running_sum += i;
            }
        }

        let full_set = HashSet::from_iter ( 1..n+1 );
        let difference: HashSet<_> = full_set.difference(&set).collect();

        println!( "{}", set.len ( ) );
        println!( "{}", set.iter ( ).fold ( String::new ( ), |acc, &val| acc + " " + &val.to_string ( ) ).trim ( ) );
        
        println!( "{}", difference.len ( ) );
        println!( "{}", difference.iter ( ).fold ( String::new ( ), |acc, &val| acc + " " + &val.to_string ( ) ).trim ( ) );
    } else {
        // We can't proceed due to 
        println!("NO");
    }

    
}

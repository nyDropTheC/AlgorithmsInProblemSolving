use std::collections::HashSet;
use std::io;

fn permutations ( chars: Vec<char>, built: String, ret: &mut HashSet<String> ) {
    for ( index, &ch ) in chars.iter ( ).enumerate ( ) {
        let mut cloned = chars.clone ( );
        let mut added = built.clone ( );

        added.push ( ch );

        cloned.remove ( index );

        if cloned.is_empty ( ) {
            ret.insert ( added.clone ( ) );
        }

        permutations( cloned, added, ret );
    }
}

fn main() {
    let mut input = String::new ( );

    io::stdin ( )
        .read_line ( &mut input )
        .expect ( "Failed to read line" );

    let charset = input.trim ( ).chars ( ).collect::<Vec<char>> ( );

    let mut ret_set : HashSet<String> = HashSet::new ( );

    permutations( charset, String::new ( ), &mut ret_set );

    println! ( "{}", ret_set.len ( ) );

    let mut vec_set = ret_set.iter ( ).collect::<Vec<_>> ( );

    vec_set.sort ( );

    for ln in vec_set {
        println! ( "{}", ln );
    }
}

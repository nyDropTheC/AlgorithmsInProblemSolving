use std::{io, collections::HashSet};

fn main() {
    let mut input: String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let n = input.trim().parse::<usize>().expect("Input a valid number");

    let mut removed_indices: HashSet<usize> = HashSet::new();
    let mut removals: Vec<usize> = Vec::new();
    let mut flipper = false;
    while removed_indices.len() < n {
        for i in 1..=n {
            if !removed_indices.contains(&i) {
                if flipper {
                    removals.push(i);
                    removed_indices.insert(i);
                }
                flipper = !flipper;
            }
        }
    }

    println!("{}", removals.iter().fold(String::new(), |acc, &add| acc + " " + &add.to_string()).trim());
}

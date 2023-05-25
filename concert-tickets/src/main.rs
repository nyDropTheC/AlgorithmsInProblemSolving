use std::collections::BTreeMap;
use std::io;
use std::io::BufRead;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let mut ticket_map: BTreeMap<i32, i32> = BTreeMap::new();

    for i in io::stdin()
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>() {
            match ticket_map.contains_key(&i) {
                false => ticket_map.insert(i, 1),
                true => ticket_map.insert(i, ticket_map.get(&i).unwrap() + 1)
            };
    }

    for customer in io::stdin()
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>() {
            let mut found = false;
            let range = ticket_map.range(..=customer).rev();

            let mut inserted: Vec<(i32, i32)> = Vec::new();
            let mut deleted: Vec<i32> = Vec::new();

            for (ticket_price, ticket_count) in range {
                found = true;

                let new_count = *ticket_count - 1;

                if new_count > 0 {
                    inserted.push((*ticket_price, new_count));
                } else {
                    deleted.push(*ticket_price);
                }

                println!("{}", ticket_price);
                    
                break;
            }

            for (k, v) in inserted {
                ticket_map.insert(k, v);
            }

            for k in deleted {
                ticket_map.remove(&k);
            }

            if !found {
                println!("-1");
            }
    }
}

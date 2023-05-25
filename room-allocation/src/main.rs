use std::io;
use std::collections::BTreeMap;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let mut customers: Vec<(i64, i64)> = Vec::new();

    for i in 0..input.trim().parse::<i64>().expect("Failed to parse") {
        let mut line = String::new();

        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        let mut parts = line
            .trim()
            .split_whitespace()
            .map(|n| n.trim().parse::<i64>().expect("Failed to parse"));

        match ( parts.next(), parts.next() ) {
            (Some(a), Some(b)) => {
                customers.push((b, a));
            },
            _ => {}
        }
    }

    let mut room_map: BTreeMap<i64, (i64, i64)> = BTreeMap::new();
    let mut allocations: Vec<i64> = Vec::new();
    let mut room_id_counter: i64 = 0;

    for (departure, arrival) in customers {
        let mut removals: Vec<i64> = Vec::new();
        let mut additions: Vec<(i64, i64)> = Vec::new();

        match room_map.range(..arrival).rev().next() {
            Some((previous_departure, (room_id, _))) => {
                allocations.push(*room_id);

                additions.push((departure, *room_id));
                removals.push(previous_departure.clone());
            },
            None => {
                room_id_counter += 1;
                allocations.push(room_id_counter);

                room_map.insert(departure, (room_id_counter, 0));
            }
        }

        for i in removals {
            room_map.remove(&i);
        }

        for (new_departure, room_id) in additions {
            room_map.insert(new_departure, (room_id, 0));
        }
    }

    println!("{}", room_id_counter);
    println!("{}", allocations.iter().fold(String::new(), |acc, &add| acc + " " + &add.to_string()).trim());
}

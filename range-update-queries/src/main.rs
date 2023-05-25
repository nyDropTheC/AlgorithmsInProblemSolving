use std::io;
use std::io::BufRead;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let mut parts = input.split_whitespace().map(|s| s.parse::<i64>());
    parts.next();
    let query_count: i64 = parts.next().unwrap().unwrap();

    let list: Vec<usize> = io::stdin()
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let mut range_list: Vec<(usize, usize, usize)> = Vec::new();
    let mut resp_buffer: Vec<usize> = Vec::new();

    for _i in 0..query_count {
        let mut line = String::new();

        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        let mut parts = line.split_whitespace().map(|s| s.parse::<usize>());

        match (parts.next(), parts.next(), parts.next(), parts.next()) {
            (Some(Ok(1)), Some(Ok(a)), Some(Ok(b)), Some(Ok(n))) => {
                range_list.push((a, b, n));
            },
            (Some(Ok(2)), Some(Ok(n)), None, None) => {
                let mut orig = list.get(n - 1).unwrap().clone();

                for (a, b, increase) in range_list.clone() {
                    if a <= n && b >= n {
                        orig += increase;
                    }
                }

                resp_buffer.push(orig);
            },
            _ => {}
        }
    }

    for resp in resp_buffer {
        println!("{}", resp);
    }
}

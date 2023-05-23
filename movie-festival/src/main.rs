use std::cmp::Ordering;
use std::fmt::Debug;
use std::io;

#[derive(Debug)]
struct Movie {
    start: i32,
    end: i32,
}

impl Ord for Movie {
    fn cmp(&self, other: &Self) -> Ordering {
        self.end.cmp(&other.end)
    }
}

impl PartialOrd for Movie {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Movie {
    fn eq(&self, other: &Self) -> bool {
        self.end == other.end
    }
}

impl ToString for Movie {
    fn to_string(&self) -> String {
        format!("{} {}", self.start, self.end)
    }
}

impl Eq for Movie {}

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let n: usize = input.trim().parse().expect("Input a valid number");
    let mut storage: Vec<Movie> = Vec::new();

    for _i in 0..n {
        let mut line = String::new();

        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        let mut parts = line.split_whitespace().map(|s| s.parse::<i32>());

        storage.push(Movie {
            start: parts.next().unwrap().unwrap(),
            end: parts.next().unwrap().unwrap(),
        });
    }

    storage.sort();

    let mut attainable_end = i32::MIN;
    let mut solution = 0;

    for movie in storage {
        if movie.start >= attainable_end {
            attainable_end = movie.end;
            solution += 1;
        }
    }

    println!("{}", solution);
}

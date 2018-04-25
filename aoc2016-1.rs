use std::io::Read;

#[derive(Clone, Copy, Debug)]
enum Turn {
    Left,
    Right,
}

fn turn(s: &str) -> Turn {
    match s {
        "L" => Turn::Left,
        "R" => Turn::Right,
        t => panic!("bad turn description {}", t),
    }
}

fn steps(s: &str) -> u64 {
    s.parse().expect(&format!("bad steps description {}", s))
}

fn main() {
    let mut input: String = String::new();
    let _ = std::io::stdin()
        .read_to_string(&mut input)
        .expect("input read failed");
    for mov in input.trim().split(", ") {
        let (t, s) = mov.split_at(1);
        println!("{:?} {}", turn(t), steps(s));
    }
}

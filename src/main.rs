// Copyright © 2018 Bart Massey
// This code is available under the "MIT License". Please see
// the file "LICENSE" in this distribution for license terms.

// Advent of Code 2016 Problem 1 in Rust

use std::io::Read;

#[derive(Clone, Copy, Debug)]
struct Posn(i64, i64);

impl Posn {
    fn origin_distance(self) -> i64 {
        let Posn(x, y) = self;
        x.abs() + y.abs()
    }
}

#[derive(Clone, Copy, Debug)]
enum Turn {
    Left,
    Right,
}
use Turn::*;

#[derive(Clone, Copy, Debug)]
enum Dirn {
    North,
    East,
    South,
    West,
}
use Dirn::*;

impl Dirn {
    fn turn(self, turn: Turn) -> Dirn {
        match turn {
            Left => match self {
                North => West,
                East => North,
                South => East,
                West => South,
            },
            Right => match self {
                North => East,
                East => South,
                South => West,
                West => North,
            },
        }
    }

    fn step(self, Posn(x, y): Posn) -> Posn {
        match self {
            North => Posn(x, y - 1),
            South => Posn(x, y + 1),
            East => Posn(x + 1, y),
            West => Posn(x - 1, y),
        }
    }
}

fn to_turn(s: &str) -> Turn {
    match s {
        "L" => Left,
        "R" => Right,
        t => panic!("bad turn description {}", t),
    }
}

fn steps(s: &str) -> u64 {
    s.parse().expect(&format!("bad steps description {}", s))
}

fn main() {
    let mut input = String::new();
    let _ = std::io::stdin()
        .read_to_string(&mut input)
        .expect("input read failed");
    let mut dirn = North;
    let mut posn = Posn(0, 0);
    for mov in input.trim().split(", ") {
        let (t, s) = mov.split_at(1);
        let turn = to_turn(t);
        let steps = steps(s);
        dirn = dirn.turn(turn);
        for _ in 0..steps {
            posn = dirn.step(posn);
        }
    }
    println!("{}", posn.origin_distance());
}

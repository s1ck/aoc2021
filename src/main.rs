#![allow(dead_code)]
#![feature(array_windows)]
#![feature(test)]
#![feature(int_abs_diff)]

extern crate test;

use std::fmt::Debug;
use std::str::FromStr;

mod d01;
mod d02;
mod d03;
mod d04;
mod d05;
mod d06;
mod d07;
mod d08;
mod d09;
mod d10;
mod d11;
mod d12;
mod d13;
mod d14;
mod d15;
mod d16;
mod d17;
mod d18;
mod d19;
mod d20;
mod d21;
mod d22;
mod d23;
mod d24;
mod d25;

fn main() {
    let day = std::env::args()
        .nth(1)
        .map(|d| d.parse::<u32>().unwrap())
        .unwrap();

    let (res1, res2) = match day {
        1 => d01::run(&read_file("input/d01.txt")),
        2 => d02::run(&read_file("input/d02.txt")),
        3 => d03::run(
            read_file_with("input/d03.txt", |line| line.to_string())
                .iter()
                .map(|s| s.as_str())
                .collect::<Vec<_>>()
                .as_slice(),
        ),
        4 => d04::run(
            read_file_with("input/d04.txt", |line| line.to_string())
                .iter()
                .map(|s| s.as_str())
                .collect::<Vec<_>>()
                .as_slice(),
        ),
        5 => d05::run(&read_file("input/d05.txt")),
        6 => d06::run(&read_file_with("input/d06.txt", d06::parse)[0]),
        7 => d07::run(&read_file_with("input/d07.txt", d07::parse)[0]),
        8 => d08::run(&read_file("input/d08.txt")),
        9 => d09::run(read_file_with("input/d09.txt", d09::parse)),
        10 => d10::run(
            read_file_with("input/d10.txt", |line| line.to_string())
                .iter()
                .map(|s| s.as_str())
                .collect::<Vec<_>>()
                .as_slice(),
        ),
        11 => d11::run(
            read_file_with("input/d11.txt", |line| line.to_string())
                .iter()
                .map(|s| s.as_str())
                .collect::<Vec<_>>()
                .as_slice(),
        ),
        12 => d12::run(
            read_file_with("input/d12.txt", |line| line.to_string())
                .iter()
                .map(|s| s.as_str())
                .collect::<Vec<_>>()
                .as_slice(),
        ),
        13 => d13::run(
            std::fs::read_to_string("input/d13.txt")
                .expect("file not found")
                .as_str(),
        ),
        14 => d14::run(
            std::fs::read_to_string("input/d14.txt")
                .expect("file not found")
                .as_str(),
        ),
        15 => d15::run(
            std::fs::read_to_string("input/d15.txt")
                .expect("file not found")
                .as_str(),
        ),
        16 => d16::run(
            std::fs::read_to_string("input/d16.txt")
                .expect("file not found")
                .as_str(),
        ),
        17 => d17::run(
            std::fs::read_to_string("input/d17.txt")
                .expect("file not found")
                .as_str(),
        ),
        18 => d18::run(read_file("input/d18.txt")),
        19 => d19::run(
            std::fs::read_to_string("input/d19.txt")
                .expect("file not found")
                .as_str(),
        ),
        20 => d20::run(
            std::fs::read_to_string("input/d20.txt")
                .expect("file not found")
                .as_str(),
        ),
        21 => d21::run(read_file("input/d21.txt")),
        22 => d22::run(read_file("input/d22.txt")),
        23 => d23::run(),
        24 => d24::run(),
        25 => d25::run(read_file("input/d25.txt")),
        _ => panic!("invalid input"),
    };

    println!("{}, {}", res1, res2);
}

fn read_file<T>(file_name: &str) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    std::fs::read_to_string(file_name)
        .expect("file not found")
        .lines()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn read_file_with<T>(file_name: &str, f: impl Fn(&str) -> T) -> Vec<T> {
    std::fs::read_to_string(file_name)
        .expect("file not found")
        .lines()
        .map(f)
        .collect()
}

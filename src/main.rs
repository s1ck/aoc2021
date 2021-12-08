#![allow(dead_code)]

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

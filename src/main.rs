use std::fmt::Debug;
use std::str::FromStr;

mod d01;
mod d02;

fn main() {
    let day = std::env::args()
        .nth(1)
        .map(|d| d.parse::<u32>().unwrap())
        .unwrap();

    let (res1, res2) = match day {
        1 => d01::run(&read_file("input/d01.txt")),
        2 => d02::run(&read_file("input/d02.txt")),
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
        .map(|x| x.parse::<T>().unwrap())
        .collect()
}

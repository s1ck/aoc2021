use std::fmt::Debug;
use std::str::FromStr;

mod d01;
mod d02;
mod d03;
mod d04;

fn main() {
    let day = std::env::args()
        .nth(1)
        .map(|d| d.parse::<u32>().unwrap())
        .unwrap();

    let (res1, res2) = match day {
        1 => d01::run(&read_file("input/d01.txt")),
        2 => d02::run(&read_file("input/d02.txt")),
        3 => d03::run(
            read_file_raw("input/d03.txt")
                .iter()
                .map(|s| s.as_str())
                .collect::<Vec<_>>()
                .as_slice(),
        ),
        4 => d04::run(
            read_file_raw("input/d04.txt")
                .iter()
                .map(|s| s.as_str())
                .collect::<Vec<_>>()
                .as_slice(),
        ),
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

fn read_file_raw(file_name: &str) -> Vec<String> {
    std::fs::read_to_string(file_name)
        .expect("file not found")
        .lines()
        .map(String::from)
        .collect::<Vec<_>>()
}

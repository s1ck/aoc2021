use std::str::FromStr;
use Direction::*;

pub enum Direction {
    Forward,
    Down,
    Up,
}

pub struct Input(Direction, u32);

pub fn run(ins: &[Input]) -> (usize, usize) {
    (part1(ins) as usize, part2(ins) as usize)
}

fn part1(ins: &[Input]) -> u32 {
    let (x, y) = ins
        .iter()
        .fold((0, 0), |(x, y), Input(dir, dist)| match *dir {
            Forward => (x + dist, y),
            Down => (x, y + dist),
            Up => (x, y - dist),
        });
    x * y
}

fn part2(ins: &[Input]) -> u32 {
    let (x, y, _) = ins
        .iter()
        .fold((0, 0, 0), |(x, y, aim), Input(dir, dist)| match *dir {
            Forward => (x + dist, y + (dist * aim), aim),
            Down => (x, y, aim + dist),
            Up => (x, y, aim - dist),
        });

    x * y
}

impl FromStr for Input {
    type Err = std::io::Error;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let input = line
            .split_once(' ')
            .map(|(dir, dist)| {
                let dist = dist.parse::<u32>().unwrap();

                let dir = match dir {
                    "forward" => Direction::Forward,
                    "down" => Direction::Down,
                    "up" => Direction::Up,
                    _ => unreachable!(),
                };

                Input(dir, dist)
            })
            .unwrap();

        Ok(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let ins = [
            Input(Forward, 5),
            Input(Down, 5),
            Input(Forward, 8),
            Input(Up, 3),
            Input(Down, 8),
            Input(Forward, 2),
        ];

        let res = part1(&ins);

        assert_eq!(150, res);
    }

    #[test]
    fn test_part2() {
        let ins = [
            Input(Forward, 5),
            Input(Down, 5),
            Input(Forward, 8),
            Input(Up, 3),
            Input(Down, 8),
            Input(Forward, 2),
        ];

        let res = part2(&ins);

        assert_eq!(900, res);
    }
}

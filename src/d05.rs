use std::{collections::HashMap, str::FromStr};

pub struct Point {
    x: i32,
    y: i32,
}

impl FromStr for Point {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.split_once(',')
            .map(|(x, y)| Self {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
            })
            .unwrap())
    }
}

pub struct Line {
    start: Point,
    end: Point,
}

impl FromStr for Line {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.split_once(" -> ")
            .map(|(p1, p2)| Self {
                start: p1.parse().unwrap(),
                end: p2.parse().unwrap(),
            })
            .unwrap())
    }
}

impl Line {
    fn points(&self) -> Vec<(i32, i32)> {
        let dist_x = (self.start.x - self.end.x).abs();
        let dist_y = (self.start.y - self.end.y).abs();

        let dir_x = self.end.x.cmp(&self.start.x) as i32;
        let dir_y = self.end.y.cmp(&self.start.y) as i32;

        let step_x = dist_x.cmp(&0) as i32 * dir_x;
        let step_y = dist_y.cmp(&0) as i32 * dir_y;

        let length = dist_x.max(dist_y) + 1;

        (0..length).fold(vec![], |mut points, i| {
            points.push((self.start.x + (i * step_x), self.start.y + (i * step_y)));
            points
        })
    }

    fn is_straight(&self) -> bool {
        self.start.x == self.end.x || self.start.y == self.end.y
    }
}

pub fn run(lines: &[Line]) -> (usize, usize) {
    (part1(lines) as usize, part2(lines) as usize)
}

fn part1(lines: &[Line]) -> u32 {
    lines
        .iter()
        .filter(|line| line.is_straight())
        .fold(HashMap::new(), |mut points, line| {
            line.points()
                .iter()
                .for_each(|p| *points.entry(*p).or_insert(0) += 1);
            points
        })
        .values()
        .filter(|c| **c > 1)
        .count() as u32
}

fn part2(lines: &[Line]) -> u32 {
    lines
        .iter()
        .fold(HashMap::new(), |mut points, line| {
            line.points()
                .iter()
                .for_each(|p| *points.entry(*p).or_insert(0) += 1);
            points
        })
        .values()
        .filter(|c| **c > 1)
        .count() as u32
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &'static str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn test_part1() {
        let input = INPUT
            .split("\n")
            .map(|line| line.parse::<Line>().unwrap())
            .collect::<Vec<_>>();

        assert_eq!(5, part1(&input));
    }

    #[test]
    fn test_part2() {
        let input = INPUT
            .split("\n")
            .map(|line| line.parse::<Line>().unwrap())
            .collect::<Vec<_>>();

        assert_eq!(12, part2(&input));
    }
}

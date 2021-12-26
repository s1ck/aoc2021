use std::{
    ops::{Deref, DerefMut},
    str::FromStr,
};

#[derive(Debug, PartialEq, Eq)]
pub enum Field {
    East,
    South,
    Empty,
}

impl From<char> for Field {
    fn from(c: char) -> Self {
        match c {
            '>' => Self::East,
            'v' => Self::South,
            '.' => Self::Empty,
            _ => unreachable!(),
        }
    }
}

impl std::fmt::Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::East => ">",
                Self::South => "v",
                Self::Empty => ".",
            }
        )
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Line(Vec<Field>);

impl FromStr for Line {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Line(s.trim().chars().map(Field::from).collect::<Vec<_>>()))
    }
}

impl Deref for Line {
    type Target = Vec<Field>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Line {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub fn run(lines: Vec<Line>) -> (usize, usize) {
    (part1(lines), 0)
}

fn part1(mut lines: Vec<Line>) -> usize {
    let mut steps = 1;

    loop {
        if simulate(&mut lines) {
            return steps;
        }

        steps += 1;
    }
}

fn simulate(lines: &mut [Line]) -> bool {
    let mut done = true;

    let rows = lines.len();
    let cols = lines[0].len();

    // Move east cucumbers
    for row in lines.iter_mut() {
        let move_last = row[0] == Field::Empty && row[cols - 1] == Field::East;

        let mut col = 0;
        while col < cols - 1 {
            if row[col] == Field::East && row[col + 1] == Field::Empty {
                row[col] = Field::Empty;
                row[col + 1] = Field::East;
                done = false;
                col += 1;
            }
            col += 1;
        }

        if move_last {
            row[cols - 1] = Field::Empty;
            row[0] = Field::East;
            done = false;
        }
    }

    // Move south cucumbers
    for col in 0..cols {
        let move_last = lines[0][col] == Field::Empty && lines[rows - 1][col] == Field::South;

        let mut row = 0;
        while row < rows - 1 {
            if lines[row][col] == Field::South && lines[row + 1][col] == Field::Empty {
                lines[row][col] = Field::Empty;
                lines[row + 1][col] = Field::South;
                done = false;
                row += 1;
            }
            row += 1;
        }

        if move_last {
            lines[rows - 1][col] = Field::Empty;
            lines[0][col] = Field::South;
            done = false;
        }
    }

    done
}

#[cfg(test)]
mod tests {
    use super::Field::*;
    use super::*;

    const INPUT: &str = r#"v...>>.vv>
                           .vv>>.vv..
                           >>.>v>...v
                           >>v>>.>.v.
                           v>v.vv.v..
                           >.>>..v...
                           .vv..>.>v.
                           v.v..>>v.v
                           ....v..v.>"#;

    fn parse(input: &str) -> Vec<Line> {
        input
            .split('\n')
            .map(|line| line.trim().parse::<Line>().unwrap())
            .collect::<Vec<_>>()
    }

    #[test]
    fn test_parse() {
        assert_eq!(
            parse(INPUT)[0],
            Line(vec![
                South, Empty, Empty, Empty, East, East, Empty, South, South, East
            ])
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(parse(INPUT)), 58);
    }
}

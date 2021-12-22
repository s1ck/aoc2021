use std::{collections::HashSet, fmt::Display, num::ParseIntError, str::FromStr};

pub fn run(cuboids: Vec<Cuboid>) -> (usize, usize) {
    (part1(&cuboids), 0)
}

fn part1(cuboids: &[Cuboid]) -> usize {
    let mut coords = HashSet::<(i32, i32, i32)>::new();

    cuboids
        .iter()
        .filter(|c| c.is_within(-50, 50))
        .for_each(|c| {
            for x in c.x.from..=c.x.to {
                for y in c.y.from..=c.y.to {
                    for z in c.z.from..=c.z.to {
                        if c.state {
                            coords.insert((x, y, z));
                        } else {
                            coords.remove(&(x, y, z));
                        }
                    }
                }
            }
        });

    coords.len()
}

#[derive(Debug)]
pub struct Range {
    from: i32,
    to: i32,
}

impl FromStr for Range {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.split_once('=').unwrap().1;
        let (from, to) = s.split_once("..").unwrap();
        Ok(Self {
            from: from.parse::<i32>()?,
            to: to.parse::<i32>()?,
        })
    }
}

impl Display for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}..{}", self.from, self.to)
    }
}

impl Range {
    fn is_within(&self, from: i32, to: i32) -> bool {
        self.from >= from && self.to <= to
    }
}

#[derive(Debug)]
pub struct Cuboid {
    state: bool,
    x: Range,
    y: Range,
    z: Range,
}

impl FromStr for Cuboid {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (state, ranges) = s.trim().split_once(' ').unwrap();
        let mut ranges = ranges.split(",");
        let x = ranges.next().unwrap().parse::<Range>()?;
        let y = ranges.next().unwrap().parse::<Range>()?;
        let z = ranges.next().unwrap().parse::<Range>()?;

        Ok(Self {
            state: state == "on",
            x,
            y,
            z,
        })
    }
}

impl Display for Cuboid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} x={},y={},z={}", self.state, self.x, self.y, self.z)
    }
}

impl Cuboid {
    fn is_within(&self, from: i32, to: i32) -> bool {
        self.x.is_within(from, to) && self.y.is_within(from, to) && self.z.is_within(from, to)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMALL_INPUT: &str = r#"on x=10..12,y=10..12,z=10..12
                                 on x=11..13,y=11..13,z=11..13
                                 off x=9..11,y=9..11,z=9..11
                                 on x=10..10,y=10..10,z=10..10"#;

    const INPUT: &str = r#"on x=-20..26,y=-36..17,z=-47..7
                           on x=-20..33,y=-21..23,z=-26..28
                           on x=-22..28,y=-29..23,z=-38..16
                           on x=-46..7,y=-6..46,z=-50..-1
                           on x=-49..1,y=-3..46,z=-24..28
                           on x=2..47,y=-22..22,z=-23..27
                           on x=-27..23,y=-28..26,z=-21..29
                           on x=-39..5,y=-6..47,z=-3..44
                           on x=-30..21,y=-8..43,z=-13..34
                           on x=-22..26,y=-27..20,z=-29..19
                           off x=-48..-32,y=26..41,z=-47..-37
                           on x=-12..35,y=6..50,z=-50..-2
                           off x=-48..-32,y=-32..-16,z=-15..-5
                           on x=-18..26,y=-33..15,z=-7..46
                           off x=-40..-22,y=-38..-28,z=23..41
                           on x=-16..35,y=-41..10,z=-47..6
                           off x=-32..-23,y=11..30,z=-14..3
                           on x=-49..-5,y=-3..45,z=-29..18
                           off x=18..30,y=-20..-8,z=-3..13
                           on x=-41..9,y=-7..43,z=-33..15
                           on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
                           on x=967..23432,y=45373..81175,z=27513..53682"#;

    fn parse(input: &str) -> Vec<Cuboid> {
        input
            .split('\n')
            .map(|line| line.trim().parse::<Cuboid>().unwrap())
            .collect::<Vec<_>>()
    }

    #[test]
    fn test_parse() {
        parse(SMALL_INPUT).iter().for_each(|c| println!("{}", c));
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(SMALL_INPUT)), 39);
        assert_eq!(part1(&parse(INPUT)), 590784);
    }
}

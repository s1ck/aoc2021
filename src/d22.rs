use std::{collections::HashSet, fmt::Display, num::ParseIntError, ops::Sub, str::FromStr};

pub fn run(cuboids: Vec<Cuboid>) -> (usize, usize) {
    (part1(&cuboids), part2(&cuboids))
}

fn part1(cuboids: &[Cuboid]) -> usize {
    let mut coords = HashSet::<(i32, i32, i32)>::new();

    cuboids
        .iter()
        .filter(|c| c.is_within(Range::new(-50, 50)))
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

fn part2(cuboids: &[Cuboid]) -> usize {
    let mut clippings: Vec<Cuboid> = Vec::with_capacity(cuboids.len());

    cuboids.iter().for_each(|c| {
        if c.state {
            let mut local_clippings = vec![*c];

            clippings.iter().for_each(|clipping| {
                local_clippings = local_clippings
                    .iter()
                    .flat_map(|local_clipping| *local_clipping - *clipping)
                    .collect::<Vec<_>>();
            });

            clippings.extend(local_clippings);
        } else {
            clippings = clippings
                .iter()
                .flat_map(|clipping| *clipping - *c)
                .collect::<Vec<_>>();
        }
    });

    clippings
        .iter()
        .map(|clipping| clipping.volume() as usize)
        .sum()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Range {
    from: i32,
    to: i32,
}

impl From<(i32, i32)> for Range {
    fn from((from, to): (i32, i32)) -> Self {
        Self { from, to }
    }
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
    fn new(from: i32, to: i32) -> Self {
        Self { from, to }
    }

    fn len(&self) -> usize {
        self.to.abs_diff(self.from) as usize + 1
    }
}

impl std::ops::BitAnd for Range {
    type Output = Option<Self>;

    fn bitand(self, rhs: Self) -> Self::Output {
        if self.to < rhs.from || self.from > rhs.to {
            return None;
        }

        Some(Self {
            from: self.from.max(rhs.from),
            to: self.to.min(rhs.to),
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
        let mut ranges = ranges.split(',');
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
    fn new<T: Into<Range>>(state: bool, x: T, y: T, z: T) -> Self {
        Self {
            state,
            x: x.into(),
            y: y.into(),
            z: z.into(),
        }
    }

    fn is_within(&self, range: Range) -> bool {
        (self.x & range).is_some() && (self.y & range).is_some() && (self.z & range).is_some()
    }

    fn volume(&self) -> usize {
        self.x.len() * self.y.len() * self.z.len()
    }
}

impl Sub for Cuboid {
    type Output = Vec<Self>;

    fn sub(self, rhs: Self) -> Self::Output {
        let x_intersect = self.x & rhs.x;
        let y_intersect = self.y & rhs.y;
        let z_intersect = self.z & rhs.z;

        // println!("x_intersect = {:?}", x_intersect);
        // println!("y_intersect = {:?}", y_intersect);
        // println!("z_intersect = {:?}", z_intersect);

        match (x_intersect, y_intersect, z_intersect) {
            (Some(x_intersect), Some(y_intersect), Some(z_intersect)) => {
                let mut clippings = vec![];

                if self.x.from < x_intersect.from {
                    clippings.push(Cuboid::new(
                        self.state,
                        Range::new(self.x.from, x_intersect.from - 1),
                        self.y,
                        self.z,
                    ));
                }
                if self.x.to > x_intersect.to {
                    clippings.push(Cuboid::new(
                        self.state,
                        Range::new(x_intersect.to + 1, self.x.to),
                        self.y,
                        self.z,
                    ));
                }
                if self.y.from < y_intersect.from {
                    clippings.push(Cuboid::new(
                        self.state,
                        x_intersect,
                        Range::new(self.y.from, y_intersect.from - 1),
                        self.z,
                    ));
                }
                if self.y.to > y_intersect.to {
                    clippings.push(Cuboid::new(
                        self.state,
                        x_intersect,
                        Range::new(y_intersect.to + 1, self.y.to),
                        self.z,
                    ));
                }
                if self.z.from < z_intersect.from {
                    clippings.push(Cuboid::new(
                        self.state,
                        x_intersect,
                        y_intersect,
                        Range::new(self.z.from, z_intersect.from - 1),
                    ));
                }
                if self.z.to > z_intersect.to {
                    clippings.push(Cuboid::new(
                        self.state,
                        x_intersect,
                        y_intersect,
                        Range::new(z_intersect.to + 1, self.z.to),
                    ));
                }

                clippings
            }
            _ => vec![self],
        }
    }
}

#[cfg(test)]
mod tests {
    use test::Bencher;

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

    #[test]
    fn test_part2_sample() {
        assert_eq!(part2(&parse(SMALL_INPUT)), 39);
    }

    // #[test]
    fn test_part2_input() {
        let cuboids = parse(
            std::fs::read_to_string("input/d22-test.txt")
                .expect("file not found")
                .as_str(),
        );

        assert_eq!(part2(&cuboids), 2758514936282235);
    }

    #[test]
    fn test_range_len() {
        assert_eq!(Range::new(10, 15).len(), 6);
    }

    #[test]
    fn test_range_intersection() {
        let range0 = Range::new(10, 15);
        let range1 = Range::new(12, 17);
        assert_eq!((range0 & range1).unwrap(), Range::new(12, 15));

        let range0 = Range::new(10, 15);
        let range1 = Range::new(15, 17);
        assert_eq!((range0 & range1).unwrap(), Range::new(15, 15));

        let range0 = Range::new(10, 15);
        let range1 = Range::new(16, 17);
        assert!((range0 & range1).is_none());
    }

    #[test]
    fn test_cuboid_volume() {
        let cuboid0 = Cuboid::new(true, (10, 15), (12, 16), (2, 4));
        assert_eq!(cuboid0.volume(), 90);
    }

    #[test]
    fn test_cuboid_subtraction_overlap() {
        let cuboid0 = Cuboid::new(true, (10, 12), (10, 12), (10, 12));
        let cuboid1 = Cuboid::new(true, (11, 13), (11, 13), (11, 13));

        let actual = cuboid1 - cuboid0;

        actual.iter().for_each(|c| println!("{}", c));

        assert_eq!(
            actual,
            vec![
                Cuboid::new(true, (13, 13), (11, 13), (11, 13)),
                Cuboid::new(true, (11, 12), (13, 13), (11, 13)),
                Cuboid::new(true, (11, 12), (11, 12), (13, 13)),
            ]
        );
    }

    #[test]
    fn test_cuboid_subtraction_contains() {
        let cuboid0 = Cuboid::new(true, (10, 15), (12, 16), (2, 4));
        let cuboid1 = Cuboid::new(true, (5, 20), (5, 20), (0, 10));

        assert_eq!(cuboid0 - cuboid1, vec![]);
    }

    #[bench]
    fn bench_part1(bencher: &mut Bencher) {
        let cuboids = parse(
            std::fs::read_to_string("input/d22.txt")
                .expect("file not found")
                .as_str(),
        );

        bencher.iter(|| assert_eq!(part1(&cuboids), 568000));
    }

    #[bench]
    fn bench_part2(bencher: &mut Bencher) {
        let cuboids = parse(
            std::fs::read_to_string("input/d22.txt")
                .expect("file not found")
                .as_str(),
        );

        bencher.iter(|| assert_eq!(part2(&cuboids), 1177411289280259));
    }
}

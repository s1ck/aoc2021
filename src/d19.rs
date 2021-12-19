use std::{
    collections::{HashMap, HashSet},
    ops::{Add, Sub},
    str::FromStr,
};

use graph::prelude::*;

pub fn run(input: &str) -> (usize, usize) {
    let mut scanners = input
        .split("\n\n")
        .map(|scanner| scanner.parse::<Scanner>().unwrap())
        .collect::<Vec<_>>();

    (part1(&mut scanners), 0)
}

fn part1(scanners: &mut [Scanner]) -> usize {
    let mut rotations = HashMap::new();

    for id_left in 0..scanners.len() {
        for id_right in (id_left + 1)..scanners.len() {
            let matches = scanners[id_left].overlap(&scanners[id_right]);

            if !matches.is_empty() {
                let (diff_left, diff_right) = matches
                    .array_windows()
                    .map(|[(l0, r0), (l1, r1)]| (*l0 - *l1, *r0 - *r1))
                    .find(|(l_diff, _)| l_diff.x != l_diff.y && l_diff.y != l_diff.z)
                    .unwrap();

                let (rotation, sign) = diff_right.mapping(&diff_left);

                let l = matches[0].0;
                let r = matches[0].1.rotate(rotation, sign);

                let center = Point::center(&l, &r);

                rotations.insert((id_left, id_right), (center, rotation, sign));

                let (rotation, sign) = diff_left.mapping(&diff_right);

                let l = matches[0].1;
                let r = matches[0].0.rotate(rotation, sign);

                let center = Point::center(&l, &r);

                rotations.insert((id_right, id_left), (center, rotation, sign));
            }
        }
    }

    // find transformation paths and transform according to rotations

    let g: UndirectedCsrGraph<usize> = GraphBuilder::new()
        .edges(rotations.keys().copied().collect::<Vec<_>>())
        .build();

    let mut beacons = scanners[0].points.iter().copied().collect::<HashSet<_>>();

    for scanner_id in 1..scanners.len() {
        let path = dfs(&g, scanner_id);

        for (from, to) in path.iter().rev() {
            let (center, rotation, sign) = rotations.get(&(*from, *to)).unwrap();

            scanners[scanner_id].points.iter_mut().for_each(|point| {
                let rotated = point.rotate(*rotation, *sign);
                let translated = *center + rotated;

                *point = translated;
            });
        }

        scanners[scanner_id].points.iter().for_each(|p| {
            beacons.insert(*p);
        });
    }

    return beacons.len();

    fn dfs(g: &UndirectedCsrGraph<usize>, end: usize) -> Vec<(usize, usize)> {
        fn dfs_inner(
            g: &UndirectedCsrGraph<usize>,
            current: usize,
            end: usize,
            path: &mut Vec<(usize, usize)>,
            visited: &mut HashSet<usize>,
        ) -> bool {
            if current == end {
                return true;
            }
            for neighbor in g.neighbors(current) {
                if visited.contains(neighbor) {
                    continue;
                }
                visited.insert(*neighbor);
                path.push((current, *neighbor));
                if !dfs_inner(g, *neighbor, end, path, visited) {
                    path.pop();
                } else {
                    return true;
                }
            }

            return false;
        }

        let mut path = vec![];
        let mut visited = HashSet::new();

        visited.insert(0);

        dfs_inner(g, 0, end, &mut path, &mut visited);

        path
    }
}

#[derive(PartialEq, PartialOrd, Clone, Copy, Default, Eq, Hash, Ord)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{},{}", self.x, self.y, self.z)
    }
}

impl FromStr for Point {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, s) = s.split_once(',').unwrap();
        let (y, z) = s.split_once(',').unwrap();

        Ok(Self {
            x: x.parse::<i32>().unwrap(),
            y: y.parse::<i32>().unwrap(),
            z: z.parse::<i32>().unwrap(),
        })
    }
}

impl Point {
    fn mapping(&self, other: &Self) -> ([usize; 3], [i8; 3]) {
        let mut rotation = [0; 3];
        let mut sign = [1; 3];

        let (r0, s0) = if self.x.abs() == other.x.abs() {
            (0, if self.x == other.x { 1 } else { -1 })
        } else if self.x.abs() == other.y.abs() {
            (1, if self.x == other.y { 1 } else { -1 })
        } else {
            (2, if self.x == other.z { 1 } else { -1 })
        };
        rotation[0] = r0;
        sign[0] = s0;

        let (r1, s1) = if self.y.abs() == other.x.abs() {
            (0, if self.y == other.x { 1 } else { -1 })
        } else if self.y.abs() == other.y.abs() {
            (1, if self.y == other.y { 1 } else { -1 })
        } else {
            (2, if self.y == other.z { 1 } else { -1 })
        };

        rotation[1] = r1;
        sign[1] = s1;

        let (r2, s2) = if self.z.abs() == other.x.abs() {
            (0, if self.z == other.x { 1 } else { -1 })
        } else if self.z.abs() == other.y.abs() {
            (1, if self.z == other.y { 1 } else { -1 })
        } else {
            (2, if self.z == other.z { 1 } else { -1 })
        };

        rotation[2] = r2;
        sign[2] = s2;

        (rotation, sign)
    }

    fn rotate(&self, rotation: [usize; 3], sign: [i8; 3]) -> Self {
        let mut res = Self { x: 0, y: 0, z: 0 };

        let target = match rotation[0] {
            0 => &mut res.x,
            1 => &mut res.y,
            2 => &mut res.z,
            _ => unreachable!(),
        };

        *target = self.x * sign[0] as i32;

        let target = match rotation[1] {
            0 => &mut res.x,
            1 => &mut res.y,
            2 => &mut res.z,
            _ => unreachable!(),
        };

        *target = self.y * sign[1] as i32;

        let target = match rotation[2] {
            0 => &mut res.x,
            1 => &mut res.y,
            2 => &mut res.z,
            _ => unreachable!(),
        };

        *target = self.z * sign[2] as i32;

        res
    }

    fn center(left: &Self, right: &Self) -> Self {
        let c_x = if right.x > 0 {
            left.x - right.x.abs()
        } else {
            left.x + right.x.abs()
        };

        let c_y = if right.y > 0 {
            left.y - right.y.abs()
        } else {
            left.y + right.y.abs()
        };

        let c_z = if right.z > 0 {
            left.z - right.z.abs()
        } else {
            left.z + right.z.abs()
        };

        let center = Point {
            x: c_x,
            y: c_y,
            z: c_z,
        };

        center
    }
}

struct Scanner {
    id: u32,
    points: Vec<Point>,
}

impl std::fmt::Display for Scanner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Scanner: {}", self.id)?;
        self.points.iter().try_for_each(|p| writeln!(f, "{}", p))?;
        Ok(())
    }
}

impl FromStr for Scanner {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().map(|line| line.trim()).collect::<Vec<_>>();
        let id = lines[0].split(' ').nth(2).unwrap().parse::<u32>().unwrap();
        let points = lines
            .iter()
            .skip(1)
            .map(|line| line.parse::<Point>().unwrap())
            .collect::<Vec<_>>();

        Ok(Self { id, points })
    }
}

impl Scanner {
    fn distances(&self) -> Vec<HashSet<[u32; 3]>> {
        self.points
            .iter()
            .map(|p| {
                self.points
                    .iter()
                    .filter(|other| *p != **other)
                    .map(|other| {
                        let mut d = [0; 3];

                        d[0] = p.x.abs_diff(other.x);
                        d[1] = p.y.abs_diff(other.y);
                        d[2] = p.z.abs_diff(other.z);

                        d.sort();
                        d
                    })
                    .collect::<HashSet<_>>()
            })
            .collect::<Vec<_>>()
    }

    fn overlap(&self, other: &Self) -> Vec<(Point, Point)> {
        let d_self = self.distances();
        let d_other = other.distances();

        let matches = d_self
            .iter()
            .enumerate()
            .flat_map(|(i_self, p_self)| {
                d_other
                    .iter()
                    .enumerate()
                    .filter_map(|(i_other, p_other)| {
                        if p_self.intersection(p_other).collect::<Vec<_>>().len() == 11 {
                            Some((self.points[i_self], other.points[i_other]))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        matches
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_sample() {
        let f = std::fs::read_to_string("input/d19-test.txt").expect("file not found");

        let mut scanners = f
            .split("\n\n")
            .map(|scanner| scanner.parse::<Scanner>().unwrap())
            .collect::<Vec<_>>();

        assert_eq!(part1(&mut scanners), 79);
    }

    #[test]
    fn test_part1() {
        let f = std::fs::read_to_string("input/d19.txt").expect("file not found");

        let mut scanners = f
            .split("\n\n")
            .map(|scanner| scanner.parse::<Scanner>().unwrap())
            .collect::<Vec<_>>();

        assert_eq!(part1(&mut scanners), 398);
    }

    #[test]
    fn test_distances() {
        let distances = "--- scanner 0 ---
                         42,84,42
                         50,90,60
                         55,-10,-42"
            .parse::<Scanner>()
            .unwrap()
            .distances();

        distances.iter().for_each(|d| println!("{:?}", d));

        let mut expected = vec![];

        let mut d0 = HashSet::new();
        d0.insert([6, 8, 18]);
        d0.insert([13, 84, 94]);

        expected.push(d0);

        let mut d0 = HashSet::new();
        d0.insert([5, 100, 102]);
        d0.insert([6, 8, 18]);

        expected.push(d0);

        let mut d0 = HashSet::new();
        d0.insert([5, 100, 102]);
        d0.insert([13, 84, 94]);

        expected.push(d0);

        assert_eq!(distances, expected);
    }

    #[test]
    fn test_overlap() {
        let scanners = INPUT
            .split("\n\n")
            .map(|s| s.parse::<Scanner>().unwrap())
            .collect::<Vec<_>>();

        let overlap = scanners[0].overlap(&scanners[1]);
        assert_eq!(overlap.len(), 12);
    }

    const INPUT: &str = r#"--- scanner 0 ---
                           404,-588,-901
                           528,-643,409
                           -838,591,734
                           390,-675,-793
                           -537,-823,-458
                           -485,-357,347
                           -345,-311,381
                           -661,-816,-575
                           -876,649,763
                           -618,-824,-621
                           553,345,-567
                           474,580,667
                           -447,-329,318
                           -584,868,-557
                           544,-627,-890
                           564,392,-477
                           455,729,728
                           -892,524,684
                           -689,845,-530
                           423,-701,434
                           7,-33,-71
                           630,319,-379
                           443,580,662
                           -789,900,-551
                           459,-707,401

                           --- scanner 1 ---
                           686,422,578
                           605,423,415
                           515,917,-361
                           -336,658,858
                           95,138,22
                           -476,619,847
                           -340,-569,-846
                           567,-361,727
                           -460,603,-452
                           669,-402,600
                           729,430,532
                           -500,-761,534
                           -322,571,750
                           -466,-666,-811
                           -429,-592,574
                           -355,545,-477
                           703,-491,-529
                           -328,-685,520
                           413,935,-424
                           -391,539,-444
                           586,-435,557
                           -364,-763,-893
                           807,-499,-711
                           755,-354,-619
                           553,889,-390"#;
}

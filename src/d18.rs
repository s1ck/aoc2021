use std::str::FromStr;

pub fn run(trees: Vec<Tree>) -> (usize, usize) {
    (part1(trees.clone()), part2(trees))
}

fn part1(trees: Vec<Tree>) -> usize {
    trees
        .into_iter()
        .reduce(|t1, t2| t1.reduce(t2))
        .unwrap()
        .magnitude() as usize
}

fn part2(trees: Vec<Tree>) -> usize {
    trees
        .iter()
        .map(|left| {
            trees
                .iter()
                .map(|right| left.clone().reduce(right.clone()).magnitude())
                .max()
                .unwrap()
        })
        .max()
        .unwrap() as usize
}

#[derive(Clone, Debug, PartialEq)]
pub enum Tree {
    Reg(u8),
    Pair(Box<(Tree, Tree)>),
}

#[derive(Clone, Copy)]
enum Direction {
    Left,
    Right,
}

impl std::fmt::Display for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Reg(v) => write!(f, "{}", v),
            Self::Pair(pair) => write!(f, "[{},{}]", pair.0, pair.1),
        }
    }
}

impl Tree {
    fn of(val: impl Into<Self>) -> Self {
        val.into()
    }

    fn from(s: &str) -> Self {
        s.parse::<Self>().unwrap()
    }

    fn magnitude(&self) -> u32 {
        match self {
            Self::Reg(v) => *v as u32,
            Self::Pair(pair) => 3 * pair.0.magnitude() + 2 * pair.1.magnitude(),
        }
    }

    fn reduce(self, other: Tree) -> Self {
        let mut tree = Self::Pair(Box::new((self, other)));

        loop {
            let (mut next, explode) = tree.explode(0);

            if explode.is_some() {
                tree = next;
                continue;
            }

            if !next.split() {
                return next;
            }

            tree = next;
        }
    }

    fn insert(&self, value: u8, d: Direction) -> Self {
        match (self, d) {
            (Tree::Reg(v), _) => Tree::Reg(v + value),
            (Tree::Pair(pair), Direction::Left) => {
                Tree::Pair(Box::new((pair.0.insert(value, d), pair.1.clone())))
            }
            (Tree::Pair(pair), Direction::Right) => {
                Tree::Pair(Box::new((pair.0.clone(), pair.1.insert(value, d))))
            }
        }
    }

    fn explode(&self, depth: u8) -> (Self, Option<(Option<u8>, Option<u8>)>) {
        match self {
            Tree::Pair(pair) if depth == 4 => match (&pair.0, &pair.1) {
                (&Tree::Reg(l), &Tree::Reg(r)) => (Tree::Reg(0), Some((Some(l), Some(r)))),
                _ => unreachable!(),
            },
            Tree::Pair(pair) => {
                let (new_lhs, exploded) = pair.0.explode(depth + 1);

                if let Some((left, right)) = exploded {
                    if let Some(r) = right {
                        return (
                            Tree::Pair(Box::new((new_lhs, pair.1.insert(r, Direction::Left)))),
                            Some((left, None)),
                        );
                    }
                    return (
                        Tree::Pair(Box::new((new_lhs, pair.1.clone()))),
                        Some((left, None)),
                    );
                } else {
                    let (new_rhs, exploded) = pair.1.explode(depth + 1);

                    if let Some((left, right)) = exploded {
                        if let Some(l) = left {
                            return (
                                Tree::Pair(Box::new((pair.0.insert(l, Direction::Right), new_rhs))),
                                Some((None, right)),
                            );
                        }
                        return (
                            Tree::Pair(Box::new((pair.0.clone(), new_rhs))),
                            Some((None, right)),
                        );
                    }
                }

                (Tree::Pair(Box::new((pair.0.clone(), pair.1.clone()))), None)
            }
            reg => (reg.clone(), None),
        }
    }

    fn split(&mut self) -> bool {
        match self {
            Tree::Reg(v) if *v > 9 => {
                *self = Tree::Pair(Box::new((Tree::Reg(*v / 2), Tree::Reg((*v + 1) / 2))));
                true
            }
            Tree::Pair(pair) => {
                let lhs_split = pair.0.split();

                let rhs_split = if !lhs_split { pair.1.split() } else { false };

                lhs_split || rhs_split
            }
            _ => false,
        }
    }
}

impl From<u8> for Tree {
    fn from(v: u8) -> Self {
        Self::Reg(v)
    }
}

impl<T: Into<Self>, U: Into<Self>> From<(T, U)> for Tree {
    fn from((left, right): (T, U)) -> Self {
        Self::Pair(Box::new((left.into(), right.into())))
    }
}

impl FromStr for Tree {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (node, _) = parse(s.as_bytes());
        Ok(node)
    }
}

fn parse(bytes: &[u8]) -> (Tree, &[u8]) {
    match bytes[0] {
        b'[' => {
            let (lhs, rem) = parse(&bytes[1..]);
            let (rhs, rem) = parse(&rem[1..]); // skip comma
            (Tree::Pair(Box::new((lhs, rhs))), &rem[1..]) // skip ]
        }
        b'0'..=b'9' => (Tree::Reg(bytes[0] - b'0'), &bytes[1..]),
        c => panic!("unexpected: '{}'", c),
    }
}

#[cfg(test)]
mod tests {

    use test::Bencher;

    use super::*;

    const INPUT: &str = r#"[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
                           [[[5,[2,8]],4],[5,[[9,9],0]]]
                           [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
                           [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
                           [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
                           [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
                           [[[[5,4],[7,7]],8],[[8,3],8]]
                           [[9,3],[[9,9],[6,[4,9]]]]
                           [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
                           [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]"#;

    #[test]
    fn test_parse() {
        let actual = INPUT
            .split('\n')
            .map(|line| line.trim().parse::<Tree>().unwrap())
            .collect::<Vec<Tree>>();

        assert_eq!(
            actual,
            vec![
                Tree::of((((0, (5, 8)), ((1, 7), (9, 6))), ((4, (1, 2)), ((1, 4), 2)))),
                Tree::of((((5, (2, 8)), 4), (5, ((9, 9), 0)))),
                Tree::of((6, (((6, 2), (5, 6)), ((7, 6), (4, 7))))),
                Tree::of((((6, (0, 7)), (0, 9)), (4, (9, (9, 0))))),
                Tree::of((((7, (6, 4)), (3, (1, 3))), (((5, 5), 1), 9))),
                Tree::of(((6, ((7, 3), (3, 2))), (((3, 8), (5, 7)), 4))),
                Tree::of(((((5, 4), (7, 7)), 8), ((8, 3), 8))),
                Tree::of(((9, 3), ((9, 9), (6, (4, 9))))),
                Tree::of(((2, ((7, 7), 7)), ((5, 8), ((9, 3), (0, 2))))),
                Tree::of(((((5, 2), 5), (8, (3, 7))), ((5, (7, 5)), (4, 4)))),
            ]
        );
    }

    #[test]
    fn test_part1() {
        let trees = INPUT
            .split('\n')
            .map(|line| line.trim().parse::<Tree>().unwrap())
            .collect::<Vec<Tree>>();

        assert_eq!(part1(trees), 4140);
    }

    #[test]
    fn test_part2() {
        let trees = INPUT
            .split('\n')
            .map(|line| line.trim().parse::<Tree>().unwrap())
            .collect::<Vec<Tree>>();

        assert_eq!(part2(trees), 3993);
    }

    #[test]
    fn test_explode() {
        let input = vec![
            Tree::of((((((9, 8), 1), 2), 3), 4)),
            Tree::of((7, (6, (5, (4, (3, 2)))))),
            Tree::of(((6, (5, (4, (3, 2)))), 1)),
        ];

        let expected = vec![
            Tree::of(((((0, 9), 2), 3), 4)),
            Tree::of((7, (6, (5, (7, 0))))),
            Tree::of(((6, (5, (7, 0))), 3)),
        ];

        assert_eq!(
            input
                .iter()
                .map(|tree| tree.explode(0).0)
                .collect::<Vec<_>>(),
            expected
        );
    }

    #[test]
    fn test_split() {
        let actual = vec![
            Tree::of((0, 15)),
            Tree::of(((((0, 7), 4), (15, (0, 13))), (1, 1))),
            Tree::of(((((0, 7), 4), ((7, 8), (0, 13))), (1, 1))),
        ];

        let expected = vec![
            Tree::of((0, (7, 8))),
            Tree::of(((((0, 7), 4), ((7, 8), (0, 13))), (1, 1))),
            Tree::of(((((0, 7), 4), ((7, 8), (0, (6, 7)))), (1, 1))),
        ];

        assert_eq!(
            actual
                .into_iter()
                .map(|mut tree| {
                    tree.split();
                    tree
                })
                .collect::<Vec<_>>(),
            expected
        )
    }

    #[test]
    fn test_magnitude() {
        assert_eq!(Tree::of((9, 1)).magnitude(), 29);
        assert_eq!(Tree::of(((9, 1), (1, 9))).magnitude(), 129);
    }

    #[test]
    fn test_reduce() {
        let trees = vec![
            Tree::from("[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]"),
            Tree::from("[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]"),
            Tree::from("[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]"),
            Tree::from("[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]"),
            Tree::from("[7,[5,[[3,8],[1,4]]]]"),
            Tree::from("[[2,[2,2]],[8,[8,1]]]"),
            Tree::from("[2,9]"),
            Tree::from("[1,[[[9,3],9],[[9,0],[0,7]]]]"),
            Tree::from("[[[5,[7,4]],7],1]"),
            Tree::from("[[[[4,2],2],6],[8,7]]"),
        ];

        assert_eq!(
            trees.into_iter().reduce(|t1, t2| t1.reduce(t2)).unwrap(),
            Tree::from("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")
        );
    }

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        let input = std::fs::read_to_string("input/d18.txt")
            .expect("file not found")
            .lines()
            .map(|line| line.parse::<Tree>().unwrap())
            .collect::<Vec<_>>();

        b.iter(|| assert_eq!(part1(input.clone()), 3987));
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        let input = std::fs::read_to_string("input/d18.txt")
            .expect("file not found")
            .lines()
            .map(|line| line.parse::<Tree>().unwrap())
            .collect::<Vec<_>>();

        b.iter(|| assert_eq!(part2(input.clone()), 4500));
    }
}

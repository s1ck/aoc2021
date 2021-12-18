use std::str::FromStr;

pub fn run(trees: Vec<Tree>) -> (usize, usize) {
    (part1(trees.clone()), part2(trees))
}

fn part1(trees: Vec<Tree>) -> usize {
    let sum = trees[1..]
        .iter()
        .fold(trees[0].clone(), |t, next| t.add(next.clone()));

    sum.magnitude() as usize
}

fn part2(trees: Vec<Tree>) -> usize {
    trees
        .iter()
        .map(|left| {
            trees
                .iter()
                .map(|right| left.clone().add(right.clone()).magnitude())
                .max()
                .unwrap()
        })
        .max()
        .unwrap() as usize
}

#[derive(Clone, Debug, PartialEq)]
pub enum Tree {
    Reg(u8),
    Pair(Box<Tree>, Box<Tree>),
}

impl std::fmt::Display for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Reg(v) => write!(f, "{}", v),
            Self::Pair(lhs, rhs) => write!(f, "[{},{}]", lhs, rhs),
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

    fn magnitude(self) -> u32 {
        match self {
            Self::Reg(v) => v as u32,
            Self::Pair(lhs, rhs) => 3 * lhs.magnitude() + 2 * rhs.magnitude(),
        }
    }

    fn add(self, other: Tree) -> Self {
        let mut tree = Self::Pair(Box::new(self), Box::new(other));

        loop {
            let (next, _, did_explode) = Tree::explode_inner(&tree, 0);

            if did_explode {
                tree = next;
                continue;
            }

            let (next, did_split) = next.split_inner();

            if !did_split {
                return next;
            }

            tree = next;
        }
    }

    fn insert_left(&self, value: u8) -> Self {
        match self {
            Tree::Reg(v) => Tree::Reg(v + value),
            Tree::Pair(lhs, rhs) => {
                Tree::Pair(Box::new(lhs.insert_left(value)), Box::new(*rhs.clone()))
            }
        }
    }

    fn insert_right(&self, value: u8) -> Self {
        match self {
            Tree::Reg(v) => Tree::Reg(v + value),
            Tree::Pair(lhs, rhs) => {
                Tree::Pair(Box::new(*lhs.clone()), Box::new(rhs.insert_right(value)))
            }
        }
    }

    fn explode(&self) -> Self {
        return Tree::explode_inner(self, 0).0;
    }

    fn explode_inner(t: &Tree, depth: usize) -> (Tree, Option<(Option<u8>, Option<u8>)>, bool) {
        if depth < 4 {
            return match t {
                Tree::Pair(lhs, rhs) => {
                    let (new_lhs, exploded, did_explode) = Tree::explode_inner(lhs, depth + 1);

                    if let Some((left, right)) = exploded {
                        if let Some(r) = right {
                            return (
                                Tree::Pair(Box::new(new_lhs), Box::new(rhs.insert_left(r))),
                                Some((left, None)),
                                did_explode,
                            );
                        }
                        return (
                            Tree::Pair(Box::new(new_lhs), Box::new(*rhs.clone())),
                            Some((left, None)), // parent needs to handle lhs
                            did_explode,
                        );
                    } else {
                        let (new_rhs, exploded, did_explode) = Tree::explode_inner(rhs, depth + 1);

                        if let Some((left, right)) = exploded {
                            if let Some(l) = left {
                                return (
                                    Tree::Pair(Box::new(lhs.insert_right(l)), Box::new(new_rhs)),
                                    Some((None, right)),
                                    did_explode,
                                );
                            }
                            return (
                                Tree::Pair(Box::new(*lhs.clone()), Box::new(new_rhs)),
                                Some((None, right)),
                                did_explode,
                            );
                        }
                    }

                    (
                        Tree::Pair(Box::new(*lhs.clone()), Box::new(*rhs.clone())),
                        None,
                        false,
                    )
                }
                _ => (t.clone(), None, false),
            };
        } else {
            return match t {
                Tree::Pair(left, right) => {
                    let l = match **left {
                        Tree::Reg(l) => l,
                        _ => unreachable!(),
                    };

                    let r = match **right {
                        Tree::Reg(r) => r,
                        _ => unreachable!(),
                    };

                    (Tree::Reg(0), Some((Some(l), Some(r))), true)
                }
                reg => (reg.clone(), None, false),
            };
        }
    }

    fn split(&self) -> Self {
        self.split_inner().0
    }

    fn split_inner(&self) -> (Self, bool) {
        match self {
            Tree::Reg(v) if *v > 9 => (
                Tree::Pair(
                    Box::new(Tree::Reg(f32::floor(*v as f32 / 2.0) as u8)),
                    Box::new(Tree::Reg(f32::ceil(*v as f32 / 2.0) as u8)),
                ),
                true,
            ),
            Tree::Reg(_) => (self.clone(), false),
            Tree::Pair(lhs, rhs) => {
                let (lhs, did_lhs_split) = lhs.split_inner();

                let (rhs, did_rhs_split) = if !did_lhs_split {
                    rhs.split_inner()
                } else {
                    (*rhs.clone(), false)
                };

                (
                    Tree::Pair(Box::new(lhs), Box::new(rhs)),
                    did_lhs_split || did_rhs_split,
                )
            }
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
        Self::Pair(Box::new(left.into()), Box::new(right.into()))
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
            (Tree::Pair(Box::new(lhs), Box::new(rhs)), &rem[1..]) // skip ]
        }
        b'0'..=b'9' => (Tree::Reg(bytes[0] - b'0'), &bytes[1..]),
        c => panic!("unexpected: '{}'", c),
    }
}

#[cfg(test)]
mod tests {

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
    fn test_part2() {
        let trees = INPUT
            .split('\n')
            .map(|line| line.trim().parse::<Tree>().unwrap())
            .collect::<Vec<Tree>>();

        assert_eq!(part2(trees), 3993);
    }

    #[test]
    fn test_parse_1() {
        let tree = "[4,2]".parse::<Tree>().unwrap();
        assert_eq!(tree, Tree::of((4, 2)));
    }

    #[test]
    fn test_parse_2() {
        let tree = "[4,[8,4]]".parse::<Tree>().unwrap();
        assert_eq!(tree, Tree::of((4, (8, 4))));
    }

    #[test]
    fn test_parse_3() {
        let tree = "[[1,[2,3]],[4,5]]".parse::<Tree>().unwrap();
        assert_eq!(tree, Tree::of(((1, (2, 3)), (4, 5))))
    }

    #[test]
    fn test_parse_input() {
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
    fn test_add() {
        let lhs = "[[[[4,3],4],4],[7,[[8,4],9]]]".parse::<Tree>().unwrap();
        let rhs = "[1,1]".parse::<Tree>().unwrap();

        let sum = lhs.add(rhs);

        assert_eq!(
            sum,
            "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]".parse::<Tree>().unwrap()
        );
    }

    #[test]
    fn test_add_multiple() {
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

        let sum = trees[1..]
            .iter()
            .fold(trees[0].clone(), |t, next| t.add(next.clone()));
        assert_eq!(
            sum,
            Tree::from("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")
        );
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
            input.iter().map(|tree| tree.explode()).collect::<Vec<_>>(),
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
            actual.iter().map(|tree| tree.split()).collect::<Vec<_>>(),
            expected
        )
    }

    #[test]
    fn test_magnitude() {
        assert_eq!(Tree::of((9, 1)).magnitude(), 29);
        assert_eq!(Tree::of(((9, 1), (1, 9))).magnitude(), 129);
    }
}

use std::collections::{HashSet, VecDeque};

pub fn run(field: Vec<Vec<u8>>) -> (usize, usize) {
    let field = field.iter().map(|line| line.as_slice()).collect::<Vec<_>>();

    (part1(&field).0 as usize, part2(&field) as usize)
}

fn part1(field: &[&[u8]]) -> (u32, Vec<(usize, usize)>) {
    let h = field.len();
    let w = field[0].len();

    let mut sum = 0_u32;
    let mut depths = vec![];

    for i in 0..h {
        for j in 0..w {
            let c = field[i][j];
            let t = if i == 0 { c + 1 } else { field[i - 1][j] };
            let r = if j == w - 1 { c + 1 } else { field[i][j + 1] };
            let b = if i == h - 1 { c + 1 } else { field[i + 1][j] };
            let l = if j == 0 { c + 1 } else { field[i][j - 1] };

            if t > c && r > c && b > c && l > c {
                depths.push((i, j));
                sum += c as u32 + 1;
            }
        }
    }

    (sum, depths)
}

fn part2(field: &[&[u8]]) -> u32 {
    let (_, depths) = part1(field);
    let mut sizes = depths
        .iter()
        .map(|depth| bfs(field, *depth))
        .collect::<Vec<_>>();
    sizes.sort_by(|a, b| a.cmp(b).reverse());
    return sizes.iter().take(3).product();

    fn bfs(field: &[&[u8]], root: (usize, usize)) -> u32 {
        let mut size = 0;
        let mut queue = VecDeque::new();
        queue.push_back(root);

        let h = field.len();
        let w = field[0].len();

        let mut seen = HashSet::new();

        while let Some((i, j)) = queue.pop_front() {
            let c = field[i][j];
            if c == 9 {
                continue;
            }
            size += 1;

            let t = if i == 0 { c + 1 } else { field[i - 1][j] };
            let r = if j == w - 1 { c + 1 } else { field[i][j + 1] };
            let b = if i == h - 1 { c + 1 } else { field[i + 1][j] };
            let l = if j == 0 { c + 1 } else { field[i][j - 1] };

            if i > 0 && t > c {
                let p = (i - 1, j);
                if !seen.contains(&p) {
                    seen.insert(p);
                    queue.push_back(p);
                }
            }
            if j < w - 1 && r > c {
                let p = (i, j + 1);
                if !seen.contains(&p) {
                    seen.insert(p);
                    queue.push_back(p);
                }
            }
            if i < h - 1 && b > c {
                let p = (i + 1, j);
                if !seen.contains(&p) {
                    seen.insert(p);
                    queue.push_back(p);
                }
            }
            if j > 0 && l > c {
                let p = (i, j - 1);
                if !seen.contains(&p) {
                    seen.insert(p);
                    queue.push_back(p);
                }
            }
        }
        size
    }
}

pub fn parse(line: &str) -> Vec<u8> {
    line.trim().chars().map(|c| c as u8 - b'0').collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"2199943210
                           3987894921
                           9856789892
                           8767896789
                           9899965678"#;

    #[test]
    fn test_part1() {
        let input = INPUT
            .split('\n')
            .map(|line| parse(line))
            .collect::<Vec<_>>();

        assert_eq!(run(input).0, 15);
    }

    #[test]
    fn test_part2() {
        let input = INPUT
            .split('\n')
            .map(|line| parse(line))
            .map(|line| {
                println!("{:?}", line);
                line
            })
            .collect::<Vec<_>>();

        assert_eq!(run(input).1, 1134);
    }
}

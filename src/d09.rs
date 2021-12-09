use std::collections::HashSet;

pub fn run(field: Vec<Vec<u8>>) -> (usize, usize) {
    (part1(field.clone()).0 as usize, part2(field) as usize)
}

fn part1(field: Vec<Vec<u8>>) -> (u32, Vec<(usize, usize)>) {
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

fn part2(field: Vec<Vec<u8>>) -> u32 {
    fn bfs(field: &[Vec<u8>], root: (usize, usize)) -> u32 {
        let mut queue = vec![root];
        let mut size = 0;

        let h = field.len();
        let w = field[0].len();

        let mut seen = HashSet::new();

        while !queue.is_empty() {
            let (i, j) = queue.remove(0);
            size += 1;
            let c = field[i][j];

            let t = if i == 0 { c + 1 } else { field[i - 1][j] };
            let r = if j == w - 1 { c + 1 } else { field[i][j + 1] };
            let b = if i == h - 1 { c + 1 } else { field[i + 1][j] };
            let l = if j == 0 { c + 1 } else { field[i][j - 1] };

            if i > 0 && t > c && t != 9 {
                let p = (i - 1, j);
                if !seen.contains(&p) {
                    seen.insert(p);
                    queue.push(p);
                }
            }
            if j != w - 1 && r > c && r != 9 {
                let p = (i, j + 1);
                if !seen.contains(&p) {
                    seen.insert(p);
                    queue.push(p);
                }
            }
            if i != h - 1 && b > c && b != 9 {
                let p = (i + 1, j);
                if !seen.contains(&p) {
                    seen.insert(p);
                    queue.push(p);
                }
            }
            if j > 0 && l > c && l != 9 {
                let p = (i, j - 1);
                if !seen.contains(&p) {
                    seen.insert(p);
                    queue.push(p);
                }
            }
        }
        size
    }

    let mut sizes = part1(field.clone())
        .1
        .iter()
        .map(|depth| bfs(&field, *depth))
        .collect::<Vec<_>>();

    sizes.sort_by(|a, b| a.cmp(b).reverse());
    sizes.iter().take(3).product()
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
            .map(|line| {
                println!("{:?}", line);
                line
            })
            .collect::<Vec<_>>();

        assert_eq!(part1(input).0, 15);
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

        assert_eq!(part2(input), 1134);
    }
}

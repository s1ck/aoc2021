use std::collections::HashSet;

pub fn run(lines: &str) -> (usize, usize) {
    let (coords, folds) = parse(lines);

    (part1(&coords, &folds), part2(&coords, &folds))
}

fn part1(coords: &[(u32, u32)], folds: &[(char, u32)]) -> usize {
    fold(coords, folds[0]).len()
}

fn part2(coords: &[(u32, u32)], folds: &[(char, u32)]) -> usize {
    let coords = coords.iter().cloned().collect::<Vec<_>>();
    let coords = folds.iter().fold(coords, |c, f| fold(&c, *f));

    let (width, height) = coords.iter().max().unwrap();

    let mut m = vec![vec![' '; *width as usize + 1]; *height as usize + 1];

    coords
        .iter()
        .for_each(|(x, y)| m[*y as usize][*x as usize] = 'x');

    m.iter()
        .for_each(|row| println!("{}", row.iter().collect::<String>()));

    0
}

fn fold(coords: &[(u32, u32)], (axis, line): (char, u32)) -> Vec<(u32, u32)> {
    coords
        .iter()
        .fold(HashSet::new(), |mut set, (x, y)| {
            set.insert(match axis {
                'x' if *x > line => (line - (*x - line), *y),
                'y' if *y > line => (*x, line - (*y - line)),
                _ => (*x, *y),
            });
            set
        })
        .into_iter()
        .collect::<Vec<_>>()
}

fn parse(lines: &str) -> (Vec<(u32, u32)>, Vec<(char, u32)>) {
    let (coords, folds) = lines.split_once("\n\n").unwrap();

    let coords = coords
        .split('\n')
        .map(|line| line.trim())
        .map(|line| line.split_once(',').unwrap())
        .map(|(x, y)| (x.parse::<u32>().unwrap(), y.parse::<u32>().unwrap()))
        .collect::<Vec<_>>();

    let folds = folds
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| line.trim())
        .map(|line| line.split(' ').nth(2).unwrap())
        .map(|fold| fold.split_once('=').unwrap())
        .map(|(axis, pos)| (axis.chars().nth(0).unwrap(), pos.parse::<u32>().unwrap()))
        .collect::<Vec<_>>();

    (coords, folds)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"6,10
                           0,14
                           9,10
                           0,3
                           10,4
                           4,11
                           6,0
                           6,12
                           4,1
                           0,13
                           10,12
                           3,4
                           3,0
                           8,4
                           1,10
                           2,14
                           8,10
                           9,0

                           fold along y=7
                           fold along x=5"#;
    #[test]
    fn test_part1() {
        let (coords, folds) = parse(INPUT);

        assert_eq!(part1(&coords, &folds), 17);
    }
}

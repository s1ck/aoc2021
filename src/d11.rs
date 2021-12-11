pub fn run(mut dumbos: Vec<Vec<u8>>) -> (usize, usize) {
    (
        part1(&mut dumbos.clone()) as usize,
        part2(&mut dumbos) as usize,
    )
}

fn part1(field: &mut [Vec<u8>]) -> u32 {
    let mut flashes = 0;

    for _ in 0..100 {
        let mut queue = phase1(field);

        while let Some(next) = queue.pop() {
            flashes += 1;
            phase2(next, field, &mut queue);
        }
    }

    flashes
}

fn part2(field: &mut [Vec<u8>]) -> u32 {
    let mut step = 1;

    loop {
        let mut flashes = 0;
        let mut queue = phase1(field);

        while let Some(next) = queue.pop() {
            flashes += 1;
            phase2(next, field, &mut queue);
        }

        if flashes == 100 {
            return step;
        }

        step += 1;
    }
}

fn phase1(field: &mut [Vec<u8>]) -> Vec<(usize, usize)> {
    let mut queue = vec![];

    for i in 0..field.len() {
        for j in 0..field.len() {
            field[i][j] += 1;

            if field[i][j] > 9 {
                queue.push((i, j));
            }
        }
    }

    queue
}

fn phase2((row, col): (usize, usize), field: &mut [Vec<u8>], queue: &mut Vec<(usize, usize)>) {
    field[row as usize][col as usize] = 0;

    for (n_row, n_col) in [
        (row.wrapping_sub(1), col.wrapping_sub(1)),
        (row.wrapping_sub(1), col),
        (row.wrapping_sub(1), col + 1),
        (row, col.wrapping_sub(1)),
        (row, col + 1),
        (row + 1, col.wrapping_sub(1)),
        (row + 1, col),
        (row + 1, col + 1),
    ] {
        if let Some(n) = field
            .get_mut(n_row as usize)
            .and_then(|row| row.get_mut(n_col as usize))
        {
            if *n > 0 {
                *n += 1;

                if *n > 9 && !queue.contains(&(n_row, n_col)) {
                    queue.push((n_row, n_col));
                }
            }
        }
    }
}

pub fn parse(line: &str) -> Vec<u8> {
    line.trim().bytes().map(|b| b - b'0').collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"5483143223
                           2745854711
                           5264556173
                           6141336146
                           6357385478
                           4167524645
                           2176841721
                           6882881134
                           4846848554
                           5283751526"#;
    #[test]
    fn test_part1() {
        let mut input = INPUT
            .split('\n')
            .map(|line| parse(line))
            .collect::<Vec<_>>();

        assert_eq!(part1(&mut input), 1656);
    }

    #[test]
    fn test_part2() {
        let mut input = INPUT
            .split('\n')
            .map(|line| parse(line))
            .collect::<Vec<_>>();

        assert_eq!(part2(&mut input), 195);
    }
}

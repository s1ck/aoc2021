const SIZE: usize = 10;

pub fn run(lines: &[&str]) -> (usize, usize) {
    (
        part1(&mut parse(lines)) as usize,
        part2(&mut parse(lines)) as usize,
    )
}

fn part1(field: &mut [[u8; SIZE]; SIZE]) -> u32 {
    let mut flashes = 0;

    for _ in 0..SIZE * SIZE {
        let mut queue = increment(field);

        while let Some(next) = queue.pop() {
            flashes += 1;
            flash(next, field, &mut queue);
        }
    }

    flashes
}

fn part2(field: &mut [[u8; SIZE]; SIZE]) -> u32 {
    let mut step = 1;

    loop {
        let mut flashes = 0;
        let mut queue = increment(field);

        while let Some(next) = queue.pop() {
            flashes += 1;
            flash(next, field, &mut queue);
        }

        if flashes == SIZE * SIZE {
            return step;
        }

        step += 1;
    }
}

fn increment(field: &mut [[u8; SIZE]; SIZE]) -> Vec<(usize, usize)> {
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

fn flash(
    (row, col): (usize, usize),
    field: &mut [[u8; SIZE]; SIZE],
    queue: &mut Vec<(usize, usize)>,
) {
    field[row][col] = 0;

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

fn parse(lines: &[&str]) -> [[u8; SIZE]; SIZE] {
    let mut res = [[0; SIZE]; SIZE];

    lines.iter().enumerate().for_each(|(row, line)| {
        line.trim()
            .bytes()
            .enumerate()
            .for_each(|(col, b)| res[row][col] = b - b'0')
    });

    res
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
        let input = INPUT.split('\n').collect::<Vec<_>>();
        assert_eq!(run(&input).0, 1656);
    }

    #[test]
    fn test_part2() {
        let input = INPUT.split('\n').collect::<Vec<_>>();
        assert_eq!(run(&input).1, 195);
    }
}

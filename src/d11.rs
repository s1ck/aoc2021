pub fn run(mut dumbos: Vec<Vec<u8>>) -> (usize, usize) {
    (
        part1(&mut dumbos.clone()) as usize,
        part2(&mut dumbos) as usize,
    )
}

fn part1(dumbos: &mut [Vec<u8>]) -> u32 {
    let mut flashes = 0;

    for _ in 0..100 {
        let mut flash_queue = phase1(dumbos);

        while let Some(next) = flash_queue.pop() {
            flashes += 1;
            phase2(next, dumbos, &mut flash_queue);
        }
    }

    return flashes;
}

fn part2(dumbos: &mut [Vec<u8>]) -> u32 {
    let mut step = 1;

    loop {
        let mut flashes = 0;
        let mut flash_queue = phase1(dumbos);

        while let Some(next) = flash_queue.pop() {
            flashes += 1;
            phase2(next, dumbos, &mut flash_queue);
        }

        if flashes == 100 {
            return step;
        }
        step += 1;
    }
}

fn phase1(dumbos: &mut [Vec<u8>]) -> Vec<(i32, i32)> {
    let mut flash = vec![];

    for i in 0..dumbos.len() {
        for j in 0..dumbos.len() {
            dumbos[i][j] += 1;

            if dumbos[i][j] > 9 {
                flash.push((i as i32, j as i32));
            }
        }
    }

    flash
}

fn phase2((x, y): (i32, i32), dumbos: &mut [Vec<u8>], flash_queue: &mut Vec<(i32, i32)>) {
    const NEIGHBORS: [(i32, i32); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    println!("flash = ({},{})", x, y);

    for (d_x, d_y) in NEIGHBORS {
        let n_x = x + d_x;
        let n_y = y + d_y;

        if n_x < 0 || n_x == dumbos.len() as i32 {
            continue;
        }

        if n_y < 0 || n_y == dumbos.len() as i32 {
            continue;
        }

        // let n_x = (x + d_x).clamp(0, dumbos.len() as i32 - 1);
        // let n_y = (y + d_y).clamp(0, dumbos.len() as i32 - 1);

        let n = &mut dumbos[n_x as usize][n_y as usize];

        // 0 - flashed
        if *n > 0 {
            *n += 1;

            if *n > 9 && !flash_queue.contains(&(n_x, n_y)) {
                flash_queue.push((n_x, n_y));
            }
        }
    }

    dumbos[x as usize][y as usize] = 0;
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

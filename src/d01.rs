pub fn run(measurements: &[i64]) -> (u32, u32) {
    (part1(measurements), part2(measurements))
}

fn part1(measurements: &[i64]) -> u32 {
    measurements.windows(2).fold(
        0_u32,
        |cnt, slice| if slice[1] > slice[0] { cnt + 1 } else { cnt },
    )
}

fn part2(measurements: &[i64]) -> u32 {
    let sums = measurements
        .windows(3)
        .map(|slice| slice.iter().sum())
        .collect::<Vec<_>>();
    part1(&sums)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        assert_eq!(7, part1(&input));
    }

    #[test]
    fn test_part2() {
        let input = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        assert_eq!(5, part2(&input))
    }
}

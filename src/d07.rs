pub fn run(line: &[i32]) -> (usize, usize) {
    (part1(line) as usize, part2(line) as usize)
}

pub fn parse(line: &str) -> Vec<i32> {
    line.split(',')
        .map(|d| d.parse::<i32>().unwrap())
        .collect::<Vec<_>>()
}

fn part1(positions: &[i32]) -> i32 {
    let max_position = positions.iter().max().unwrap();

    (0..*max_position)
        .into_iter()
        .map(|x| positions.iter().map(|position| (x - position).abs()).sum())
        .min()
        .unwrap()
}

fn part2(positions: &[i32]) -> i32 {
    let max_position = positions.iter().max().unwrap();

    (0..*max_position)
        .into_iter()
        .map(|x| {
            positions
                .iter()
                .map(|position| {
                    let diff = (x - position).abs();
                    diff * (diff + 1) / 2
                })
                .sum()
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test_part1() {
        let positions = INPUT
            .split(',')
            .map(|d| d.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        assert_eq!(part1(&positions), 37);
    }

    #[test]
    fn test_part2() {
        let positions = INPUT
            .split(',')
            .map(|d| d.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        assert_eq!(part2(&positions), 168);
    }
}

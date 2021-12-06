pub fn run(lines: &[usize]) -> (usize, usize) {
    (part1(lines), part2(lines))
}

fn part1(input: &[usize]) -> usize {
    simulate(input, 80)
}

fn part2(input: &[usize]) -> usize {
    simulate(input, 256)
}

fn simulate(fish: &[usize], runs: usize) -> usize {
    let mut fish_per_day = [0_usize; 9];

    for day in fish {
        fish_per_day[*day] += 1;
    }

    for _ in 0..runs {
        // n fishies with day 0 spawn n new fishies with day 8
        // and re-enter the simulation as day 6 fishies ..
        fish_per_day.rotate_left(1);
        fish_per_day[6] += fish_per_day[8];
    }

    fish_per_day.iter().sum()
}

pub fn parse(line: &str) -> Vec<usize> {
    line.split(',')
        .map(|f| f.parse::<usize>().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &'static str = "3,4,3,1,2";

    #[test]
    fn test_part1() {
        let input = INPUT
            .split(',')
            .map(|d| d.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        assert_eq!(part1(&input), 5934);
    }

    #[test]
    fn test_part2() {
        let input = INPUT
            .split(',')
            .map(|d| d.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        assert_eq!(part2(&input), 26984457539);
    }
}

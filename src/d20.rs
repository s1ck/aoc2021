use std::collections::HashSet;

pub fn run(lines: &str) -> (usize, usize) {
    let (algo, map) = parse(lines);

    (part1(map.clone(), &algo), part2(map, &algo))
}

fn part1(map: HashSet<(isize, isize)>, algo: &[bool]) -> usize {
    simulate(map, algo, 2)
}

fn part2(map: HashSet<(isize, isize)>, algo: &[bool]) -> usize {
    simulate(map, algo, 50)
}

fn simulate(mut map: HashSet<(isize, isize)>, algo: &[bool], steps: usize) -> usize {
    let mut draw_infinite = false;
    // algo[0] encodes if we need to draw '#' for points that represent infinity.
    // If this is the case, we need to toggle this with each step to remove the
    // '#'s drawn in the previous step.
    let toggle = algo[0];

    for _ in 0..steps {
        map = step(&map, algo, draw_infinite);

        if toggle {
            draw_infinite = !draw_infinite;
        }
    }

    map.len()
}

fn step(
    map: &HashSet<(isize, isize)>,
    algo: &[bool],
    draw_infinite: bool,
) -> HashSet<(isize, isize)> {
    let r_min = *map.iter().map(|(r, _)| r).min().unwrap();
    let r_max = *map.iter().map(|(r, _)| r).max().unwrap();
    let c_min = *map.iter().map(|(_, c)| c).min().unwrap();
    let c_max = *map.iter().map(|(_, c)| c).max().unwrap();

    let mut res = HashSet::new();

    for r in (r_min - 2)..(r_max + 2) {
        for c in (c_min - 2)..(c_max + 2) {
            let mut idx: u32 = 0;
            let mut bit: u32 = 8;

            for r_delta in [-1, 0, 1] {
                for c_delta in [-1, 0, 1] {
                    let r_nbr = r + r_delta;
                    let c_nbr = c + c_delta;

                    let pick = if r_nbr < r_min || r_nbr > r_max || c_nbr < c_min || c_nbr > c_max {
                        draw_infinite
                    } else {
                        map.contains(&(r_nbr, c_nbr))
                    };

                    if pick {
                        idx += 2_u32.pow(bit);
                    }
                    bit = bit.wrapping_sub(1);
                }
            }

            if algo[idx as usize] {
                res.insert((r, c));
            }
        }
    }

    res
}

fn parse(lines: &str) -> (Vec<bool>, HashSet<(isize, isize)>) {
    let (algo, map) = lines.split_once("\n\n").unwrap();

    let algo = algo.chars().map(|c| c == '#').collect::<Vec<_>>();

    assert_eq!(algo.len(), 512);

    let map = map
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.trim()
                .chars()
                .enumerate()
                .filter_map(|(col, bit)| {
                    if bit == '#' {
                        Some((row as isize, col as isize))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<HashSet<_>>();

    (algo, map)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse() {
        let lines = std::fs::read_to_string("input/d20-test.txt").expect("file not found");

        let (algo, map) = parse(&lines);

        assert!(map.contains(&(0, 0)));
        assert!(map.contains(&(1, 0)));
        assert!(map.contains(&(2, 0)));
        assert!(map.contains(&(2, 1)));
        assert!(map.contains(&(3, 2)));
        assert!(map.contains(&(4, 2)));
        assert!(map.contains(&(0, 3)));
        assert!(map.contains(&(4, 3)));
        assert!(map.contains(&(2, 4)));
        assert!(map.contains(&(4, 4)));
        assert_eq!(map.len(), 10);
        assert_eq!(algo.len(), 512);
        assert_eq!(algo[0..5], vec![false, false, true, false, true]);
    }

    #[test]
    fn test_part1_sample() {
        let lines = std::fs::read_to_string("input/d20-test.txt").expect("file not found");

        let (algo, map) = parse(&lines);

        assert_eq!(part1(map, &algo), 35);
    }

    #[test]
    fn test_part1_input() {
        let lines = std::fs::read_to_string("input/d20.txt").expect("file not found");

        let (algo, map) = parse(&lines);

        assert_eq!(part1(map, &algo), 5359);
    }

    #[test]
    fn test_part2_sample() {
        let lines = std::fs::read_to_string("input/d20-test.txt").expect("file not found");

        let (algo, map) = parse(&lines);

        assert_eq!(part2(map, &algo), 3351);
    }

    #[test]
    fn test_part2_input() {
        let lines = std::fs::read_to_string("input/d20.txt").expect("file not found");

        let (algo, map) = parse(&lines);

        assert_eq!(part2(map, &algo), 12333);
    }
}

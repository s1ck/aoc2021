pub fn run(line: &str) -> (usize, usize) {
    let area = parse(line);
    let (peak, vels) = simulate(area);

    (peak as usize, vels as usize)
}

fn simulate(target_area: ((i32, i32), (i32, i32))) -> (i32, usize) {
    let mut peak = 0;
    let mut vels = 0;

    // target area: x=156..202, y=-110..-69
    for y_v in -110..200 {
        for x_v in 1..250 {
            let (hit, y_max) = fire(x_v, y_v, target_area);
            if hit {
                vels += 1;
                peak = i32::max(peak, y_max);
            }
        }
    }

    return (peak, vels);
}

fn fire(
    mut x_v: i32,
    mut y_v: i32,
    ((x_from, x_to), (y_from, y_to)): ((i32, i32), (i32, i32)),
) -> (bool, i32) {
    let mut x = 0;
    let mut y = 0;
    let mut peak = y;

    loop {
        x += x_v;
        y += y_v;

        if x_v > 0 {
            x_v -= 1
        } else if x_v < 0 {
            x_v += 1
        };

        y_v -= 1;

        if y > peak {
            peak = y;
        }

        if x >= x_from && x <= x_to && y >= y_from && y <= y_to {
            return (true, peak);
        }
        if x > x_to || y < y_from {
            return (false, 0);
        }
    }
}

fn parse(line: &str) -> ((i32, i32), (i32, i32)) {
    let area = line.split_once(": ").unwrap().1;
    let (x, y) = area.split_once(", ").unwrap();
    let x_range = x
        .split_once('=')
        .unwrap()
        .1
        .split_once("..")
        .map(|(l, h)| (l.parse::<i32>().unwrap(), h.parse::<i32>().unwrap()))
        .unwrap();
    let y_range = y
        .split_once('=')
        .unwrap()
        .1
        .split_once("..")
        .map(|(l, h)| (l.parse::<i32>().unwrap(), h.parse::<i32>().unwrap()))
        .unwrap();

    (x_range, y_range)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "target area: x=20..30, y=-10..-5";
    const INPUT1: &str = "target area: x=156..202, y=-110..-69";

    #[test]
    fn test_part1() {
        let target_area = parse(INPUT);

        assert_eq!(simulate(target_area).0, 45);
    }

    #[test]
    fn test_part2() {
        let target_area = parse(INPUT);

        assert_eq!(simulate(target_area).1, 112);
    }

    #[test]
    fn test_part1_input() {
        let target_area = parse(INPUT1);

        assert_eq!(simulate(target_area).0, 5995);
    }

    #[test]
    fn test_part2_input() {
        let target_area = parse(INPUT1);

        assert_eq!(simulate(target_area).1, 3202);
    }
}

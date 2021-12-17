pub fn run(line: &str) -> (usize, usize) {
    simulate(parse(line))
}

fn simulate(target @ ((_, x_max), (y_min, _)): ((i32, i32), (i32, i32))) -> (usize, usize) {
    let mut peak = 0;
    let mut vels = 0;

    (1..=x_max).for_each(|d_x| {
        (y_min..=y_min.abs()).for_each(|d_y| {
            let (hit, y_max) = fire(d_x, d_y, target);
            if hit {
                vels += 1;
                peak = i32::max(peak, y_max);
            }
        })
    });

    (peak as usize, vels)
}

fn fire(
    mut x_v: i32,
    mut y_v: i32,
    ((x_min, x_max), (y_min, y_max)): ((i32, i32), (i32, i32)),
) -> (bool, i32) {
    let mut x = 0;
    let mut y = 0;
    let mut peak = y;

    loop {
        x += x_v;
        y += y_v;

        x_v -= x_v.signum();
        y_v -= 1;

        peak = peak.max(y);

        if x >= x_min && x <= x_max && y >= y_min && y <= y_max {
            return (true, peak);
        }
        if x > x_max || y < y_min {
            return (false, peak);
        }
    }
}

fn parse(line: &str) -> ((i32, i32), (i32, i32)) {
    let (x, y) = line[13..].split_once(", ").unwrap();
    let x_range = x[2..]
        .split_once("..")
        .map(|(l, h)| (l.parse::<i32>().unwrap(), h.parse::<i32>().unwrap()))
        .unwrap();
    let y_range = y[2..]
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
    const INPUT2: &str = "target area: x=70..125, y=-159..-121";

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

    #[test]
    fn test_paul() {
        let target_area = parse(INPUT2);

        let (peak, vels) = simulate(target_area);

        assert_eq!(peak, 12561);
        assert_eq!(vels, 3785);
    }
}

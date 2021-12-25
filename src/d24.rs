// each block has the same instructions
// but different c1, c2, and c3 values

// inp w
// mul x 0
// add x z
// mod x 26
// div z 1  -> c1
// add x 14 -> c2
// eql x w
// eql x 0
// mul y 0
// add y 25
// mul y x
// add y 1
// mul z y
// mul y 0
// add y w
// add y 8  -> c3
// mul y x
// add z y

#[rustfmt::skip]
const C1: [i32;14] = [1,   1,  1,  1,  26,  1, 26,  26,  1,  1, 26,  26,  26, 26];
#[rustfmt::skip]
const C2: [i32;14] = [14, 13, 13, 12, -12, 12, -2, -11, 13, 14,  0, -12, -13, -6];
#[rustfmt::skip]
const C3: [i32;14] = [8,   8,  3, 10,   8,  8,  8,   5,  9,  3,  4,   9,   2,  7];

pub fn run() -> (usize, usize) {
    (part1(), part2())
}

fn part1() -> usize {
    let mut z = vec![];
    let mut res = vec![];

    let w = 9;

    (0..14).for_each(|op| {
        if C1[op] == 26 {
            let (j, v) = z.pop().unwrap();
            if v + C2[op] > w {
                res[j] -= v + C2[op] - w;
                res.push(w);
            } else {
                res.push(v + C2[op]);
            }
        } else {
            z.push((res.len(), w + C3[op]));
            res.push(w);
        }
    });

    res.iter()
        .map(|n| n.to_string())
        .collect::<String>()
        .parse::<usize>()
        .unwrap()
}

fn part2() -> usize {
    let mut z: Vec<(usize, i32)> = vec![];
    let mut res: Vec<i32> = vec![];

    let w = 1;

    (0..14).for_each(|op| {
        if C1[op] == 26 {
            let (j, v) = z.pop().unwrap();
            if v + C2[op] <= 0 {
                res[j] += -(v + C2[op]) + w;
                res.push(w);
            } else {
                res.push(v + C2[op]);
            }
        } else {
            z.push((res.len(), w + C3[op]));
            res.push(w);
        }
    });

    res.iter()
        .map(|n| n.to_string())
        .collect::<String>()
        .parse::<usize>()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part1(), 79997391969649);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part2(), 16931171414113);
    }
}

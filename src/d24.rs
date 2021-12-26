// each block has the same instructions
// but different c1, c2, and c3 values

// type 1
// ------
// inp w    -> a digit between 1 and 9
// mul x 0
// add x z
// mod x 26
// div z 1  -> c1
// add x 14 -> c2 | x = (z % 26) + 14 .. C2 is always greater than 9, i.e. x is always greater than 9, i.e. x != w
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

// z = 26z + w + c3

// type 2
// ------
// inp w
// mul x 0
// add x z
// mod x 26
// div z 26  -> c1
// add x -12 -> c2
// eql x w
// eql x 0
// mul y 0
// add y 25
// mul y x
// add y 1
// mul z y
// mul y 0
// add y w
// add y 9  -> c3
// mul y x
// add z y

// z = if x == w { z / 26 } else { 26 * floor(z / 26) + w + c3 }

// There are two types of blocks
// Type 1 (C1 == 1)  => z = 26z + w + C3
// Type 2 (C1 == 26) => if x == w { z = z / 26 } else { 26 * floor(z / 26) + w + C3 }
// (note, that 26z + w + c3 == 26 * floor(z/26) + w + C3)

// The puzzle input contains:
// 7 blocks of type 1 which always increase z and
// 7 blocks of type 2 which either decrease or "align" z

// the goal is z == 0, i.e. we need 7 increase and 7 decrease ops

// One option is to brute force the input values given in order to make
// sure that there is an equal amount of increase and decrease operations.

// Another implementation idea:

// z can be modeled as an arithmetic stack across the 14 input ops where
// each op can be simplified as follows (see also the check() method)/
// Note, that we only need to care about 'w + C3'.

// inp w
// x = z.top() + C2
// if C1 == 26 {
//   z.pop()
// if x != w {
//   z.push(w + C3)
// }

// The goal is to adapt the w's to have 7 push and 7 pop operations.

#[rustfmt::skip]
const C1: [i32;14] = [1,   1,  1,  1,  26,  1, 26,  26,  1,  1, 26,  26,  26, 26];
#[rustfmt::skip]
const C2: [i32;14] = [14, 13, 13, 12, -12, 12, -2, -11, 13, 14,  0, -12, -13, -6];
#[rustfmt::skip]
const C3: [i32;14] = [8,   8,  3, 10,   8,  8,  8,   5,  9,  3,  4,   9,   2,  7];

pub fn run() -> (usize, usize) {
    (max(), min())
}

// Run the puzzle program for the given input
// and return the arithmetic stack.
//
// The input is valid if the stack is empty.
fn check(input: [i32; 14]) -> Vec<i32> {
    let mut z = vec![];

    for op in 0..14 {
        let w = input[op];
        // x = z.top() + C2
        let x = if op == 0 { 0 } else { z[z.len() - 1] } + C2[op];
        if C1[op] == 26 {
            z.pop();
        }
        if x != w {
            z.push(w + C3[op]);
        }
    }

    z
}

fn max() -> usize {
    let mut z = vec![];
    let mut res = vec![];

    // We start with the max value for w as input
    // for each op and adapt them if necessary
    let w = 9;

    (0..14).for_each(|op| {
        if C1[op] == 1 {
            // type 1 operation
            // we just push w + C3 to the stack and
            // store w (9) as input for that op
            z.push((res.len(), w + C3[op]));
            res.push(w);
        } else {
            // type 2 operation
            assert!(C1[op] == 26);
            assert!(C2[op] <= 0);
            let (j, v) = z.pop().unwrap();

            // We need to make sure that w stays within its bounds.
            if v + C2[op] > w {
                // If the corresponding push operation picked a value
                // for w that is too large, we need to adapt it now.
                res[j] -= v + C2[op] - w;
                res.push(w);
            } else {
                res.push(v + C2[op]);
            }
        }
    });

    res.iter()
        .map(|n| n.to_string())
        .collect::<String>()
        .parse::<usize>()
        .unwrap()
}

fn min() -> usize {
    let mut z: Vec<(usize, i32)> = vec![];
    let mut res: Vec<i32> = vec![];

    // We start with the min value for w as input
    // for each op and adapt them if necessary
    let w = 1;

    (0..14).for_each(|op| {
        if C1[op] == 1 {
            z.push((res.len(), w + C3[op]));
            res.push(w);
        } else {
            let (j, v) = z.pop().unwrap();
            if v + C2[op] <= 0 {
                res[j] += -(v + C2[op]) + w;
                res.push(w);
            } else {
                res.push(v + C2[op]);
            }
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
    fn test_check_valid() {
        let max = [7, 9, 9, 9, 7, 3, 9, 1, 9, 6, 9, 6, 4, 9];
        assert_eq!(check(max), vec![]);
    }

    #[test]
    fn test_check_invalid() {
        let max = [4, 2, 4, 2, 4, 2, 4, 2, 4, 2, 4, 2, 4, 2];
        assert_eq!(check(max), vec![12, 10, 7, 12, 7, 13, 9]); // invalid
    }

    #[test]
    fn test_part_1() {
        assert_eq!(max(), 79997391969649);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(min(), 16931171414113);
    }
}

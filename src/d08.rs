use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

pub fn run(inputs: &[Input]) -> (usize, usize) {
    (part1(inputs) as usize, part2(inputs) as usize)
}

#[derive(Debug)]
pub struct Input {
    input: Vec<String>,
    output: Vec<String>,
}

impl FromStr for Input {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (input, output) = s.split_once(" | ").unwrap();
        let input = input.split(' ').map(String::from).collect::<Vec<_>>();
        let output = output.split(' ').map(String::from).collect::<Vec<_>>();

        Ok(Input { input, output })
    }
}
// 0 -> 6
// 1 -> 2*
// 2 -> 5
// 3 -> 5
// 4 -> 4*
// 5 -> 5
// 6 -> 6
// 7 -> 3*
// 8 -> 7*
// 9 -> 6
fn part1(inputs: &[Input]) -> u32 {
    inputs.iter().fold(0, |sum, input| {
        sum + input.output.iter().fold(0, |sum, word| {
            sum + match word.len() {
                2 | 3 | 4 | 7 => 1,
                _ => 0,
            }
        })
    })
}

fn part2(inputs: &[Input]) -> u32 {
    fn decode_input(input: &Input) -> u32 {
        let Input { input, output } = input;

        let input = input
            .iter()
            .map(|word| word.chars().collect::<HashSet<_>>())
            .collect::<Vec<_>>();

        let one = input.iter().find(|word| word.len() == 2).unwrap();
        let four = input.iter().find(|word| word.len() == 4).unwrap();
        let seven = input.iter().find(|word| word.len() == 3).unwrap();
        let eight = input.iter().find(|word| word.len() == 7).unwrap();

        let dummy = HashSet::new();
        let mut found = Vec::with_capacity(10);
        found.resize(10, &dummy);

        found[1] = one;
        found[4] = four;
        found[7] = seven;
        found[8] = eight;

        let nine = input
            .iter()
            .filter(|word| !found.contains(&word))
            .find(|&word| {
                (eight - word).len() == 1 && (one - word).len() == 0 && (four - word).len() == 0
            })
            .unwrap();

        found[9] = nine;

        let six = input
            .iter()
            .filter(|word| !found.contains(&word))
            .find(|&word| (eight - word).len() == 1 && (one - word).len() == 1)
            .unwrap();

        found[6] = six;

        let zero = input
            .iter()
            .filter(|word| !found.contains(&word))
            .find(|word| word.len() == 6)
            .unwrap();

        found[0] = zero;

        let five = input
            .iter()
            .filter(|word| !found.contains(&word))
            .find(|&word| (word - nine).len() == 0 && (one - word).len() == 1)
            .unwrap();

        found[5] = five;

        let two = input
            .iter()
            .filter(|word| !found.contains(&word))
            .find(|&word| word.len() == 5 && (one - word).len() == 1)
            .unwrap();

        found[2] = two;

        let three = input
            .iter()
            .filter(|word| !found.contains(&word))
            .find(|word| true)
            .unwrap();

        found[3] = three;

        let output = output
            .iter()
            .map(|word| {
                let word = word.chars().collect::<HashSet<_>>();
                let (n, _) = found
                    .iter()
                    .enumerate()
                    .find(|(i, &n)| (n - &word).len() == 0 && (&word - n).len() == 0)
                    .unwrap();

                n.to_string()
            })
            .collect::<String>();

        output.parse::<u32>().unwrap()
    }

    inputs
        .iter()
        .fold(0, |sum, input| sum + decode_input(input))
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &str = r#"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
                           edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
                           fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
                           fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
                           aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
                           fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
                           dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
                           bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
                           egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
                           gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"#;

    #[test]
    fn test_part1() {
        let input = INPUT
            .split('\n')
            .map(|line| line.trim())
            .map(|line| line.parse::<Input>().unwrap())
            .collect::<Vec<_>>();

        assert_eq!(part1(&input), 26);
    }

    #[test]
    fn test_part2() {
        let input = INPUT
            .split('\n')
            .map(|line| line.trim())
            .map(|line| line.parse::<Input>().unwrap())
            .collect::<Vec<_>>();

        assert_eq!(part2(&input), 61229);
    }

    #[test]
    fn vec_eq() {
        assert_eq!(vec![1, 2, 3], vec![1, 2, 3])
    }
}

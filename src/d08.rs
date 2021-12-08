use std::{collections::HashSet, str::FromStr};

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
    fn decode(input: &Input) -> u32 {
        let Input { input, output } = input;

        let input = input
            .iter()
            .map(|word| word.chars().collect::<HashSet<_>>())
            .collect::<Vec<_>>();

        let mut found = Vec::with_capacity(10);
        found.resize(10, None);

        found[1] = input.iter().find(|word| word.len() == 2);
        found[4] = input.iter().find(|word| word.len() == 4);
        found[7] = input.iter().find(|word| word.len() == 3);
        found[8] = input.iter().find(|word| word.len() == 7);
        found[9] = input
            .iter()
            .filter(|&word| !found.contains(&Some(word)))
            .find(|&word| {
                (found[8].unwrap() - word).len() == 1
                    && (found[1].unwrap() - word).is_empty()
                    && (found[4].unwrap() - word).is_empty()
            });
        found[6] = input
            .iter()
            .filter(|&word| !found.contains(&Some(word)))
            .find(|&word| {
                (found[8].unwrap() - word).len() == 1 && (found[1].unwrap() - word).len() == 1
            });
        found[0] = input
            .iter()
            .filter(|&word| !found.contains(&Some(word)))
            .find(|word| word.len() == 6);
        found[5] = input
            .iter()
            .filter(|&word| !found.contains(&Some(word)))
            .find(|&word| {
                (word - found[9].unwrap()).is_empty() && (found[1].unwrap() - word).len() == 1
            });
        found[2] = input
            .iter()
            .filter(|&word| !found.contains(&Some(word)))
            .find(|&word| word.len() == 5 && (found[1].unwrap() - word).len() == 1);
        found[3] = input
            .iter()
            .filter(|&word| !found.contains(&Some(word)))
            .find(|_| true);

        output
            .iter()
            .rev()
            .map(|word| word.chars().collect::<HashSet<_>>())
            .map(|word| {
                found
                    .iter()
                    .enumerate()
                    .find(|(_, &n)| n.unwrap() == &word)
                    .unwrap()
                    .0 as u32
            })
            .enumerate()
            .fold(0, |sum, (i, n)| sum + (10_u32.pow(i as u32) * n))
    }

    inputs.iter().fold(0, |sum, input| sum + decode(input))
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
}

use Type::*;

#[derive(Clone, Copy)]
enum Type {
    Oxygen,
    Co2,
}

pub fn run(ins: &[&str]) -> (u32, u32) {
    (part1(ins), part2(ins))
}

fn part1(words: &[&str]) -> u32 {
    let input_len = words.len() as u32;
    let word_len = words[0].len() as u32;
    let column_counts = column_counts(words);

    let gamma_rate = column_counts
        .into_iter()
        .map(|n| if input_len - n > n { "1" } else { "0" })
        .collect::<String>();

    let gamma_rate = u32::from_str_radix(&gamma_rate, 2).unwrap();
    let mask = u32::pow(2, word_len) - 1;
    let epsilon_rate = !gamma_rate & mask;

    gamma_rate * epsilon_rate
}

fn part2(words: &[&str]) -> u32 {
    fn filter_words<'a>(w: &[&'a str], i: usize, tpe: Type) -> Vec<&'a str> {
        let column_counts = column_counts(w);
        let mut filter_one = w.len() as u32 - column_counts[i] <= column_counts[i];

        if matches!(tpe, Co2) {
            filter_one = !filter_one;
        }

        w.iter()
            .filter_map(|word| match word.chars().nth(i).unwrap() {
                '0' if !filter_one => Some(*word),
                '1' if filter_one => Some(*word),
                _ => None,
            })
            .collect::<Vec<_>>()
    }

    fn rating(words: &[&str], tpe: Type) -> u32 {
        let mut i = 0;
        let mut next = filter_words(words, i, tpe);
        while next.len() > 1 && i < words[0].len() {
            i += 1;
            next = filter_words(next.as_slice(), i, tpe);
        }
        u32::from_str_radix(next[0], 2).unwrap()
    }

    let oxy_rate = rating(words, Oxygen);
    let co2_rate = rating(words, Co2);

    oxy_rate * co2_rate
}

fn column_counts(words: &[&str]) -> Vec<u32> {
    words.iter().fold(vec![0; words[0].len()], |mut v, n| {
        v.iter_mut()
            .zip(n.chars().into_iter().map(|c| c.to_digit(10).unwrap()))
            .for_each(|(sum, bit)| *sum += bit);
        v
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let ins = [
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ];

        let res = part1(&ins);

        assert_eq!(198, res);
    }

    #[test]
    fn test_part2() {
        let ins = [
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ];

        let res = part2(&ins);

        assert_eq!(230, res);
    }
}

use std::collections::HashMap;

pub fn run(input: &str) -> (usize, usize) {
    let (template, rules) = parse(input);

    (
        part1(template.clone(), &rules, 10),
        part2(template, &rules, 40),
    )
}

// brute-force
fn part1(template: Vec<char>, rules: &HashMap<(char, char), char>, steps: u32) -> usize {
    let mut next = template;

    for _ in 0..steps {
        next = next
            .array_windows()
            .fold(vec![next[0]], |mut v, [left, right]| {
                let mid = rules.get(&(*left, *right)).unwrap();

                v.push(*mid);
                v.push(*right);

                v
            });
    }

    let counts = next.iter().fold(HashMap::new(), |mut counts, c| {
        *counts.entry(*c).or_insert(0) += 1;
        counts
    });

    diff(&counts)
}

// map-based
fn part2(template: Vec<char>, rules: &HashMap<(char, char), char>, steps: u32) -> usize {
    let mut counts = template
        .array_windows()
        .fold(HashMap::new(), |mut counts, [left, right]| {
            *counts.entry((*left, *right)).or_insert(0) += 1;
            counts
        });

    let mut char_counts = template.iter().fold(HashMap::new(), |mut counts, n| {
        *counts.entry(*n).or_insert(0) += 1;
        counts
    });

    for _ in 0..steps {
        let mut next = counts.clone();

        for (t @ (left, right), count) in counts.iter() {
            let mid = rules.get(t).unwrap();

            *next.entry((*left, *right)).or_insert(0) -= count;
            *next.entry((*left, *mid)).or_insert(0) += count;
            *next.entry((*mid, *right)).or_insert(0) += count;

            *char_counts.entry(*mid).or_insert(0) += count;
        }

        counts = next;
    }

    diff(&char_counts)
}

fn diff(counts: &HashMap<char, usize>) -> usize {
    let mut counts = counts.iter().collect::<Vec<_>>();

    counts.sort_by(|t1, t2| t1.1.cmp(t2.1));

    let min = counts[0].1;
    let max = counts[counts.len() - 1].1;

    max - min
}

fn parse(input: &str) -> (Vec<char>, HashMap<(char, char), char>) {
    let (template, rules) = input.split_once("\n\n").unwrap();

    let template = template.chars().collect::<Vec<_>>();

    let rules = rules
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| line.trim().split_once(" -> ").unwrap())
        .map(|(pair, insertion)| {
            let mut pair = pair.chars();
            let pair = (pair.next().unwrap(), pair.next().unwrap());
            let insertion = insertion.chars().next().unwrap();
            (pair, insertion)
        })
        .collect::<HashMap<_, _>>();

    (template, rules)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"NNCB

                           CH -> B
                           HH -> N
                           CB -> H
                           NH -> C
                           HB -> C
                           HC -> B
                           HN -> C
                           NN -> C
                           BH -> H
                           NC -> B
                           NB -> B
                           BN -> B
                           BB -> N
                           BC -> B
                           CC -> N
                           CN -> C"#;

    #[test]
    fn test_part1() {
        let (template, rules) = parse(INPUT);

        assert_eq!(part2(template, &rules, 10), 1588);
    }

    #[test]
    fn test_part2() {
        let (template, rules) = parse(INPUT);

        assert_eq!(part2(template, &rules, 40), 2188189693529);
    }
}

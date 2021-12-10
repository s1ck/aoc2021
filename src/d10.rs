pub fn run(lines: &[&str]) -> (usize, usize) {
    (part1(lines) as usize, part2(lines))
}

fn part1(lines: &[&str]) -> u32 {
    lines.iter().map(|line| check_line(line).0).sum()
}

fn part2(lines: &[&str]) -> usize {
    let mut scores = lines
        .iter()
        .map(|line| check_line(line))
        .filter(|(error, _)| *error == 0)
        .map(|(_, stack)| {
            stack
                .iter()
                .rev()
                .map(|c| match c {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => unreachable!(),
                })
                .fold(0, |prod, n| prod * 5 + n)
        })
        .collect::<Vec<_>>();

    scores.sort();
    scores[scores.len() / 2]
}

fn check_line(line: &str) -> (u32, Vec<char>) {
    let mut stack = vec![];

    let error = line
        .chars()
        .map(|c| match c {
            '(' | '[' | '{' | '<' => {
                stack.push(c);
                0
            }
            d @ (')' | ']' | '}' | '>') => match stack.pop().unwrap() {
                '(' if c == ')' => 0,
                '[' if c == ']' => 0,
                '{' if c == '}' => 0,
                '<' if c == '>' => 0,
                _ => match d {
                    ')' => 3,
                    ']' => 57,
                    '}' => 1197,
                    '>' => 25137,
                    _ => unreachable!(),
                },
            },
            d => panic!("found {}", d),
        })
        .find(|x| *x > 0)
        .unwrap_or_default();

    (error, stack)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"[({(<(())[]>[[{[]{<()<>>
                           [(()[<>])]({[<{<<[]>>(
                           {([(<{}[<>[]}>{[]{[(<()>
                           (((({<>}<{<{<>}{[]{[]{}
                           [[<[([]))<([[{}[[()]]]
                           [{[{({}]{}}([{[{{{}}([]
                           {<[[]]>}<{[{[{[]{()[[[]
                           [<(<(<(<{}))><([]([]()
                           <{([([[(<>()){}]>(<<{{
                           <{([{{}}[<[[[<>{}]]]>[]]"#;
    #[test]
    fn test_part1() {
        let lines = INPUT
            .split('\n')
            .map(|line| line.trim())
            .collect::<Vec<_>>();

        assert_eq!(part1(&lines), 26397);
    }

    #[test]
    fn test_part2() {
        let lines = INPUT
            .split('\n')
            .map(|line| line.trim())
            .collect::<Vec<_>>();

        assert_eq!(part2(&lines), 288957);
    }
}

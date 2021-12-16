pub fn run(lines: &str) -> (usize, usize) {
    let lines = parse(lines);

    (part1(&lines), part2(&lines))
}

fn part1(lines: &[String]) -> usize {
    parse_packet(lines[0].as_str()).0
}

fn part2(lines: &[String]) -> usize {
    parse_packet(lines[0].as_str()).2
}

fn parse_packet(p: &str) -> (usize, usize, usize) {
    let mut offset = 0;
    let (version, read) = parse_version(p);
    offset += read;
    let (type_id, read) = parse_type_id(&p[offset..]);
    offset += read;

    let (version_sum, read, value) = match type_id {
        4 => parse_literal(&p[offset..]),
        _ => {
            let (version_sum, read, values) = parse_operator(&p[offset..]);

            let value = match type_id {
                // sum
                0 => values.iter().fold(0, |sum, next| sum + next),
                // product
                1 => values.iter().fold(1, |prod, next| prod * next),
                // min
                2 => values[1..]
                    .iter()
                    .fold(values[0], |min, next| if *next < min { *next } else { min }),
                // max
                3 => values[1..]
                    .iter()
                    .fold(values[0], |max, next| if *next > max { *next } else { max }),
                // greater than
                5 => {
                    if values[0] > values[1] {
                        1
                    } else {
                        0
                    }
                }
                // less than
                6 => {
                    if values[0] < values[1] {
                        1
                    } else {
                        0
                    }
                }
                // equal
                7 => {
                    if values[0] == values[1] {
                        1
                    } else {
                        0
                    }
                }
                _ => unreachable!(),
            };

            (version_sum, read, value)
        }
    };

    offset += read;

    return (version as usize + version_sum, offset, value);

    fn parse_version(p: &str) -> (u32, usize) {
        (u32::from_str_radix(&p[0..3], 2).unwrap(), 3)
    }

    fn parse_type_id(p: &str) -> (u32, usize) {
        (u32::from_str_radix(&p[0..3], 2).unwrap(), 3)
    }

    fn parse_length_type_id(p: &str) -> (u8, usize) {
        (u8::from_str_radix(&p[0..1], 2).unwrap(), 1)
    }

    fn parse_literal(mut p: &str) -> (usize, usize, usize) {
        let mut number = String::new();
        let mut read = 0;

        loop {
            number.push_str(&p[1..5]);
            read += 5;
            if matches!(&p[0..1], "1") {
                p = &p[5..];
            } else {
                break;
            }
        }

        let val = usize::from_str_radix(number.as_str(), 2).unwrap();

        (0, read, val)
    }

    fn parse_operator(p: &str) -> (usize, usize, Vec<usize>) {
        let mut offset = 0;
        let mut version_sum = 0;

        let (length_type_id, read) = parse_length_type_id(p);
        offset += read;

        let mut values = vec![];

        if length_type_id == 0 {
            let bits = u32::from_str_radix(&p[1..16], 2).unwrap();

            offset += 15;
            let boundary = offset + bits as usize;

            while offset < boundary as usize {
                let (version, read, value) = parse_packet(&p[offset..]);
                values.push(value);
                offset += read;
                version_sum += version;
            }
        } else {
            let packets = u32::from_str_radix(&p[1..12], 2).unwrap();

            offset += 11;

            for _ in 0..packets {
                let (version, read, value) = parse_packet(&p[offset..]);
                values.push(value);
                offset += read;
                version_sum += version;
            }
        }

        (version_sum, offset, values)
    }
}

fn parse(lines: &str) -> Vec<String> {
    lines
        .split('\n')
        .map(|line| line.trim())
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(16).unwrap())
                .map(|c| format!("{:04b}", c))
                .collect::<String>()
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT_0: &str = "D2FE28";
    const INPUT_1: &str = "8A004A801A8002F478";
    const INPUT_2: &str = "620080001611562C8802118E34";
    const INPUT_3: &str = "C0015000016115A2E0802F182340";
    const INPUT_4: &str = "A0016C880162017C3686B18A3D4780";
    const INPUT_5: &str = "38006F45291200";

    #[test]
    fn test_part1_0() {
        let input_0 = parse(INPUT_0);
        assert_eq!(part1(&input_0), 6)
    }

    #[test]
    fn test_part1_1() {
        let input_1 = parse(INPUT_1);
        assert_eq!(part1(&input_1), 16)
    }

    #[test]
    fn test_part1_2() {
        let input_2 = parse(INPUT_2);
        assert_eq!(part1(&input_2), 12)
    }

    #[test]
    fn test_part1_3() {
        let input_3 = parse(INPUT_3);
        assert_eq!(part1(&input_3), 23)
    }

    #[test]
    fn test_part1_4() {
        let input_4 = parse(INPUT_4);
        assert_eq!(part1(&input_4), 31)
    }

    #[test]
    fn test_part1_5() {
        let input_5 = parse(INPUT_5);

        assert_eq!(part1(&input_5), 9)
    }

    #[test]
    fn test_part2_0() {
        let input = parse("C200B40A82");
        assert_eq!(part2(&input), 3);
    }

    #[test]
    fn test_part2_1() {
        let input = parse("04005AC33890");
        assert_eq!(part2(&input), 54);
    }

    #[test]
    fn test_part2_2() {
        let input = parse("880086C3E88112");
        assert_eq!(part2(&input), 7);
    }
}

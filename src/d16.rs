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

    #[test]
    fn test_part1() {
        let input = parse("D2FE28");
        assert_eq!(part1(&input), 6);

        let input = parse("8A004A801A8002F478");
        assert_eq!(part1(&input), 16);

        let input = parse("620080001611562C8802118E34");
        assert_eq!(part1(&input), 12);

        let input = parse("C0015000016115A2E0802F182340");
        assert_eq!(part1(&input), 23);

        let input = parse("A0016C880162017C3686B18A3D4780");
        assert_eq!(part1(&input), 31);

        let input = parse("38006F45291200");
        assert_eq!(part1(&input), 9);
    }

    #[test]
    fn test_part2() {
        let input = parse("C200B40A82");
        assert_eq!(part2(&input), 3);

        let input = parse("04005AC33890");
        assert_eq!(part2(&input), 54);

        let input = parse("880086C3E88112");
        assert_eq!(part2(&input), 7);

        let input = parse("CE00C43D881120");
        assert_eq!(part2(&input), 9);

        let input = parse("D8005AC2A8F0");
        assert_eq!(part2(&input), 1);

        let input = parse("F600BC2D8F");
        assert_eq!(part2(&input), 0);

        let input = parse("9C005AC2F8F0");
        assert_eq!(part2(&input), 0);

        let input = parse("9C0141080250320F1802104A08");
        assert_eq!(part2(&input), 1);
    }
}

pub fn run(line: &str) -> (usize, usize) {
    let line = parse(line);

    (part1(&line), part2(&line))
}

fn part1(line: &str) -> usize {
    parse_packet(line).0
}

fn part2(line: &str) -> usize {
    parse_packet(line).2
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
                0 => values.iter().sum(),
                1 => values.iter().product(),
                2 => *values.iter().min().unwrap(),
                3 => *values.iter().max().unwrap(),
                5 => (values[0] > values[1]) as usize,
                6 => (values[0] < values[1]) as usize,
                7 => (values[0] == values[1]) as usize,
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

    fn parse_type_id(p: &str) -> (u8, usize) {
        (u8::from_str_radix(&p[0..3], 2).unwrap(), 3)
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

        (0, read, usize::from_str_radix(number.as_str(), 2).unwrap())
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

fn parse(line: &str) -> String {
    line.trim()
        .chars()
        .map(|c| c.to_digit(16).unwrap())
        .map(|c| format!("{:04b}", c))
        .collect::<String>()
}

#[cfg(test)]
mod tests {

    use test::Bencher;

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

    #[bench]
    fn bench_parse(b: &mut Bencher) {
        let input = std::fs::read_to_string("input/d16.txt").expect("file not found");

        b.iter(|| parse(input.as_str()));
    }

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        let input = std::fs::read_to_string("input/d16.txt").expect("file not found");
        let line = parse(input.as_str());
        b.iter(|| part1(&line));
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        let input = std::fs::read_to_string("input/d16.txt").expect("file not found");
        let line = parse(input.as_str());
        b.iter(|| part2(&line));
    }
}

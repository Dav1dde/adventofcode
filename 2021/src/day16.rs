use aoc2021::Input;
use itertools::Itertools;
use nom::bits::complete::take;
use nom::IResult;
use std::io::Read;

type Bits<'a> = (&'a [u8], usize);

fn hex_value(v: u8) -> u8 {
    match v {
        b'0'..=b'9' => v - b'0',
        b'A'..=b'F' => v - b'A' + 10,
        _ => panic!("invalid hex digit {}", v),
    }
}

fn parse_literal(mut input: Bits) -> IResult<Bits, (usize, u64)> {
    let mut result = 0;
    let mut size = 0;
    loop {
        let (inp, ctrl): (Bits, u8) = take(1usize)(input)?;
        let (inp, number): (Bits, u64) = take(4usize)(inp)?;

        result = result << 4 | number;
        size += 5;

        input = inp;
        if ctrl == 0 {
            break Ok((input, (size, result)));
        }
    }
}

#[derive(Debug, PartialEq)]
enum Operator {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}

impl Operator {
    fn evaluate(&self, packets: &[Packet]) -> u64 {
        let values = packets.iter().map(|packet| packet.evaluate());
        match self {
            Self::Sum => values.sum(),
            Self::Product => values.reduce(|accum, value| accum * value).unwrap(),
            Self::Minimum => values.min().unwrap(),
            Self::Maximum => values.max().unwrap(),
            Self::GreaterThan => {
                if packets[0].evaluate() > packets[1].evaluate() {
                    1
                } else {
                    0
                }
            }
            Self::LessThan => {
                if packets[0].evaluate() < packets[1].evaluate() {
                    1
                } else {
                    0
                }
            }
            Self::EqualTo => {
                if packets[0].evaluate() == packets[1].evaluate() {
                    1
                } else {
                    0
                }
            }
        }
    }
}

impl TryFrom<u32> for Operator {
    type Error = anyhow::Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        let r = match value {
            0 => Self::Sum,
            1 => Self::Product,
            2 => Self::Minimum,
            3 => Self::Maximum,
            5 => Self::GreaterThan,
            6 => Self::LessThan,
            7 => Self::EqualTo,
            _ => anyhow::bail!("invalid operator {}", value),
        };
        Ok(r)
    }
}

#[derive(Debug, PartialEq)]
enum Packet {
    Literal {
        version: u8,
        value: u64,
    },
    Operator {
        version: u8,
        operator: Operator,
        packets: Vec<Packet>,
    },
}

impl Packet {
    fn version(&self) -> u8 {
        match self {
            Self::Literal { version, .. } => *version,
            Self::Operator { version, .. } => *version,
        }
    }

    fn sum_version(&self) -> usize {
        match self {
            Self::Operator {
                version, packets, ..
            } => (*version as usize) + packets.iter().map(|p| p.sum_version()).sum::<usize>(),
            _ => self.version() as usize,
        }
    }

    fn evaluate(&self) -> u64 {
        match self {
            Self::Literal { value, .. } => *value,
            Self::Operator {
                operator, packets, ..
            } => operator.evaluate(packets),
        }
    }
}

fn parse_packet(input: Bits) -> IResult<Bits, (usize, Packet)> {
    let (input, version) = take(3usize)(input)?;
    let (input, type_id) = take(3usize)(input)?;

    match type_id {
        4 => {
            let (input, (size, value)) = parse_literal(input)?;
            Ok((input, (size + 6, Packet::Literal { version, value })))
        }
        type_id => {
            let operator = type_id.try_into().unwrap();
            let (input, length_type_id): (Bits, usize) = take(1usize)(input)?;
            let length_size = 15usize - (length_type_id << 2);
            let (mut input, length): (Bits, usize) = take(length_size)(input)?;
            let mut packets = vec![];
            let mut sub_size = 0;
            for i in 0.. {
                let (inp, (size, packet)) = parse_packet(input)?;
                sub_size += size;
                input = inp;
                packets.push(packet);

                if length_type_id == 0 && sub_size >= length {
                    assert_eq!(sub_size, length);
                    break;
                } else if length_type_id == 1 && i + 1 >= length {
                    break;
                }
            }
            let size = 6 + 1 + length_size + sub_size;
            Ok((
                input,
                (
                    size,
                    Packet::Operator {
                        version,
                        operator,
                        packets,
                    },
                ),
            ))
        }
    }
}

fn parse_input(input: &[u8]) -> IResult<&[u8], Packet> {
    let (_, packet) = parse_packet((input, 0)).unwrap();
    Ok((input, packet.1))
}

fn read(reader: Input) -> Vec<u8> {
    reader
        .bytes()
        .map(|b| b.unwrap())
        .tuples()
        .map(|(a, b)| hex_value(a) << 4 | hex_value(b))
        .collect::<Vec<_>>()
}

pub fn part1(reader: Input) -> anyhow::Result<u64> {
    Ok(parse_input(&read(reader)).unwrap().1.sum_version() as u64)
}

pub fn part2(reader: Input) -> anyhow::Result<u64> {
    Ok(parse_input(&read(reader)).unwrap().1.evaluate())
}

pub fn main() {
    aoc2021::cli::run(part1, part2).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_literal() {
        assert_eq!(
            parse_input(b"D2FE28").unwrap().1,
            Packet::Literal {
                version: 6,
                value: 2021
            }
        );
    }

    #[test]
    fn test_parse_operator() {
        assert_eq!(
            parse_input(b"38006F45291200").unwrap().1,
            Packet::Operator {
                version: 1,
                operator: Operator::LessThan,
                packets: vec![
                    Packet::Literal {
                        version: 6,
                        value: 10
                    },
                    Packet::Literal {
                        version: 2,
                        value: 20
                    },
                ]
            }
        );
    }

    #[test]
    fn test_sum() {
        assert_eq!(
            parse_input(b"8A004A801A8002F478").unwrap().1.sum_version(),
            16
        );
        assert_eq!(
            parse_input(b"620080001611562C8802118E34")
                .unwrap()
                .1
                .sum_version(),
            12
        );
        assert_eq!(
            parse_input(b"C0015000016115A2E0802F182340")
                .unwrap()
                .1
                .sum_version(),
            23
        );
        assert_eq!(
            parse_input(b"A0016C880162017C3686B18A3D4780")
                .unwrap()
                .1
                .sum_version(),
            31
        );
    }

    #[test]
    fn test_evaluate() {
        assert_eq!(parse_input(b"C200B40A82").unwrap().1.evaluate(), 3);
        assert_eq!(parse_input(b"04005AC33890").unwrap().1.evaluate(), 54);
        assert_eq!(parse_input(b"880086C3E88112").unwrap().1.evaluate(), 7);
        assert_eq!(parse_input(b"CE00C43D881120").unwrap().1.evaluate(), 9);
        assert_eq!(parse_input(b"D8005AC2A8F0").unwrap().1.evaluate(), 1);
        assert_eq!(parse_input(b"F600BC2D8F").unwrap().1.evaluate(), 0);
        assert_eq!(parse_input(b"9C005AC2F8F0").unwrap().1.evaluate(), 0);
        assert_eq!(
            parse_input(b"9C0141080250320F1802104A08")
                .unwrap()
                .1
                .evaluate(),
            1
        );
    }
}

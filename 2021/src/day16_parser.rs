use nom::bytes::complete::tag;
use nom::combinator::{consumed, map_res};
use nom::error::make_error;
use nom::multi::{fold_many0, many_m_n};
use nom::sequence::preceded;
use nom::IResult;
use nom::bytes::complete::take;
use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
pub struct PacketMetadata {
    pub version: u8,
    pub type_id: u8,
}

// TODO(keytoaster): Revisit the struct structure.
#[derive(Debug, PartialEq)]
pub enum Packet {
    Literal {
        metadata: PacketMetadata,
        packet: LiteralPacket,
    },
    Sum {
        metadata: PacketMetadata,
        packet: SumPacket,
    },
    Product {
        metadata: PacketMetadata,
        packet: ProductPacket,
    },
    Minimum {
        metadata: PacketMetadata,
        packet: MinimumPacket,
    },
    Maximum {
        metadata: PacketMetadata,
        packet: MaximumPacket,
    },
    GreaterThan {
        metadata: PacketMetadata,
        packet: GreaterThanPacket,
    },
    LessThan {
        metadata: PacketMetadata,
        packet: LessThanPacket,
    },
    EqualTo {
        metadata: PacketMetadata,
        packet: EqualToPacket,
    },
}

#[derive(Debug, PartialEq)]
pub struct LiteralPacket {
    pub value: u64,
}

#[derive(Debug, PartialEq)]
pub struct SumPacket {
    pub subpackets: Vec<Packet>,
}

#[derive(Debug, PartialEq)]
pub struct ProductPacket {
    pub subpackets: Vec<Packet>,
}

#[derive(Debug, PartialEq)]
pub struct MinimumPacket {
    pub subpackets: Vec<Packet>,
}

#[derive(Debug, PartialEq)]
pub struct MaximumPacket {
    pub subpackets: Vec<Packet>,
}

#[derive(Debug, PartialEq)]
pub struct GreaterThanPacket {
    pub subpackets: Vec<Packet>,
}

#[derive(Debug, PartialEq)]
pub struct LessThanPacket {
    pub subpackets: Vec<Packet>,
}

#[derive(Debug, PartialEq)]
pub struct EqualToPacket {
    pub subpackets: Vec<Packet>,
}

pub fn hex2bin(s: &str) -> Result<String, ParseIntError> {
    s.chars()
        .map(|c| u8::from_str_radix(&c.to_string(), 16))
        .map(|u| u.and_then(|u| Ok(format!("{:04b}", u))))
        .collect()
}

fn bin2dec(input: &str) -> Result<u32, ParseIntError> {
    u32::from_str_radix(input, 2)
}

pub fn parse_version(input: &str) -> Result<u8, ParseIntError> {
    u8::from_str_radix(input, 2)
}

pub fn parse_type_id(input: &str) -> Result<u8, ParseIntError> {
    u8::from_str_radix(input, 2)
}

// fn literal_value_partial(input: &str) -> IResult<&str, String> {
//     alt((
//         map(preceded(tag("0"), take(4usize)), |s: &str| s.to_string()),
//         map(
//             preceded(tag("1"), tuple((take(4usize), literal_value_partial))),
//             |(bits, rest)| bits.to_string() + &rest,
//         ),
//     ))(input)
// }

fn literal_value_partial(input: &str) -> IResult<&str, String> {
    let (input, mut partial) = fold_many0(
        preceded(tag("1"), take(4usize)),
        String::new,
        |mut acc: String, item| {
            acc.push_str(item);
            acc
        },
    )(input)?;

    let (input, partial2) = preceded(tag("0"), take(4usize))(input)?;
    partial.push_str(partial2);

    Ok((input, partial))
}

pub fn literal_packet(input: &str) -> IResult<&str, LiteralPacket> {
    let (input, literal_value) =
        map_res(literal_value_partial, |s| u64::from_str_radix(&s, 2))(input)?;

    Ok((
        input,
        LiteralPacket {
            value: literal_value,
        },
    ))
}

pub fn subpackets(input: &str) -> IResult<&str, Vec<Packet>> {
    let (input, length_type_id) = take(1usize)(input)?;

    match length_type_id {
        "0" => {
            let (mut input, length) = take(15usize)(input)?;
            let mut length = bin2dec(length).unwrap();

            let mut retval = Vec::new();

            while length > 0 {
                let c = consumed(packet)(input)?;
                input = c.0;
                let (consumed, subpacket) = c.1;
                retval.push(subpacket);
                length -= consumed.len() as u32;
            }
            assert!(length == 0, "length inconsistent");

            Ok((input, retval))
        }
        "1" => {
            let (input, num_subpackets) = take(11usize)(input)?;
            let num_subpackets = bin2dec(num_subpackets).unwrap();

            let (input, retval) =
                many_m_n(num_subpackets as usize, num_subpackets as usize, packet)(input)?;

            Ok((input, retval))
        }
        _ => {
            panic!("unexpected bit value")
        }
    }
}

pub fn packet(input: &str) -> IResult<&str, Packet> {
    let (input, version) = map_res(take(3usize), parse_version)(input)?;
    let (input, type_id) = map_res(take(3usize), parse_type_id)(input)?;

    let metadata = PacketMetadata { version, type_id };

    match type_id {
        4 => {
            let (input, literal_packet) = literal_packet(input)?;
            return Ok((
                input,
                Packet::Literal {
                    metadata: metadata,
                    packet: literal_packet,
                },
            ));
        },
        0 => {
            let (input, sp) = subpackets(input)?;
            return Ok((
                input,
                Packet::Sum {
                    metadata: metadata,
                    packet: SumPacket { subpackets: sp },
                },
            ));
        },
        1 => {
            let (input, sp) = subpackets(input)?;
            return Ok((
                input,
                Packet::Product {
                    metadata: metadata,
                    packet: ProductPacket { subpackets: sp },
                },
            ));
        },
        2 => {
            let (input, sp) = subpackets(input)?;
            return Ok((
                input,
                Packet::Minimum {
                    metadata: metadata,
                    packet: MinimumPacket { subpackets: sp },
                },
            ));
        },
        3 => {
            let (input, sp) = subpackets(input)?;
            return Ok((
                input,
                Packet::Maximum {
                    metadata: metadata,
                    packet: MaximumPacket { subpackets: sp },
                },
            ));
        },
        5 => {
            let (input, sp) = subpackets(input)?;
            return Ok((
                input,
                Packet::GreaterThan {
                    metadata: metadata,
                    packet: GreaterThanPacket { subpackets: sp },
                },
            ));
        },
        6 => {
            let (input, sp) = subpackets(input)?;
            return Ok((
                input,
                Packet::LessThan {
                    metadata: metadata,
                    packet: LessThanPacket { subpackets: sp },
                },
            ));
        },
        7 => {
            let (input, sp) = subpackets(input)?;
            return Ok((
                input,
                Packet::EqualTo {
                    metadata: metadata,
                    packet: EqualToPacket { subpackets: sp },
                },
            ));
        },
        _ => {
            // Not sure what to do about this. I just want an error.
            Err(nom::Err::Error(make_error(
                input,
                nom::error::ErrorKind::Fail,
            )))
        }
    }
}

#[test]
fn test_literal() -> Result<(), Box<dyn Error>> {
    let p = hex2bin("D2FE28")?;
    let (r, p) = packet(&p).unwrap();

    assert_eq!(r, "000");
    match p {
        Packet::Literal { packet: p, .. } => {
            assert_eq!(p.value, 2021);
            Ok(())
        }
        _ => Err("not a Literal packet".into()),
    }
}

#[test]
fn test_operator_len_type_0() -> Result<(), Box<dyn Error>> {
    let p = hex2bin("38006F45291200")?;
    let (r, p) = packet(&p).unwrap();

    assert_eq!(r, "0000000");
    match p {
        Packet::LessThan { packet: p, .. } => match &p.subpackets[..] {
            [Packet::Literal {
                packet: LiteralPacket { value: 10 },
                ..
            }, Packet::Literal {
                packet: LiteralPacket { value: 20 },
                ..
            }] => Ok(()),
            _ => Err("subpackets incorrect".into()),
        },
        _ => Err("wrong packet type".into()),
    }
}

#[test]
fn test_operator_len_type_1() -> Result<(), Box<dyn Error>> {
    let p = hex2bin("EE00D40C823060")?;
    let (r, p) = packet(&p).unwrap();

    assert_eq!(r, "00000");
    match p {
        Packet::Maximum { packet: p, .. } => match &p.subpackets[..] {
            [Packet::Literal {
                packet: LiteralPacket { value: 1 },
                ..
            }, Packet::Literal {
                packet: LiteralPacket { value: 2 },
                ..
            }, Packet::Literal {
                packet: LiteralPacket { value: 3 },
                ..
            }] => Ok(()),
            _ => Err("subpackets incorrect".into()),
        },
        _ => Err("wrong packet type".into()),
    }
}

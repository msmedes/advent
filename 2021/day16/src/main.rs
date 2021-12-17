// first 3 bits: version
// first 3 bits: type id

// type id 4 is a literal
// padded wth leading zeros until it's length is a multiple of 4 bits
// then grouped into groups of four bits
// each group is prefaced by  1 except the last which is prefaced by 0

enum LengthTypeID {
    SubPacketLength,
    NumSubPackets,
}

enum PacketType {
    Literal,
    Operator,
}

fn hex_to_binary(s: &str) -> String {
    format!("{:04b}", u8::from_str_radix(s, 16).unwrap())
}

fn binary_from_hex(s: &str) -> String {
    let mut binary = String::new();
    for c in s.chars() {
        binary.push_str(&hex_to_binary(&c.to_string()));
    }
    binary
}

fn int_from_binary(s: &str) -> u32 {
    u32::from_str_radix(s, 2).unwrap()
}

fn parse_literal(s: &str) -> Option<(u32, usize)> {
    let mut literal = String::new();
    let mut breakpoint = 0;
    for i in (0..s.len()).step_by(5) {
        literal.push_str(&s[i + 1..i + 5]);
        if s.chars().nth(i).unwrap() == '0' {
            breakpoint = i + 5;
            break;
        }
    }
    if literal.len() == 0 {
        return None;
    }
    Some((int_from_binary(&literal), breakpoint))
}

fn parse_packet(s: &str) -> u32 {
    let version_number = int_from_binary(&s[..3]);
    let type_id = int_from_binary(&s[3..6]);
    let mut sub_packet_version_numbers = 0;
    if type_id == 4 {
        let (_, packet_end) = parse_literal(&s[6..]).unwrap();
    } else {
        let length_type_id = int_from_binary(&s[6..7]);
        if length_type_id == 0 {
        } else {
            let num_subpackets = int_from_binary(&s[7..18]);
            sub_packet_version_numbers += parse_packet()
        }
    }
    version_number + sub_packet_version_numbers
}

fn main() {
    let test = "D2FE28";
    assert_eq!(binary_from_hex(test), "110100101111111000101000");
    assert_eq!(int_from_binary("110"), 6);
    assert_eq!(parse_packet(&binary_from_hex(test)), 2021);
}

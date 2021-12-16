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

fn main() {
    println!("Hello, world!");
}

use rustc_hash::FxHashSet;

fn main() {
    let key1 = 2959251;
    let key2 = 4542595;

    let handshake2 = handshake(key2);

    println!("{}", transform(key1, handshake2));
}

fn handshake(key: usize) -> usize {
    let subject = 7;
    let mut loop_size = 0;
    let mut val = 1;

    while val != key {
        val *= subject;
        val = val % 20201227;
        loop_size += 1
    }
    loop_size
}

fn transform(subject: usize, loop_size: usize) -> usize {
    let mut val = 1;

    for _ in 0..loop_size {
        val = val * subject;
        val = val % 20201227;
    }
    val
}

fn enc_loop(subject_number: usize, value: usize) -> usize {
    let curr = value * subject_number;
    let curr = curr % 20201227;
    curr
}

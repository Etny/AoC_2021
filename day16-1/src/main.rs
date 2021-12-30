mod biterator;
mod input;

use biterator::*;
use input::*;

fn main() {
    let mut bits = Biterator::new(INPUT);

    let output = parse_packet(&mut bits);
    println!("{}", output);
}

fn parse_packet(bits: &mut Biterator) -> u32 {
    let mut version = join_bits(bits.take(3));
    let type_id = join_bits(bits.take(3));

    if type_id == 4 {
        parse_literal_packet(bits);
    } else {
        version += parse_operator_packet(bits)
    }

    version
}

fn parse_operator_packet(bits: &mut Biterator) -> u32 {
    let mut out = 0;
    let mode = bits.next().unwrap();

    if mode == 0 {
        let len_bits = bits.take(15).collect::<Vec<_>>();
        let len = join_bits(len_bits.into_iter()) as usize;

        let mut subpackets = Biterator::from_iter(bits.take(len));

        while !subpackets.done() {
            out += parse_packet(&mut subpackets);
        }
    } else {
        let len = join_bits(bits.take(11));

        for _ in 0..len {
            out += parse_packet(bits);
        }
    }

    out
}

fn parse_literal_packet(bits: &mut Biterator) {
    let mut all_bits = vec![];

    loop {
        let first = bits.next().unwrap();

        all_bits.extend(bits.take(4));

        if first == 0 {
            break;
        }
    }
}

fn join_bits<T>(mut bits: T) -> u32
where
    T: Iterator<Item = u8>,
{
    let mut out = 0;

    while let Some(bit) = bits.next() {
        out = (out << 1) + (bit as u32);
    }

    out
}

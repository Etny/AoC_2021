mod biterator;
mod input;

use biterator::*;
use input::*;


//While the puzzle for this one wasn't very hard, I had a very rough sparring match with rust's iterators while working on this one. (I lost, so this solution is kinda bulky)
fn main() {
    let mut bits = Biterator::new(INPUT);

    let output = parse_packet(&mut bits);
    println!("{}", output);
}

fn parse_packet<T>(bits: &mut T) -> u64 
where
    T: Iterator<Item=u8>
{
    let _version = join_bits(bits.take(3));
    let type_id = join_bits(bits.take(3));

    if type_id == 4 {
        parse_literal_packet(bits)
    } else {
        parse_operator_packet(bits, type_id)
    }

}

fn parse_operator_packet<T>(bits: &mut T, id: u64) -> u64 
where
    T: Iterator<Item=u8>
{
    let mode = bits.next().unwrap();
    let mut operants = vec![];

    if mode == 0 {
        let len = join_bits(bits.take(15)) as usize;
        let mut iter = bits.take(len).collect::<Vec<_>>().into_iter().peekable();

        while iter.peek().is_some() {
            operants.push(parse_packet(&mut iter))
        }
        
    } else {
        let len = join_bits(bits.take(11));

        for _ in 0..len {
            operants.push(parse_packet(bits));
        }
    }

    perform_op(operants, id)
}

fn parse_literal_packet<T>(bits: &mut T) -> u64 
where
    T: Iterator<Item=u8>
{
    let mut all_bits = vec![];

    loop {
        let first = bits.next().unwrap();

        all_bits.extend(bits.take(4));

        if first == 0 {
            break;
        }
    }

    join_bits(all_bits)
}

fn perform_op(operants: Vec<u64>, op_id: u64) -> u64 {
    match op_id {
        0 => operants.iter().sum(),
        1 => operants.iter().product(),
        2 => *operants.iter().min().unwrap(),
        3 => *operants.iter().max().unwrap(),
        5 => if operants[0] > operants[1] { 1 } else { 0 }
        6 => if operants[0] < operants[1] { 1 } else { 0 }
        7 => if operants[0] == operants[1] { 1 } else { 0 }
        _ => 0
    }
}


fn join_bits<T>(bits: T) -> u64
where
    T: IntoIterator<Item = u8>,
{
    let mut out = 0;

    let mut iter = bits.into_iter();

    while let Some(bit) = iter.next() {
        out = (out << 1) + (bit as u64);
    }

    out
}

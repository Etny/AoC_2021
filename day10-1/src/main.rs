mod input;

use input::*;

fn main() {
    let output: u32 = INPUT.trim().lines().map(|l| {
        let mut iter = l.trim().chars();
        let first = iter.next().unwrap();
        parse_chunk(&mut iter, closing(first).unwrap())
    }).sum();

    println!("{}", output);
}

fn parse_chunk<T>(chars: &mut T, close: char) -> u32
    where T: Iterator<Item=char>
{
    while let Some(c) = chars.next() {
        if c == close { return 0; }

        if let Some(close) = closing(c) {
            let inner = parse_chunk(chars, close);
            if inner != 0 { return inner; }
        } else {
            return error(c);
        }
    }

    0
}

fn closing(c: char) -> Option<char> {
    match c {
        '(' => Some(')'),
        '[' => Some(']'),
        '{' => Some('}'),
        '<' => Some('>'),
        _ => None
    }
}

fn error(c: char) -> u32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!()
    }
}

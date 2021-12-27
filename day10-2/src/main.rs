mod input;

use input::*;

fn main() {
    let mut errors: Vec<_> = INPUT
        .trim()
        .lines()
        .map(|l| parse_line(l.trim().chars()))
        .filter(|r| *r != 0)
        .collect();

    errors.sort();

    let output = errors[(errors.len() - 1) / 2];
    println!("{}", output);
}

fn parse_line<T>(mut chars: T) -> u64
where
    T: Iterator<Item = char>,
{
    while let Some(first) = &mut chars.next() {
        if let Some(close) = closing(*first) {
            if let Ok(res) = parse_chunk(&mut chars, close) {
                if res != 0 {
                    return res;
                }
                continue;
            }
        }
        break;
    }
    
    0
}

fn parse_chunk<T>(chars: &mut T, close: char) -> Result<u64, ()>
where
    T: Iterator<Item = char>,
{
    let mut res: u64 = 0;

    while let Some(c) = chars.next() {
        if c == close {
            return Ok(0);
        }

        if let Some(close) = closing(c) {
            res = parse_chunk(chars, close)?;
        } else {
            return Err(());
        }
    }

    Ok((res * 5) + error(close))
}

fn closing(c: char) -> Option<char> {
    match c {
        '(' => Some(')'),
        '[' => Some(']'),
        '{' => Some('}'),
        '<' => Some('>'),
        _ => None,
    }
}

fn error(c: char) -> u64 {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => panic!(),
    }
}

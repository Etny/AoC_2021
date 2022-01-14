mod input;
mod scanner;

use input::*;
use scanner::*;

fn main() {
    let mut scanners = vec![];

    let mut iter = INPUT.trim().lines().peekable();

    while let Some(_) = iter.peek() {
        scanners.push(Scanner::from_dec(&mut iter));
    }

    let orientations = orientations();


    let mut first = scanners.remove(0);

    while !scanners.is_empty() {
        let mut next = vec![];

        for other in scanners {
            if first.compare(&other, orientations.clone()) == 0 { next.push(other);}
        }

        scanners = next;
    }

    
    println!("{}", first.beacons.len());
}

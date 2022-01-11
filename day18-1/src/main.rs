mod input;
use std::iter::Peekable;

use input::*;

fn main() {

    let mut nums = vec![];
    
    for l in INPUT.trim().lines() {
       nums.push(SnailNum::from_dec(&mut l.chars().peekable()));
    }

    let add = nums.into_iter().reduce(|acc, elem| acc.add(elem).reduce()).unwrap();
    println!("{}", add.magnitude());

}

enum SnailNum {
    Pair { left: Box<SnailNum>, right: Box<SnailNum> },
    Flat(i32)
}

impl SnailNum {

    fn add(self, rhs: Self) -> Self {
        Self::Pair { left: Box::new(self), right: Box::new(rhs)}
    }

    fn reduce(self) -> Self{
        let mut changed = true;
        let mut result = self;

        while changed {
            changed = false;
            result.explode(0, &mut None, &mut 0, &mut changed);
            if !changed { result.split(&mut changed); }
        }

        result
    }

    fn explode<'a>(&'a mut self, nesting: u32, last_flat: &mut Option<&'a mut Self>, carry: &mut i32, done: &mut bool) {
        match self {
            Self::Pair { .. } => {
                if nesting < 4 || *done {
                    if let Self::Pair { left, right } = self {
                        left.explode(nesting + 1, last_flat, carry, done);
                        right.explode(nesting + 1, last_flat, carry, done);
                    } 
                } else {
                    let (val_left, val_right) = match self {
                        Self::Pair{ left, right } => {
                            (match left.as_ref() { Self::Flat(vl) => *vl,_ => panic!()},
                            match right.as_ref() { Self::Flat(vr) => *vr, _ => panic!()})
                        }
                        _ => panic!()
                    };

                    if let Some(flat) = last_flat {
                        if let Self::Flat(val) = **flat {
                            **flat = Self::Flat(val + val_left);
                        }
                    }

                    *carry = val_right;

                    *self = Self::Flat(0);
                    *done = true;
                }
            }
            Self::Flat(val) => {
                if *carry != 0 {
                    *self = Self::Flat(*val + *carry);
                    *carry = 0;
                }
                *last_flat = Some(self)
            }
        }

    }

    fn split(&mut self, done: &mut bool) {
        match self {

            Self::Pair { left, right } => {
                left.split(done);
                right.split(done);
            }
            
            Self::Flat(val) => {
                if *val >= 10 && !*done {
                    let left = Box::new(Self::Flat(*val / 2));
                    let right = Box::new(Self::Flat(*val / 2 + if (*val).abs() % 2 == 0 { 0 } else { 1 }));

                    *self = Self::Pair { left, right };
                    *done = true;
                }
            }
        }
    }

    fn magnitude(&self) -> i32 {
        match self {
            Self::Flat(val) => *val,
            Self::Pair{ left, right} => 3 * left.magnitude() + 2 * right.magnitude()
        }
    }

    pub fn from_dec<T>(dec: &mut Peekable<T>) -> Self
        where T: Iterator<Item=char>
    {
        match dec.peek() {
            Some('[') => {
                dec.next().unwrap();
                let left = Box::new(Self::from_dec(dec));
                let right = Box::new(Self::from_dec(dec));

                while dec.peek() == Some(&']') || dec.peek() == Some(&',') {
                    let _ = dec.next().unwrap();
                }

                Self::Pair { left, right }
            }   
            Some(_) => {
                let num = dec.take_while(|n| n.is_alphanumeric() || *n == '-').collect::<String>();
                Self::Flat(num.parse().unwrap())
            }
            _ => panic!()
        }
    }
}

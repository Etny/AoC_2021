use std::collections::HashMap;
use crate::input::*;


//This is a input-agnostic brute-force solution. 
pub fn solve_brute(do_min: bool) {
    
    let mut blocks = vec![];
    let mut current_block = vec![];

    for line in INPUT.trim().lines().skip(1) {
        if line.contains("inp ") {
            blocks.push(current_block);
            current_block = vec![];
        } else {
            current_block.push(Instruction::new(&line.split(' ').collect::<Vec<_>>()))
        }
    }

    blocks.push(current_block);

    let mut results = vec![];

    let default = if do_min { i64::MAX } else { 0 };
    let compare = if do_min { i64::min } else { i64::max };

    let mut poss: HashMap<i64, i64> = HashMap::new();

    poss.insert(0, 0);

    for i in 0..blocks.len() {
        let mut next = HashMap::new();

        for j in poss.keys() {
            for k in 1..10 {
                let mut reg = [0i64; 4];
                reg[0] = k;
                reg[3] = *j;

                for inst in &blocks[i] {
                    inst.run(&mut reg);
                }

                if i == blocks.len() - 1 {
                    if reg[3] == 0 {
                       results.push((poss[j] * 10) + k);
                    }
                } else {
                    let other = if next.contains_key(&reg[3]) { next[&reg[3]] } else { default };
                    next.insert(reg[3], compare(other, (poss[j] * 10) + k));
                }
            }
        }

        poss = next;
    }

    println!("{}", results.iter().min().unwrap());

}

struct Instruction {
    op: fn(i64, i64) -> i64,
    lhs: Operant,
    rhs: Operant
}

impl Instruction {

    fn new(dec: &[&str]) -> Self {
        Self {
            op: to_op(dec[0]),
            lhs: Operant::from(dec[1]),
            rhs: Operant::from(dec[2])
        }
    }

    fn run(&self, registers: &mut [i64; 4]) {
        let index = match self.lhs {
            Operant::Register(i) => i,
            _ => panic!()
        };

        let rhs = match self.rhs {
            Operant::Register(i) => registers[i],
            Operant::Const(i) => i
        };

        registers[index] = (self.op)(registers[index], rhs);
    }
}

enum Operant {
    Const(i64),
    Register(usize)
}

impl Operant {
    fn from(dec: &str) -> Self {
        match dec {
            "w" => Self::Register(0),
            "x" => Self::Register(1),
            "y" => Self::Register(2),
            "z" => Self::Register(3),
            _ => Self::Const(dec.parse().unwrap())
        }
    }
}

fn to_op(dec: &str) -> fn(i64, i64) -> i64 {

    match dec {
        "add" => <i64 as core::ops::Add>::add,
        "mul" => <i64 as core::ops::Mul>::mul,
        "div" => <i64 as core::ops::Div>::div,
        "mod" => <i64 as core::ops::Rem>::rem,
        "eql" => eql_op,
        _ => panic!()
    }
}

fn eql_op(lhs: i64, rhs: i64) -> i64 {
    if lhs == rhs { 1 } else { 0 }
}
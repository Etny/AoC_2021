mod input;

use std::{
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
};

use input::*;

fn main() {
    let mut from_name = HashMap::new();

    for line in INPUT.trim().lines() {
        let caves: Vec<_> = line.split('-').collect();

        for name in &caves {
            if from_name.contains_key(name) {
                continue;
            }
            let cave = Cave {
                paths: vec![],
                size: CaveSize::from_name(name),
                name: name.to_string()
            };
            from_name.insert(*name, Rc::new(RefCell::new(cave)));
        }

        from_name[caves[0]]
            .borrow_mut()
            .paths
            .push(Rc::clone(&from_name[caves[1]]));
        from_name[caves[1]]
            .borrow_mut()
            .paths
            .push(Rc::clone(&from_name[caves[0]]));
    }

    let output = from_name["start"].borrow().get_paths(&mut vec![], 0, false);
    println!("{}", output);
}

#[derive(PartialEq, Eq)]
enum CaveSize {
    Small,
    Large,
    Start,
    End,
}

struct Cave {
    pub paths: Vec<Rc<RefCell<Cave>>>,
    size: CaveSize,
    name: String
}

impl Cave {

    pub fn get_paths(&self, visited: &mut Vec<String>, mut acc: u32, mut double: bool) -> u32 {

        visited.push(self.name.clone());

        if self.size == CaveSize::End {
            return acc + 1;
        }

        for path in &self.paths {
            let cave = path.borrow();

            let mut reset_double = false;

            if visited.contains(&cave.name) { 
                match cave.size {
                    CaveSize::Large => (),
                    CaveSize::Small => {
                        if double {
                            continue;
                        } else {
                            double = true;
                            reset_double = true;
                        }
                    }
                    _ => continue
                };
            }

            acc = cave.get_paths(visited, acc, double);

            if reset_double { double = false; }
            visited.pop();
        }
        
        acc
    }
}

impl CaveSize {
    pub fn from_name(name: &str) -> CaveSize {
        match name {
            "end" => CaveSize::End,
            "start" => CaveSize::Start,
            a if a.to_ascii_uppercase() == a => CaveSize::Large,
            _ => CaveSize::Small,
        }
    }
}

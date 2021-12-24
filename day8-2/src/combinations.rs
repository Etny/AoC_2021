use std::borrow::Borrow;

pub struct Combinations<T> {
    elements: Vec<T>,
    index: Vec<usize>,
    finished: bool,
}

impl<T: Clone> Combinations<T> {
    pub fn new<V: Borrow<[T]>>(list: V) -> Self {
        let l = list.borrow();
        Combinations {
            elements: l.to_vec(),
            index: vec![0; l.len() - 1],
            finished: false,
        }
    }
}

impl<T: Clone> Iterator for Combinations<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        let mut vec = self.elements.clone();
        let mut output = vec![];

        for i in 0..self.elements.len() - 1 {
            output.push(vec.remove(self.index[i]));
        }

        for j in 0..self.index.len() {
            let i = self.index.len() - 1 - j;
            self.index[i] += 1;
            if self.index[i] > j + 1 {
                self.index[i] = 0;
                if i == 0 {
                    self.finished = true;
                }
            } else {
                break;
            }
        }

        output.push(vec.remove(0));

        Some(output)
    }
}

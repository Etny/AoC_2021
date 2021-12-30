use crate::join_bits;

#[derive(Clone)]
pub struct Biterator {
    bytes: Vec<u8>,
    size: usize,
    bit_index: u8,
}

impl Biterator {
    pub fn new(hex: &str) -> Self {
        let mut bytes = vec![];

        for byte in hex.chars().collect::<Vec<_>>().chunks(2) {
            bytes.push(u8::from_str_radix(&byte.iter().collect::<String>(), 16).unwrap());
        }
        let size = bytes.len() * 8;

        Biterator {
            bytes,
            size,
            bit_index: 0,
        }
    }

    pub fn from_iter<T>(iter: T) -> Self
    where
        T: Iterator<Item = u8>,
    {
        let mut vec = iter.collect::<Vec<_>>();
        let mut bytes = vec![];

        let size = vec.len();

        for _ in 0..(vec.len() / 8) + 1 {
            bytes.push(
                join_bits((0..8).map(|_| if vec.is_empty() { 0 } else { vec.remove(0) })) as u8,
            );
        }

        Biterator {
            bytes,
            size,
            bit_index: 0,
        }
    }

    pub fn done(&self) -> bool {
        self.size <= 0
    }
}

impl Iterator for Biterator {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.size <= 0 {
            return None;
        }

        let result = (self.bytes[0] >> (7 - self.bit_index)) & 1;

        self.size -= 1;
        self.bit_index += 1;
        if self.bit_index >= 8 {
            self.bit_index = 0;
            self.bytes.remove(0);
        }

        Some(result)
    }
}

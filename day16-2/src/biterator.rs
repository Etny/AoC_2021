pub struct Biterator {
    bytes: Vec<u8>,
    bit_index: u8,
}

impl Biterator {
    pub fn new(hex: &str) -> Self {
        let mut bytes = vec![];

        for byte in hex.chars().collect::<Vec<_>>().chunks(2) {
            bytes.push(u8::from_str_radix(&byte.iter().collect::<String>(), 16).unwrap());
        }

        Biterator {
            bytes,
            bit_index: 0,
        }
    }
}

impl Iterator for Biterator {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.bytes.is_empty() {
            return None;
        }

        let result = (self.bytes[0] >> (7 - self.bit_index)) & 1;

        self.bit_index += 1;
        if self.bit_index >= 8 {
            self.bit_index = 0;
            self.bytes.remove(0);
        }

        Some(result)
    }
}

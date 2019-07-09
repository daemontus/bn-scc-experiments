

/// Simple bit set.
pub struct BitSet {
    values: Vec<u32>
}

impl BitSet {

    pub fn new_full(capacity: usize) -> BitSet {
        let size = (capacity / 32) + if capacity % 32 == 0 { 0 } else { 1 };
        return BitSet { values: vec![std::u32::MAX; size] }
    }

    pub fn new_empty(capacity: usize) -> BitSet {
        let size = (capacity / 32) + if capacity % 32 == 0 { 0 } else { 1 };
        return BitSet { values: vec![0; size] }
    }

    pub fn erase(&mut self, value: bool) {
        let erase_to = if value { 1 } else { 0 };
        for v in self.values.iter_mut() {
            *v = erase_to
        }
    }

    pub fn is_set(&self, index: usize) -> bool {
        let value_index = index / 32;
        let bit_index = (index % 32) as u32;
        return (self.values[value_index] >> bit_index) & 1 == 1;
    }

    pub fn flip(&mut self, index: usize) {
        let value_index = index / 32;
        let bit_index = (index % 32) as u32;
        self.values[value_index] ^= 1 << bit_index;
    }

}
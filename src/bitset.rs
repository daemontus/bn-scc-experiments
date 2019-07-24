use std::sync::atomic::{AtomicU32, Ordering};

/// Simple bit set.
pub struct BitSet {
    values: Vec<u32>
}

pub struct AtomicBitSet {
    values: Vec<AtomicU32>
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

impl AtomicBitSet {

    pub fn new_full(capacity: usize) -> AtomicBitSet {
        let size = (capacity / 32) + if capacity % 32 == 0 { 0 } else { 1 };
        return AtomicBitSet { values: (0..size).map(|_| AtomicU32::new(std::u32::MAX)).collect() }
    }

    pub fn new_empty(capacity: usize) -> AtomicBitSet {
        let size = (capacity / 32) + if capacity % 32 == 0 { 0 } else { 1 };
        return AtomicBitSet { values: (0..size).map(|_| AtomicU32::new(0)).collect() }
    }

    pub fn is_set(&self, index: usize) -> bool {
        let value_index = index / 32;
        let bit_index = (index % 32) as u32;
        return (self.values[value_index].load(Ordering::SeqCst) >> bit_index) & 1 == 1;
    }

    pub fn set(&self, index: usize) {
        loop {
            let value_index = index / 32;
            let bit_index = (index % 32) as u32;
            let old_value = self.values[value_index].load(Ordering::SeqCst);
            let new_value = old_value | (1 << bit_index);
            if self.values[value_index].compare_and_swap(old_value, new_value, Ordering::SeqCst) == old_value {
                return;
            }
        }
    }

    /*pub fn flip(&mut self, index: usize) {
        loop {
            let value_index = index / 32;
            let bit_index = (index % 32) as u32;
            let old_value = self.values[value_index].load(Ordering::SeqCst);
            let new_value = old_value ^ (1 << bit_index);
            if self.values[value_index].compare_and_set(old_value, new_value, Ordering::SeqCst) == old_value {
                return;
            }
        }
    }*/

}
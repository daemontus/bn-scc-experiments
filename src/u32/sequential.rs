use crate::bitset::BitSet;
use rand::prelude::StdRng;
use rand::{RngCore, SeedableRng};
use crate::u32::bn::{StateId, BooleanNetwork, VariableIterator};
use std::cmp::min;
use std::collections::HashMap;

/// Categorises states of the boolean network into disjoint sets of elements using the standard
/// union-find structure. Additionally, for every set, we can remember one extra u32 value.
/// Upon union, a minimum is computed from these two values.
struct DisjointSets {
    hash_mask: usize,
    is_root: BitSet,
    parent_pointer: Vec<u32>,
}

const FRESH: u32 = std::u32::MAX;
const DEAD: u32 = std::u32::MAX - 1;

pub fn scc(network: &BooleanNetwork) {
    let mut sets = DisjointSets::new(network.state_count() as usize, 1234567890);
    let mut dead = BitSet::new_empty(network.state_count() as usize);
    let mut stack: Vec<(StateId, VariableIterator)> = Vec::new();

    for root in network.states() {
        if dead.is_set(root.value as usize) { continue }

        print!("\rRemaining {}                             ", network.state_count() - root.value as u64);

        sets.set_payload(&root, 0);
        stack.push((root, network.variables()));

        while let Some((s, it)) = stack.last_mut() {
            if let Some(var) = it.next() {
                // if this variable has no successor or the successor SCC is already dead, do nothing
                if let Some(t) = network.successor(&s, &var) {
                    // Note that we can't test if t is dead (it can be a dead part of otherwise
                    // unfinished component), it root(t) is dead (the same) and if we didn't have
                    // special value for DEAD payload, we wouldn't know if the returned stack root
                    // index is valid because it can popped (and invalid) or overwritten by
                    // something else.
                    let payload = sets.get_payload(&t);
                    if payload == FRESH {
                        // t is newly discovered - add it to the stack!
                        sets.set_payload(&t, stack.len() as u32);
                        stack.push((t, network.variables()));
                    } else if payload != DEAD {
                        // t is already visited, but not dead, meaning we found a cycle.
                        // Now we have to merge everything on the stack with t, but skip
                        // the already merged parts of the graph using the stack_bottom
                        // pointers.
                        let mut to_merge_index = stack.len() - 1;
                        while sets.find_root(&stack[to_merge_index].0) != sets.find_root(&t) {
                            // skip all items already in the same set
                            to_merge_index = sets.get_payload(&stack[to_merge_index].0) as usize;
                            // union them with t
                            sets.union(stack[to_merge_index].0, t);
                            // and then move one item lower
                            to_merge_index -= 1;    // "virtual" pop
                        }
                    }
                }
            } else {
                // State is fully explored and can be removed from the stack
                let (s, _) = stack.pop().unwrap();                     // pop first to acquire ownership
                if sets.get_payload(&s) as usize == stack.len() {     // + 1 for the already popped element
                    // found component!
                    sets.set_payload(&s, DEAD)
                }
                dead.flip(s.value as usize);
            }

        }
        // reset stacks for next iteration
        stack.clear();
    }
    print!("\r");

    // count non-trivial components:
    let mut component_size: HashMap<usize, u32> = HashMap::new();
    for s in network.states() {
        if !sets.is_root(&s) {
            let root = sets.find_root(&s);
            let v = component_size.entry(root).or_insert(1);
            *v += 1;
        }
        //println!("Root of {} is {}", s, sets.find_root(&s));
    }
    println!("Non-trivial components: {}", component_size.len());
}

impl DisjointSets {

    /// Create a new disjoint sets structure using the given [capacity] (number of elements)
    /// and a [seed] for state key generator.
    fn new(capacity: usize, seed: u64) -> DisjointSets {
        let mut rnd = StdRng::seed_from_u64(seed);
        return DisjointSets {
            // hash mask is used for hashing state ids in order to implement Tarjan merge condition
            hash_mask: rnd.next_u64() as usize,
            // initially, every element is in a separate set, hence it is a root
            is_root: BitSet::new_full(capacity),
            // since initially everything is root, parent pointers store the extra u32 value initialized to 0
            parent_pointer: vec![FRESH; capacity]
        }
    }

    fn is_root(&self, key: &StateId) -> bool {
        return self.is_root.is_set(key.value as usize)
    }

    /// Compute the representing index for the set given by [key]. During search,
    /// every non-trivial path is contracted by path halving.
    fn find_root(&mut self, key: &StateId) -> usize {
        let mut item = key.value as usize;
        while !self.is_root.is_set(item) {
            let parent = self.parent_pointer[item] as usize;
            if self.is_root.is_set(parent) {
                return parent;
            } else {
                let parents_parent = self.parent_pointer[parent] as usize;
                self.parent_pointer[item] = parents_parent as u32;
                item = parents_parent;
            }
        }
        return item
    }

    /// Get the u32 payload of the given set.
    fn get_payload(&mut self, key: &StateId) -> u32 {
        let root = self.find_root(key);
        return self.parent_pointer[root];
    }

    /// Set the u32 payload for the given set.
    fn set_payload(&mut self, key: &StateId, payload: u32) {
        let root = self.find_root(key);
        self.parent_pointer[root] = payload;
    }

    /// Union two sets.
    fn union(&mut self, left: StateId, right: StateId) {
        let root_left = self.find_root(&left);
        let root_right = self.find_root(&right);
        if root_left != root_right {
            let new_payload = min(self.parent_pointer[root_left], self.parent_pointer[root_right]);
            if (root_left ^ self.hash_mask) > (root_right ^ self.hash_mask) {
                // attach right under left because left is "bigger"
                self.is_root.flip(root_right);
                self.parent_pointer[root_right] = root_left as u32;
                self.parent_pointer[root_left] = new_payload;
            } else {
                // attach left under right because right is "bigger"
                self.is_root.flip(root_left);
                self.parent_pointer[root_left] = root_right as u32;
                self.parent_pointer[root_right] = new_payload;
            }
        }
    }

}
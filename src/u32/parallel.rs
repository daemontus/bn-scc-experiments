use std::sync::atomic::{AtomicU32, Ordering};
use rand::prelude::StdRng;
use rand::{RngCore, SeedableRng};
use crate::u32::bn::{StateId, BooleanNetwork, VariableIterator};
use crate::u32::sequential::{DisjointSets, DEAD, FRESH};
use crossbeam::thread;
use crate::bitset::AtomicBitSet;


pub fn parallel_scc(network: &BooleanNetwork, parallelism: u32) {

    //let global_network = Arc::new(network);
    let global_sets = AtomicDisjointSets::new(network.state_count() as usize, 1234567890);
    let global_dead = AtomicBitSet::new_empty(network.state_count() as usize);

    println!("State count {}", network.state_count());

    let thread_id = AtomicU32::new(0);
    thread::scope(|thread_scope| {
        for _ in 0..parallelism {
            thread_scope.spawn(|_| {

                let mut sets = DisjointSets::new(network.state_count() as usize, 1234567890);
                let mut stack: Vec<(StateId, VariableIterator)> = Vec::new();

                let thread_id: u64 = thread_id.fetch_add(1, Ordering::SeqCst) as u64;
                let key: u64 = thread_id * (network.state_count() / (parallelism as u64));

                let mut explored: usize = 0;
                let mut iter: usize = 0;
                let mut max_stack_size: usize = 0;

                for root_seq in network.states() {
                    let root = StateId { value: ((root_seq.value as u64 + key) % network.state_count()) as u32 };
                    if sets.get_payload(&root) == DEAD { continue }
                    let set_of_root = global_sets.find_root(&root);
                    if global_dead.is_set(set_of_root) { continue }

                    explored += 1;
                    //if thread_id == 0 {
                    //    print!("\rRemaining {}                             ", network.state_count() - root.value as u64);
                    //}

                    sets.set_payload(&root, 0);
                    stack.push((root, network.variables()));

                    while let Some((s, it)) = stack.last_mut() {
                        iter += 1;
                        let set_of_s = global_sets.find_root(&s);
                        if global_dead.is_set(set_of_s) {
                            stack.pop();
                        } else {
                            if let Some(var) = it.next() {
                                // if this variable has no successor or the successor SCC is already dead, do nothing
                                if let Some(t) = network.successor(&s, &var) {
                                    // Note that we can't test if t is dead (it can be a dead part of otherwise
                                    // unfinished component), it root(t) is dead (the same) and if we didn't have
                                    // special value for DEAD payload, we wouldn't know if the returned stack root
                                    // index is valid because it can popped (and invalid) or overwritten by
                                    // something else.
                                    let set_of_t = global_sets.find_root(&t);
                                    let payload = sets.get_payload(&t);
                                    if payload == FRESH && !global_dead.is_set(set_of_t) {
                                        explored += 1;
                                        // t is newly discovered - add it to the stack!
                                        sets.set_payload(&t, stack.len() as u32);
                                        stack.push((t, network.variables()));
                                        if stack.len() > max_stack_size {
                                            max_stack_size = stack.len()
                                        }
                                        // this has no performance impact since the branch is easy to predict...
                                        if stack.len() as u32 == DEAD { panic!("Stack overflow!") }
                                    } else if /*payload != DEAD &&*/ !global_dead.is_set(set_of_t) {
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
                                            global_sets.union(stack[to_merge_index].0, t);
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
                                    sets.set_payload(&s, DEAD);
                                    let set_of_s = sets.find_root(&s);
                                    global_dead.set(set_of_s);  // set globally dead
                                }
                            }

                        }
                    }
                    // reset stacks for next iteration
                    stack.clear();
                }

                //if thread_id == 0 {
                //    print!("\r");
                //}

                println!("Processed {} states in {} iterations and max stack {}", explored, iter, max_stack_size)
            });
        }
    }).unwrap();

    /*// count non-trivial components:
    let mut component_size: HashMap<usize, u32> = HashMap::new();
    for s in network.states() {
        if !global_sets.is_root(&s) {
            let root = global_sets.find_root(&s);
            let v = component_size.entry(root).or_insert(1);
            *v += 1;
        }
        //println!("Root of {} is {}", s, sets.find_root(&s));
    }
    println!("Non-trivial components: {}", component_size.len());*/
}


/// This Disjoint sets structure does not store any payload or anything similar, so it can
/// be fully implemented using atomics. Since we don't have to store a payload, we can identify
/// roots as having itself as a parent.
///
struct AtomicDisjointSets {
    hash_mask: usize,
    parent_pointer: Vec<AtomicU32>
}

impl AtomicDisjointSets {

    fn new(capacity: usize, seed: u64) -> AtomicDisjointSets {
        let mut rnd = StdRng::seed_from_u64(seed);
        return AtomicDisjointSets {
            hash_mask: rnd.next_u64() as usize,
            parent_pointer: (0..capacity).map(|s| AtomicU32::new(s as u32)).collect()
        }
    }

    /*fn is_root(&self, key: &StateId) -> bool {
        return self.parent_pointer[key.value as usize].load(Ordering::SeqCst) == key.value
    }*/

    fn find_root(&self, key: &StateId) -> usize {
        return self.find_root_by_index(key.value as usize)
    }

    fn find_root_by_index(&self, key: usize) -> usize {
        let mut item = key;
        let mut parent = self.parent_pointer[item].load(Ordering::SeqCst) as usize;
        while parent != item {
            // Note: Once parent != item, it can never equal again, hence we don't have to
            // re-check this condition even though the parent can change.
            let parents_parent = self.parent_pointer[parent].load(Ordering::SeqCst) as usize;
            if parents_parent == parent {
                return parent
            } else {
                // Path halving - only update if someone else hasn't already done some changes.
                // If changes were done, we don't do anything, just advance to next item...
                self.parent_pointer[item].compare_and_swap(parent as u32, parents_parent as u32, Ordering::SeqCst);
                item = parents_parent;
                parent = self.parent_pointer[parents_parent].load(Ordering::SeqCst) as usize;
            }
        }
        return item
    }

    fn union(&self, left: StateId, right: StateId) {
        let mut l = left.value as usize;
        let mut r = right.value as usize;
        loop {
            l = self.find_root_by_index(l);
            r = self.find_root_by_index(r);
            if r == l { return } else {
                if (l ^ self.hash_mask) > (r ^ self.hash_mask) {
                    // attach right under left because left is "bigger"
                    if self.parent_pointer[r].compare_and_swap(r as u32, l as u32, Ordering::SeqCst) == r as u32 { return }
                } else {
                    if self.parent_pointer[l].compare_and_swap(l as u32, r as u32, Ordering::SeqCst) == l as u32 { return }
                }
            }
        }
    }

}
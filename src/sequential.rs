use std::cmp::min;
use crate::bn::{BooleanNetwork, State, BNVariableIterator};
use std::collections::HashSet;

pub fn dfs(network: &BooleanNetwork) {
    let mut explore: Vec<(State, BNVariableIterator)> = Vec::new();
    let mut explored: HashSet<State> = HashSet::new();

    let initial = network.states().next().expect("Empty network");
    explore.push((initial, network.variables()));

    for state in network.states() {
        for variable in network.variables() {
            if let Some(next_state) = network.successor(&state, &variable) {
                println!("Edge from {} to {}", state, next_state);
            }
        }
    }

    while let Some((state, iterator)) = explore.last_mut() {
        explored.insert(state.clone());
        if let Some(var) = iterator.next() {
            println!("Explore {} in {:?}", state, var);
            if let Some(next_state) = network.successor(&state, &var) {
                if !explored.contains(&next_state) {
                    println!("Push {}", next_state);
                    explore.push((next_state, network.variables()));
                }
            }
        } else {
            println!("Pop {}", state);
            explore.pop();
        }
    }

}

pub fn scc(network: &BooleanNetwork) {
    let mut sets = DisjointSetsWithStackBottoms::new(network.state_count());
    let mut not_dead: HashSet<State> = network.states().collect();
    //let mut dead: HashSet<State> = HashSet::new();

    while !not_dead.is_empty() {
        let mut stack: Vec<(State, BNVariableIterator)> = Vec::new();
        let initial = not_dead.iter().next().expect("Empty network").clone();
        stack.push((initial, network.variables()));
        sets.set_stack_bottom(initial.index, stack.len());
        //println!("Stack bottom of {} is {}", initial, stack.len());

        while let Some((s, it)) = stack.last_mut() {
            if let Some(v) = it.next() {
                // if this variable has no successor or the successor is already dead, do nothing
                if let Some(t) = network.successor(&s, &v) {
                    if not_dead.contains(&State { index: sets.find(t.index) }) {
                        if !sets.has_stack_bottom(t.index) {
                            // t is newly discovered - add it to the stack!
                            //println!("Discovered {} from {}", t, s);
                            stack.push((t, network.variables()));
                            sets.set_stack_bottom(t.index, stack.len());
                        } else {
                            // t is already visited, but not dead, meaning we found a cycle.
                            // Now we have to merge everything on the stack with t, but skip
                            // the already merged parts of the graph using the stack_bottom
                            // pointers.
                            //println!("Cycle found in {}", t);
                            let mut to_merge_index = stack.len() - 1;
                            while sets.find(stack[to_merge_index].0.index) != sets.find(t.index) {
                                // skip all items already in the same set
                                to_merge_index = sets.stack_bottom(stack[to_merge_index].0.index) - 1;
                                // union them with t
                                //println!("Merge {} with {}", stack[to_merge_index].0, t);
                                sets.union(stack[to_merge_index].0.index, t.index);
                                // and then move one item lower
                                to_merge_index -= 1;    // "virtual" pop
                            }
                        }
                    }
                }
            } else {
                // State is fully explored and can be removed from the stack
                let (s, _) = stack.pop().unwrap();                     // pop first to acquire ownership
                if sets.stack_bottom(s.index) == stack.len() + 1 {      // + 1 for already popped element
                    let size = sets.set_size(s.index);
                    if size > 1 {
                        //println!("Found SCC in {} of size {}", s, sets.set_size(s.index))
                    }
                }
                //println!("Mark dead: {} with bottom {}", s, sets.stack_bottom(s.index));
                not_dead.remove(&s);
            }
        }

        print!("\rRemaining {}                             ", not_dead.len());
    }

    println!();

    let mut non_trivial = 0;
    for state in network.states() {
        if sets.find(state.index) == state.index && sets.set_size(state.index) > 1 {
            non_trivial += 1;
        }
    }
    println!("Non-trivial components (old): {}", non_trivial);
}

/// Implements disjoint sets where each set also remembers the index+1 of its first
/// element occurring on the stack (stack_bottom). We use index+1 because it means initially
/// all elements can start with a stack_bottom = 0 and we won't have to cast it later.
pub struct DisjointSetsWithStackBottoms {
    parent_pointer: Vec<usize>,
    set_size: Vec<usize>,
    stack_bottom: Vec<usize>
}

impl DisjointSetsWithStackBottoms {

    pub fn new(capacity: usize) -> DisjointSetsWithStackBottoms {
        return DisjointSetsWithStackBottoms {
            // Initially, each parent pointer loops to the same element (all elements are disjoint)
            parent_pointer: (0..capacity).collect(),
            set_size: vec![1; capacity],
            stack_bottom: vec![0; capacity]
        }
    }

    /// Find the first stack index of given set.
    pub fn stack_bottom(&mut self, item: usize) -> usize {
        let root = self.find(item);
        return self.stack_bottom[root];
    }

    /// Check if the given element has already some assigned stack bottom (i.e. it is visited)
    pub fn has_stack_bottom(&self, item: usize) -> bool {
        return self.stack_bottom[item] > 0;
    }

    /// Set the stack bottom of a newly found element. Note that bottom cannot be zero!
    pub fn set_stack_bottom(&mut self, item: usize, bottom: usize) {
        self.stack_bottom[item] = bottom;
    }

    /// Find returns the representant of the set which item belongs to.
    ///
    /// It implements path halving strategy for path reduction, that is, every time
    /// we search for an item, the parent of each item is shifted one level higher.
    ///
    /// This does not require a large stack to store the whole path, but the path reduction
    /// is slower. In the future, we can try to replace this with a pre-allocated stack...
    pub fn find(&mut self, mut item: usize) -> usize {
        while self.parent_pointer[item] != item {
            let parent = self.parent_pointer[item];
            let parents_parent = self.parent_pointer[parent];
            self.parent_pointer[item] = parents_parent;
            item = parents_parent;
        }
        return item
    }

    /// Union two sets given by left and right, updating the corresponding set size and
    /// stack pointers.
    pub fn union(&mut self, left: usize, right: usize) {
        let root_left = self.find(left);
        let root_right = self.find(right);
        if root_left != root_right {
            let new_stack_bottom = min(self.stack_bottom[root_left], self.stack_bottom[root_right]);
            if self.set_size[root_left] > self.set_size[root_right] {
                // attach right under left because its smaller
                self.parent_pointer[root_right] = root_left;
                self.set_size[root_left] += self.set_size[root_right];
                self.stack_bottom[root_left] = new_stack_bottom;
            } else {
                // attach left under right because its smaller
                self.parent_pointer[root_left] = root_right;
                self.set_size[root_right] += self.set_size[root_left];
                self.stack_bottom[root_right] = new_stack_bottom;
            }
        }
    }

    pub fn set_size(&mut self, item: usize) -> usize {
        let root = self.find(item);
        return self.set_size[root];
    }


}
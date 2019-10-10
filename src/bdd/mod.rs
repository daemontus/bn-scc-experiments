use std::collections::HashMap;

mod dot_printer;

/// BDD Node represents one vertex of the BDD DAG. It specifies the variable upon which
/// we are conditioning and two pointers into the BDD itself. Hence every node can only
/// exist in a context of a specific BDD and we have to guarantee that the pointer invariants
/// are satisfied.
///
/// There are two special types of nodes, `one` and `zero` which are used to represent terminal
/// vertices. For these, the variable contains total number of variables which can appear in
/// the specific BDD (for consistency checks) and low/high pointers correspond to the value
/// of the node (one or zero). These two nodes are always placed at first two positions of
/// the BDD vector, making the pointers cyclic.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct BDDNode {
    var: u32,
    low: u32,
    high: u32
}

impl BDDNode {

    /// Make a new `one` node.
    fn mk_one(vars: u32) -> BDDNode {
        return BDDNode { var: vars, low: 1, high: 1 }
    }

    /// Make a new `zero` node.
    fn mk_zero(vars: u32) -> BDDNode {
        return BDDNode { var: vars, low: 0, high: 0 }
    }

    /// Check whether this node is effectively terminal.
    ///
    /// WARNING: This does not mean the node is necessarily terminal, it might also just
    /// point to a terminal node, effectively gaining its value. However, this should not
    /// happen in minimized BDDs.
    fn is_terminal(&self) -> bool {
        return self.low == self.high && (self.low == 1 || self.low == 0)
    }

    /// Check whether this node is effectively one.
    fn is_one(&self) -> bool {
        return self.is_terminal() && self.low == 1
    }

    /// Check whether this node is effectively zero.
    fn is_zero(&self) -> bool {
        return self.is_terminal() && self.low == 0
    }

}

/// BDD is represented as a vector of BDD Nodes which together form the DAG of the BDD.
/// The nodes are sorted in DFS post-order (also can be seen as topological order). That
/// means nodes "lower" in the graph are "lower" in the vector with two terminal nodes being
/// the first ones. Each BDD has at least one node, terminal node `zero`. Each non-empty BDD
/// has at least two nodes, the `zero` and `one` nodes (Even a `true` BDD has both of them).
///
/// The root of the DAG is considered to be the last element of the list. Due to the ordering
/// of nodes, one can interpret a right-slice of the vector as a BDD as well, since all "lower"
/// vertices must be stored "below" the last element of the slice.
///
/// We assume each BDD is ordered and canonical! The format should work correctly for non-canonical
/// BDDs as well, but some operations can work only for canonical BDDs - for example satisfiability
/// check.
///
/// We do not implement operations directly on BDDs, instead we use BDD workers to manipulate BDDs.
#[derive(Clone, Debug, PartialEq)]
pub struct BDD(Vec<BDDNode>);

impl BDD {

    /// Number of nodes in this BDD
    fn size(&self) -> usize {
        return self.0.len();
    }

    /// Index of last node in the BDD (root of the graph)
    fn last_index(&self) -> usize {
        return self.0.len() - 1;
    }

    /// VarID of the given node.
    fn var(&self, node_index: usize) -> usize {
        return self.0[node_index].var as usize;
    }

    /// High link of the given node.
    fn high_link(&self, node_index: usize) -> usize {
        return self.0[node_index].high as usize;
    }

    /// Low link of the given node.
    fn low_link(&self, node_index: usize) -> usize {
        return self.0[node_index].low as usize;
    }

    /// Number of variables in the BDD (used for consistency checks)
    fn num_vars(&self) -> u32 {
        return self.0[0].var;
    }

}

pub struct BDDWorker {
    num_vars: u32,
    var_names: Vec<String>,
    var_index_mapping: HashMap<String, u32>
}

/// BDD worker implements all necessary operations on BDDs. The operations are described
/// in terms of logic (and, or, ...) instead of sets (intersect, union, ...) because not
/// all BDDs represent sets in a strict sense. If you want a more idiomatic set operations,
/// you can implement a wrapper around BDDWorker (we might provide something like that
/// later as well).
impl BDDWorker {

    /// Create a new BDD worker initialized with given set of variables.
    pub fn new(variables: Vec<String>) -> BDDWorker {
        let mut var_index_mapping: HashMap<String, u32> = HashMap::new();
        for var_index in 0..variables.len() {
            var_index_mapping.insert(variables[var_index].clone(), var_index as u32);
        }
        return BDDWorker {
            num_vars: variables.len() as u32,
            var_names: variables,
            var_index_mapping
        }
    }

    /// Create a new BDD worker initialized with a set of anonymous variables (only identified by index).
    pub fn new_anonymous(num_vars: u32) -> BDDWorker {
        return BDDWorker::new((0..num_vars).map(|x| x.to_string()).collect())
    }

    fn mk_zero_node(&self) -> BDDNode {
        return BDDNode::mk_zero(self.num_vars)
    }

    fn mk_one_node(&self) -> BDDNode {
        return BDDNode::mk_one(self.num_vars)
    }

    /// Create a BDD corresponding to the `false` formula.
    pub fn mk_false(&self) -> BDD {
        return BDD(vec![self.mk_zero_node()])
    }

    /// Create a BDD corresponding to the `true` formula.
    pub fn mk_true(&self) -> BDD {
        return BDD(vec![self.mk_zero_node(), self.mk_one_node()])
    }

    fn var_index_out_of_bounds(&self, var_index: u32) -> ! {
        panic!("Cannot create BDD with variable ID {}, there are only {} variables.", var_index, self.num_vars)
    }

    fn var_name_out_of_bounds(&self, var_name: &String) -> ! {
        panic!("Cannot create BDD with variable {}; Available variables: {:?}.", var_name, self.var_names)
    }

    /// Create a BDD corresponding to the `x` formula where `x` is the variable of
    /// the given index.
    pub fn mk_var(&self, var_index: u32) -> BDD {
        return if var_index >= self.num_vars {
            self.var_index_out_of_bounds(var_index)
        } else {
            BDD(vec![self.mk_zero_node(), self.mk_one_node(), BDDNode {
                var: var_index,
                low: 0, high: 1
            }])
        }
    }

    /// Create a BDD corresponding to the `!x` formula where `x` is the variable of
    /// the given index.
    pub fn mk_not_var(&self, var_index: u32) -> BDD {
        return if var_index >= self.num_vars {
            self.var_index_out_of_bounds(var_index)
        } else {
            BDD(vec![self.mk_zero_node(), self.mk_one_node(), BDDNode {
                var: var_index,
                low: 0, high: 1
            }])
        }
    }

    /// Create a BDD corresponding to the `x` formula where `x` is the variable of
    /// the given name.
    pub fn mk_named_var(&self, var_name: &String) -> BDD {
        return match self.var_index_mapping.get(var_name) {
            None => self.var_name_out_of_bounds(var_name),
            Some(index) => self.mk_var(*index),
        }
    }

    /// Create a BDD corresponding to the `!x` formula where `x` is the variable
    /// of the given name.
    pub fn mk_not_named_var(&self, var_name: &String) -> BDD {
        return match self.var_index_mapping.get(var_name) {
            None => self.var_name_out_of_bounds(var_name),
            Some(index) => self.mk_not_var(*index),
        }
    }

    /// Create a BDD corresponding to `!phi` formula where `phi` is another
    /// formula given as a BDD.
    pub fn mk_not(&self, bdd: &BDD) -> BDD {
        return if self.is_false(bdd) {
            self.mk_true()
        } else if self.is_true(bdd) {
            self.mk_false()
        } else {
            let mut negation = bdd.0.clone();
            // In each node, we swap links to `zero` and `one` (but nothing else).
            // Note that this does not break the ordering of nodes because terminals have
            // special position in the vector. (Shape of the graph is the same except for
            // links to terminals which are ordered explicitly)
            for i in 2..negation.len() {    // don't flip terminals
                let node = negation.get_mut(i).unwrap();
                // if link is 0/1, flip the bit using xor
                if node.low <= 1 { node.low = node.low ^ 1; }
                if node.high <= 1 { node.high = node.high ^ 1; }
            }
            BDD(negation)
        }
    }

    pub fn mk_and(&self, left: &BDD, right: &BDD) -> BDD {
        return self.apply(left, right, |l, r| -> Option<bool> {
            if l.is_zero() || r.is_zero() { Some(false) }
            else if l.is_one() && r.is_one() { Some(true) }
            else { None }
        });
    }

    /// Universal function to implement standard logical operators. The `terminal_lookup` function
    /// takes two BDDNodes that we are currently considering and returns a fixed boolean value
    /// if these two nodes can be evaluated by the function being implemented. For example,
    /// if one of the nodes is `zero` and we are implementing `and`, we can immediately
    /// evaluate to `false`.
    fn apply<T>(&self, left: &BDD, right: &BDD, terminal_lookup: T) -> BDD
        where T: Fn(BDDNode, BDDNode) -> Option<bool>
    {
        // Result holds the new BDD we are computing. Initially, we assume both `zero` and `one`
        // nodes are present. In case the resulting BDD is empty, we have to remove the `one`
        // node. We use a special variable to keep track of whether we have already seen
        // `one` being used. This seems easier than adding `one` explicitly (less branches/code).
        let mut result: Vec<BDDNode> = vec![self.mk_zero_node(), self.mk_one_node()];
        let mut is_not_empty = false;

        // Stack is used to explore the two BDDs "side by side" in DFS-like manner.
        // The two values are always indices into the corresponding left/right BDDs that
        // still need to be processed.
        let mut stack: Vec<(usize, usize)> = Vec::new();
        stack.push((left.last_index(), right.last_index())); // Initially, BDD roots are unprocessed.

        // Finished holds indices of BDD node pairs that have been successfully computed.
        // (That is, pairs that have been removed from stack at some point) It is used to avoid
        // computing rediscovered pairs.
        let mut finished: HashMap<(usize, usize), usize> = HashMap::new();

        // Created is used to avoid duplicates. As soon as a node is inserted into result, its
        // index is added to created. Hence we avoid creation of duplicate nodes.
        let mut created: HashMap<BDDNode, usize> = HashMap::new();
        created.insert(self.mk_zero_node(), 0);
        created.insert(self.mk_one_node(), 1);

        while let Some((l,r)) = stack.last() {
            if finished.contains_key(&(*l, *r)) {
                // (l,r) was requested for resolution, but is already resolved - we can just skip
                stack.pop();
            } else {
                let (var_left, var_right) = (left.var(*l), right.var(*r));
                // Based on variables and ordering, we are either going to advance in both BDDs,
                // or just one. We advance BDD with smaller variable - smaller variables are
                // "higher" in the graph (terminal nodes are "largest", root is smallest). Hence we
                // advance the BDD where we are closer to the top.
                let left_low; let left_high; let right_low; let right_high;
                if var_left == var_right {
                    // Nodes have the same variable, advance both BDDs
                    left_low = left.low_link(*l); left_high = left.high_link(*l);
                    right_low = right.low_link(*r); right_high = right.high_link(*r);
                } else if var_left < var_right {
                    left_low = left.low_link(*l); left_high = left.high_link(*l);
                    right_low = *r; right_high = *r;
                } else {
                    left_low = *l; left_high = *l;
                    right_low = right.low_link(*r); right_high = right.high_link(*r);
                }
                let decision_var = std::cmp::min(left.var(*l), right.var(*r));

                let new_low: Option<usize> = if let Some(leaf) = terminal_lookup(left.0[left_low], right.0[right_low]) {
                    // No need to explore further, the answer is determined!
                    if !leaf { Some(0) } else { is_not_empty = true; Some(1) }  // return explicit index of a terminal
                } else {
                    // Try to get the value from cache
                    finished.get(&(left_low, right_low)).map(|x| *x)
                };

                let new_high: Option<usize> = if let Some(leaf) = terminal_lookup(left.0[left_high], right.0[right_high]) {
                    if !leaf { Some(0) } else { is_not_empty = true; Some(1) }
                } else {
                    finished.get(&(left_high, right_high)).map(|x| *x)
                };

                match (new_low, new_high) {
                    (Some(new_low), Some(new_high)) => {
                        // Both values are already computed, hence we can also complete this one
                        if new_low == new_high {
                            // There is no decision, just skip this node and point to either child
                            finished.insert((*l, *r), new_low);
                        } else {
                            // There is a decision here.
                            let node = BDDNode {
                                var: decision_var as u32,
                                low: new_low as u32,
                                high: new_high as u32
                            };
                            if let Some(index) = created.get(&node) {
                                // Node already exists, no need to add a new one, just make it
                                // a result of this operation.
                                finished.insert((*l, *r), *index);
                            } else {
                                // Node does not exist, it needs to be pushed to result.
                                // Index of the node is result len before insertion.
                                finished.insert((*l, *r), result.len());
                                created.insert(node.clone(), result.len());
                                result.push(node.clone());
                            }
                        }
                        stack.pop();    // remove (l,r) from work stack
                    }
                    // We are missing the high link, we keep (l,r) on the stack and add new work
                    (None, Some(_)) => stack.push((left_low, right_low)),
                    (Some(_), None) => stack.push((left_high, right_high)),
                    (None, None) => {
                        stack.push((left_low, right_low));
                        stack.push((left_high, right_high));
                    }
                }
            }
        }

        return if is_not_empty { BDD(result) } else { self.mk_false() }
    }

    /// Return true if the BDD represents the `false` formula.
    pub fn is_false(&self, bdd: &BDD) -> bool {
        return bdd.0.len() == 1
    }

    /// Return true if the BDD represents the `true` formula.
    pub fn is_true(&self, bdd: &BDD) -> bool {
        return bdd.0.len() == 2
    }

    /// Convert the given BDD to a .dot file string. Using zero_pruned argument,
    /// you can control whether the zero node is printed as well.
    pub fn as_dot_string(&self, bdd: &BDD, zero_pruned: bool) -> String {
        let mut buffer: Vec<u8> = Vec::new();
        dot_printer::print_bdd_as_dot(&mut buffer, bdd, &self.var_names, zero_pruned)
            .expect("Cannot write BDD to .dot string.");
        return String::from_utf8(buffer)
            .expect("Invalid UTF formatting in .dot string.");
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    // A small BDD over variables x0..x4 corresponding to formula (x4 & !x3)
    fn mk_small_test_bdd() -> BDD {
        return BDD(vec![
            BDDNode::mk_zero(5), BDDNode::mk_one(5),
            BDDNode { var: 3, low: 1, high: 0 },
            BDDNode { var: 4, low: 0, high: 2 }
        ]);
    }

    fn load_expected_results(test_name: &str) -> String {
        return std::fs::read_to_string(format!("test_results/bdd/{}", test_name)).expect("Cannot open result file.")
    }

    #[test]
    fn bdd_node_one() {
        let one = BDDNode::mk_one(2);
        assert!(one.is_terminal());
        assert!(one.is_one());
        assert!(!one.is_zero());
        assert_eq!(2, one.var);
    }

    #[test]
    fn bdd_node_zero() {
        let zero = BDDNode::mk_zero(2);
        assert!(zero.is_terminal());
        assert!(zero.is_zero());
        assert!(!zero.is_one());
        assert_eq!(2, zero.var);
    }

    #[test]
    fn bdd_impl() {
        let bdd = mk_small_test_bdd();

        assert_eq!(4, bdd.size());
        assert_eq!(3, bdd.last_index());
        assert_eq!(1, bdd.low_link(2));
        assert_eq!(0, bdd.high_link(2));
        assert_eq!(0, bdd.low_link(3));
        assert_eq!(2, bdd.high_link(3));
    }

    #[test]
    fn bdd_to_dot() {
        let bdd = mk_small_test_bdd();
        let worker = BDDWorker::new_anonymous(bdd.num_vars());
        let dot = worker.as_dot_string(&bdd, false);
        assert_eq!(load_expected_results("bdd_to_dot.dot"), dot);
    }

    #[test]
    fn bdd_to_dot_with_names() {
        let bdd = mk_small_test_bdd();
        let worker = BDDWorker::new(vec![
            "a".to_string(), "b".to_string(), "c".to_string(), "d".to_string(), "e".to_string()
        ]);
        let dot = worker.as_dot_string(&bdd, false);
        assert_eq!(load_expected_results("bdd_to_dot_with_names.dot"), dot);
    }

    #[test]
    fn bdd_to_dot_pruned() {
        let bdd = mk_small_test_bdd();
        let worker = BDDWorker::new_anonymous(bdd.num_vars());
        let dot = worker.as_dot_string(&bdd, true);
        assert_eq!(load_expected_results("bdd_to_dot_pruned.dot"), dot);
    }

    #[test]
    #[should_panic]
    fn bdd_mk_var_unknown_index() {
        let worker = BDDWorker::new_anonymous(2);
        worker.mk_var(2);
    }

    #[test]
    #[should_panic]
    fn bdd_mk_not_var_unknown_index() {
        let worker = BDDWorker::new_anonymous(2);
        worker.mk_not_var(2);
    }

    #[test]
    #[should_panic]
    fn bdd_mk_var_unknown_name() {
        let worker = BDDWorker::new(vec!["v1".to_string(), "v2".to_string()]);
        worker.mk_named_var(&"v3".to_string());
    }

    #[test]
    #[should_panic]
    fn bdd_mk_not_var_unknown_name() {
        let worker = BDDWorker::new(vec!["v1".to_string(), "v2".to_string()]);
        worker.mk_not_named_var(&"v3".to_string());
    }

    #[test]
    fn bdd_mk_not_constants() {
        let worker = BDDWorker::new_anonymous(1);
        let tt = worker.mk_true();
        let ff = worker.mk_false();

        assert_eq!(ff, worker.mk_not(&tt));
        assert_eq!(tt, worker.mk_not(&ff));
    }

    #[test]
    fn bdd_mk_not_basic() {
        let bdd = mk_small_test_bdd();
        let worker = BDDWorker::new_anonymous(bdd.num_vars());
        // !(x4 & !x3) = !x4 | x3
        let not_bdd = worker.mk_not(&bdd);
        let expected = BDD(vec![
            BDDNode::mk_zero(5), BDDNode::mk_one(5),
            BDDNode { var: 3, low: 0, high: 1 },
            BDDNode { var: 4, low: 1, high: 2 }
        ]);

        assert_eq!(expected, not_bdd);
    }

    #[test]
    fn bdd_mk_and() {
        let bdd = mk_small_test_bdd();
        let worker = BDDWorker::new_anonymous(bdd.num_vars());
        let not_bdd = worker.mk_not(&bdd);
        let and = worker.mk_and(&bdd, &not_bdd);

        println!("{:?}", and);
        assert!(worker.is_false(&and));
    }

}
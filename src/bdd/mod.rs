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

    fn load_expected_results(test_name: &str) -> String {
        return std::fs::read_to_string(format!("test_results/bdd/{}", test_name)).expect("Cannot open result file.")
    }

}
use std::collections::HashMap;
use super::{MAX_VARS, Variable, State, BooleanNetwork};

/// BNBuilder allows to create a new boolean network in a somewhat safer fashion.
/// Specifically, it checks that there are no duplicates and no update functions are
/// missing.
pub struct BNBuilder {
    variable_count: usize,
    variable_names: HashMap<usize, String>,
    update_functions: HashMap<usize, Box<dyn Fn(&State) -> bool>>
}

impl BNBuilder {

    /// Make a new empty boolean network builder.
    pub fn new() -> BNBuilder {
        return BNBuilder {
            variable_count: 0,
            variable_names: HashMap::new(),
            update_functions: HashMap::new()
        }
    }

    /// Create a new variable in this network.
    /// Panics if the variable already exists or the network is too large.
    pub fn make_variable(&mut self, name: &str) -> Variable {
        if self.variable_count >= MAX_VARS {
            panic!("Cannot create network with more than {} variables.", MAX_VARS);
        }
        let index = self.variable_count;
        self.variable_count += 1;
        for (_, existing) in &self.variable_names {
            if name == existing {
                panic!("Variable named {} already exists.", existing);
            }
        }
        self.variable_names.insert(index, String::from(name));
        return Variable { index }
    }

    /// Associate an update function with a variable.
    /// Panics if the variable does not exist or if it already has a function defined.
    pub fn update_function(&mut self, var: &Variable, fun: Box<dyn Fn(&State) -> bool>) {
        if !self.variable_names.contains_key(&var.index) {
            panic!("Variable #{} does not exist in this boolean network.", var.index);
        }
        if self.update_functions.contains_key(&var.index) {
            panic!("Cannot redefine update function for {}.", self.variable_names[&var.index])
        }
        self.update_functions.insert(var.index, fun);
    }

    /// Consume this builder into a full-fledged boolean network.
    pub fn build_network(mut self) -> BooleanNetwork {
        for v in 0..self.variable_count {
            if !self.update_functions.contains_key(&v) {
                panic!("Update function for {} not specified.", self.variable_names[&v])
            }

        }
        let mut functions: Vec<(usize, Box<dyn Fn(&State) -> bool>)> = self.update_functions.drain().collect();
        functions.sort_by_key(|&(k, _)| k);

        return BooleanNetwork {
            update_functions: functions.into_iter().map(|(_, f)| f).collect()
        }
    }

}

#[cfg(test)]
mod test {

    use std::collections::HashSet;
    use super::*;

    #[test]
    fn make_simple_bn_test() {
        let mut builder = BNBuilder::new();
        let a = builder.make_variable("a");
        let b = builder.make_variable("b");

        /* Note: since variable implements clone, it can be moved to the lambdas! */

        builder.update_function(&a, Box::new(move |s| {
            !s.get(&a) || s.get(&b)
        }));

        builder.update_function(&b, Box::new( move |s| {
            s.get(&b) || s.get(&a)
        }));

        let bn = builder.build_network();
        assert_eq!(4, bn.state_count());
        assert_eq!(2, bn.variable_count());

        let s00 = State::from_data(&[false, false]);
        let s01 = State::from_data(&[false, true]);
        let s10 = State::from_data(&[true, false]);
        let s11 = State::from_data(&[true, true]);

        let states: HashSet<State> = bn.states().collect();
        assert_eq!(4, states.len());
        assert!(states.contains(&s00));
        assert!(states.contains(&s01));
        assert!(states.contains(&s10));
        assert!(states.contains(&s11));

        let variables: HashSet<Variable> = bn.variables().collect();
        assert_eq!(2, variables.len());
        assert!(variables.contains(&a));
        assert!(variables.contains(&b));

        assert_eq!(Some(s10), bn.successor(&s00, &a));
        assert_eq!(None, bn.successor(&s00, &b));

        assert_eq!(Some(s11), bn.successor(&s01, &a));
        assert_eq!(None, bn.successor(&s01, &b));

        assert_eq!(Some(s00), bn.successor(&s10, &a));
        assert_eq!(Some(s11), bn.successor(&s10, &b));

        assert_eq!(None, bn.successor(&s11, &a));
        assert_eq!(None, bn.successor(&s11, &b));
    }

    #[test] #[should_panic]
    fn make_bn_too_big_test() {
        let mut builder = BNBuilder::new();
        for i in 0..(MAX_VARS+1) {
            builder.make_variable(&format!("{}", i));
        }
    }

    #[test] #[should_panic]
    fn make_bn_double_var_test() {
        let mut builder = BNBuilder::new();
        builder.make_variable("a");
        builder.make_variable("a");
    }

    #[test] #[should_panic]
    fn make_bn_function_for_unknown_var_test() {
        let mut builder = BNBuilder::new();
        builder.make_variable("a");
        let v = Variable { index: 2 };
        builder.update_function(&v, Box::new(|_| true))
    }

    #[test] #[should_panic]
    fn make_bn_function_redeclaration_test() {
        let mut builder = BNBuilder::new();
        let a = builder.make_variable("a");
        builder.update_function(&a, Box::new(|_| true));
        builder.update_function(&a, Box::new(|_| false));
    }

    #[test] #[should_panic]
    fn make_bn_function_missing_test() {
        let mut builder = BNBuilder::new();
        builder.make_variable("a");
        builder.build_network();
    }

}

use std::ops::{BitOr, BitXor};
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Error};

/// Every state ID is internally stored as u32 and represents a binary encoding
/// of the boolean vector of network variables.
///
/// The ordering of bits corresponds to the ordering of variables in the network,
/// i.e. variable with ID 0 is stored in the least significant bit of the state integer.
/// Note that this results in a counter-intuitive behaviour when state printed as binary
/// string is reversed compared to what one might expect due to the implicit endianness
/// when printing numbers, i.e. state {0,1,0,1,1} is printed as 0b11010.
///
/// This also means that any printing or exporting needs to be performed using the
/// [BooleanNetwork] object, because the state has no information about the actual number
/// of variables it stores.
///
/// StateId overrides bit-or operator (|) for extracting values of specific variables
/// and bit-xor operator (^) for flipping values of specific variables.
#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
pub struct StateId { pub value: u32 }

/// A variable ID represents an index into the variable array of the [BooleanNetwork].
/// Note that you should not store VariableIds extensively as they are rather
/// memory inefficient (value is very small, yet it occupies 32 bits). If you
/// need a lot of variable iterators with small memory footprint, look at [VariableIterator].
#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
pub struct VariableId { value: u32 }

/// Variable iterator is a simple, memory-efficient iterator over variables of
/// the [BooleanNetwork]. You should not assume any specific iteration order!
pub struct VariableIterator {
    remaining: u8
}

/// State iterator is a simple iterator over states of the [BooleanNetwork]. It is not
/// exactly memory efficient so try to avoid creating them excessively. On the other hand,
/// it you are creating millions of state iterators, you have bigger problems to solve...
pub struct StateIterator {
    // Max state is used to avoid integer overflow/underflow when dealing with 2^32 states.
    // Once last state is returned from the iterator, max_state is set to zero, as network
    // with no states cannot exist.
    state: u32, max_state: u32
}

/// Boolean network is a type of simple model with boolean variables and asynchronous update
/// functions.
pub struct BooleanNetwork {
    update_functions: Vec<Box<dyn Fn(StateId) -> bool + Sync>>
}

/// Boolean network builder allows to create instances of [BooleanNetwork] in a relatively
/// safe fashion. Specifically, it check for duplicities and missing values.
pub struct BooleanNetworkBuilder {
    variable_count: u32,
    variable_names: HashMap<VariableId, String>,
    update_functions: HashMap<VariableId, Box<dyn Fn(StateId) -> bool + Sync>>
}

impl BitOr<VariableId> for StateId {
    type Output = bool;

    fn bitor(self, rhs: VariableId) -> Self::Output {
        return (self.value >> rhs.value) & 1 == 1
    }

}

impl BitXor<VariableId> for StateId {
    type Output = StateId;

    fn bitxor(self, rhs: VariableId) -> Self::Output {
        return StateId { value: self.value ^ (1_u32 << rhs.value) }
    }

}

impl Display for VariableId {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_fmt(format_args!("Var({})", self.value))
    }
}

impl Display for StateId {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        f.write_str(&format!("{:#032b}", self.value).chars().rev().collect::<String>())
    }
}

impl Iterator for VariableIterator {
    type Item = VariableId;

    fn next(&mut self) -> Option<Self::Item> {
        return if self.remaining == 0 { None } else {
            self.remaining -= 1;
            Some(VariableId { value: self.remaining as u32 })
        }
    }

}

impl Iterator for StateIterator {
    type Item = StateId;

    fn next(&mut self) -> Option<Self::Item> {
        return if self.max_state == 0 { None } else {
            if self.state == self.max_state {           // last state - clean up
                self.max_state = 0;
                Some(StateId { value: self.state })
            } else {                                    // continue as usual
                self.state += 1;
                Some(StateId { value: self.state - 1 })
            }
        }
    }

}

impl BooleanNetwork {

    pub fn var_count(&self) -> u8 {
        return self.update_functions.len() as u8;
    }

    pub fn state_count(&self) -> u64 {
        // since there possibly are 32 variables, the state count can overflow u32 by 1
        return 1_u64 << self.var_count() as u64;
    }

    pub fn variables(&self) -> VariableIterator {
        return VariableIterator { remaining: self.var_count() }
    }

    pub fn states(&self) -> StateIterator {
        // assuming there are at most 32 variables, state_count - 1 is always within bounds of u32
        return StateIterator { state: 0, max_state: (self.state_count() - 1) as u32 }
    }

    /// Check if [state] has a successor in dimension given by [variable]. If yes,
    /// return such successor, otherwise return [None].
    pub fn successor(&self, state: &StateId, variable: &VariableId) -> Option<StateId> {
        let target: bool = self.update_functions[variable.value as usize](state.clone());
        return if *state | *variable == target { None } else { Some(*state ^ *variable) }
    }

}

impl BooleanNetworkBuilder {

    /// Make a new empty boolean network builder.
    pub fn new() -> BooleanNetworkBuilder {
        return BooleanNetworkBuilder {
            variable_count: 0,
            variable_names: HashMap::new(),
            update_functions: HashMap::new()
        }
    }

    /// Create a new variable in this network.
    /// Panics if the variable already exists or the network is too large.
    pub fn make_variable(&mut self, name: &str) -> VariableId {
        if self.variable_count == 32 { panic!("Cannot create network with more than 32 variables."); }
        let variable = VariableId { value: self.variable_count };
        self.variable_count += 1;
        for (_, existing) in &self.variable_names {
            if name == existing { panic!("Variable named {} already exists.", existing); }
        }
        self.variable_names.insert(variable, String::from(name));
        return variable;
    }

    /// Associate an update function with a variable.
    /// Panics if the variable does not exist or if it already has a function defined.
    pub fn update_function(&mut self, variable: &VariableId, function: Box<dyn Fn(StateId) -> bool + Sync>) {
        if !self.variable_names.contains_key(variable) {
            panic!("Variable #{} does not exist in this boolean network.", variable);
        }
        if self.update_functions.contains_key(variable) {
            panic!("Cannot redefine update function for {}.", self.variable_names[variable])
        }
        self.update_functions.insert(*variable, function);
    }

    /// Consume this builder into a full-fledged boolean network.
    pub fn build_network(mut self) -> BooleanNetwork {
        for (var, name) in self.variable_names.iter() {
            if !self.update_functions.contains_key(var) {
                panic!("Update function for {} not specified.", name)
            }
        }
        let mut functions: Vec<(VariableId, Box<dyn Fn(StateId) -> bool + Sync>)> = self.update_functions.drain().collect();
        functions.sort_by_key(|&(k, _)| k.value);

        return BooleanNetwork {
            update_functions: functions.into_iter().map(|(_, f)| f).collect()
        }
    }

}
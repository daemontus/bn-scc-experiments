use std::ops::{Shl, Shr, BitAnd, BitXor, Rem};
use std::fmt::{Display, Formatter};

pub mod builder;
pub mod generator;

const MAX_VARS: usize = 32;

/// State represents one configuration of variables inside a Boolean network.
/// Currently, it is just a vector of booleans packed into an u32. This gives us an upper
/// bound of 32 variables, but that should be enough for now. Later, it could be extended
/// to u64 if needed (and if there is an actual computational capability of handling
/// that many variables).
///
/// For this reason, we do not expose this u32 value to the world, rather, use provided
/// methods to extract information about states.
///
#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
pub struct State {
    pub index: usize
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        f.write_str(&format!("{:#06b}", self.index))
    }
}

/// Similar to states, variables of a Boolean network are represented by their indices.
/// In this case, the numbers are always quite small (<= 32), but we keep it as u32
/// just for the sake of consistency with the remaining code. The memory overhead is
/// negligible considering we don't store them extensively anywhere.
#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub struct Variable {
    index: usize
}

/// A utility syntax for extracting variable values from states using the % operator.
impl Rem<&Variable> for State {
    type Output = bool;

    fn rem(self, rhs: &Variable) -> Self::Output {
        return self.get(rhs)
    }

}

/// A utility syntax for extracting variable values from states using the % operator.
impl Rem<Variable> for State {
    type Output = bool;

    fn rem(self, rhs: Variable) -> Self::Output {
        return self.get(&rhs)
    }

}

impl State {

    /// Produce a state which exactly represents the given slice of boolean values.
    ///
    /// Panics if the slice is longer than 32 entries.
    pub fn from_data(values: &[bool]) -> State {
        if values.len() > MAX_VARS as usize {
            panic!("Cannot create state with {} variables, {} is maximum.", values.len(), MAX_VARS);
        }
        let mut index = 0;
        // Iteration is reversed since first variable is represented by the least significant bit.
        for d in (0..values.len()).rev() {
            if values[d] {
                index += 1;
            }
            if d > 0 {  // not for the last dimension!
                index = index.shl(1);
            }
        }
        State { index }
    }

    /// Test if given variable is set to true in this state.
    pub fn get(&self, var: &Variable) -> bool {
        return self.index.shr(var.index).bitand(1) == 1
    }

    /// Make a new state with the value of the given variable flipped.
    pub fn flip(&self, var: &Variable) -> State {
        let index = self.index.bitxor(1_usize.shl(var.index));
        return State { index }
    }

}

pub struct BooleanNetwork {
    update_functions: Vec<Box<dyn Fn(&State) -> bool>>
}

pub struct BNStateIterator {
    state_count: usize,
    next_state: usize
}

impl Iterator for BNStateIterator {
    type Item = State;

    fn next(&mut self) -> Option<Self::Item> {
        return if self.next_state == self.state_count {
            None
        } else {
            self.next_state += 1;
            Some(State { index: (self.next_state - 1) })
        }
    }

}

pub struct BNVariableIterator {
    next_var: usize, var_count: usize
}

impl Iterator for BNVariableIterator {
    type Item = Variable;

    fn next(&mut self) -> Option<Self::Item> {
        return if self.next_var == self.var_count {
            None
        } else {
            self.next_var += 1;
            Some(Variable { index: (self.next_var - 1) })
        }
    }
}

impl BooleanNetwork {

    pub fn variable_count(&self) -> usize {
        return self.update_functions.len();
    }

    pub fn state_count(&self) -> usize {
        return 1_usize.shl(self.variable_count());
    }

    pub fn states(&self) -> BNStateIterator {
        return BNStateIterator {
            state_count: self.state_count(), next_state: 0
        }
    }

    pub fn variables(&self) -> BNVariableIterator {
        return BNVariableIterator {
            var_count: self.variable_count(), next_var: 0
        }
    }

    pub fn successor(&self, state: &State, variable: &Variable) -> Option<State> {
        let target_value: bool = self.update_functions[variable.index as usize](state);
        return if *state % variable == target_value { None } else {
            Some(state.flip(variable))
        }
    }

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn state_test() {
        let v1 = Variable { index: 0 };
        let v2 = Variable { index: 1 };
        let v3 = Variable { index: 2 };
        let v4 = Variable { index: 3 };
        let s1 = State::from_data(&[true, false, true, true]);
        assert_eq!(true, s1 % v1);
        assert_eq!(false, s1 % v2);
        assert_eq!(true, s1 % v3);
        assert_eq!(true, s1 % v4);
        let s2 = s1.flip(&v1);
        assert_eq!(false, s2 % v1);
        assert_eq!(false, s2 % v2);
        assert_eq!(true, s2 % v3);
        assert_eq!(true, s2 % v4);
    }

    #[test] #[should_panic]
    fn state_invalid() {
        State::from_data(&vec![true; (MAX_VARS + 1) as usize][..]);
    }

}
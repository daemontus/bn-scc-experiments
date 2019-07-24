//! Defines SCC decomposition for boolean networks of up-to 2^32 states, i.e. 32 variables.
//!
//! This specialized implementation is very useful because it saves a lot of memory compared to
//! full 64-bit version (every pointer is only 4 bytes instead of 8).

pub mod bn;
pub mod models;
pub mod sequential;
pub mod parallel;
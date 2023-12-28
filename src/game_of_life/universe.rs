use super::{BitFieldCompatible, BitsField};

/// A universe is a 2D grid of cells.
/// 
/// Every cell is one of possible states (max 256 states).
/// To represent the state of a cell, we use the minimal amount of bits.
/// Let's say we have k possible states, then we need log2(k) bits to
/// represent the state of a cell.
pub struct Universe {
    width: usize,
    height: usize,
    cells: BitsField<u8>,
}

enum InitPolicy {
    Random{alive_probability: f64},
    Gaussian{alive_probability: f64, sigma: f64},
    Custom{states: Vec<u8>},
}
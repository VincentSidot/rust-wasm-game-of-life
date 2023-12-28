pub mod universe;
pub mod state;
pub mod rules;
pub mod bitsfield;

pub use universe::Universe;
pub use state::State;
pub use bitsfield::{BitsField, BitFieldCompatible, BitFieldRepresentation};
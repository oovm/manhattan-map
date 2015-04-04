mod illustration;
mod joint;
mod direction;
mod sparse_map;

pub use crate::{
    joint::Joint,
    direction::{Direction},
    sparse_map::{TaxicabMap, iters::{ MutGetTaxicabPoints}},
};

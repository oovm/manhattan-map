mod direction;
mod illustration;
mod joint;
mod sparse_map;

pub use crate::{
    direction::Direction,
    joint::Joint,
    sparse_map::{
        iters::{DiamondPoints, GetTaxicabPoints, GetTaxicabPointsAround, MutGetTaxicabPoints},
        TaxicabMap,
    },
};

//! Collection of algorithms that are used by the kernel
//!
//! Algorithmic code is collected in this module, to keep other modules focused
//! on their respective purpose.

mod approximation;
mod sweep;
mod triangulation;

pub use self::{
    approximation::Approximation, sweep::sweep_shape,
    triangulation::triangulate,
};

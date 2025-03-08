#![no_std]
#![warn(clippy::pedantic)]

pub mod core;
pub mod convenience;

pub mod prelude {
    pub use crate::convenience::retrieve;
}

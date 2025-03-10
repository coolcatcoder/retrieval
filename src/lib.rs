#![no_std]
#![warn(clippy::pedantic)]

pub mod convenience;
pub mod core;

pub mod prelude {
    pub use crate::convenience::{collect, iterate};
}

#![no_std]
#![warn(clippy::pedantic)]

pub use retrieval_proc_macros::collect_experiment;

pub mod convenience;
pub mod core;
pub mod deref;

pub mod prelude {
    pub use crate::convenience::{collect, iterate};
}

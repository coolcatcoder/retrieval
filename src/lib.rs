#![no_std]
#![warn(clippy::pedantic)]

pub use retrieval_proc_macros::{iterate, retrieve, send};
pub mod prelude {
    pub use super::{iterate, retrieve, send};
}

#[doc(hidden)]
pub struct Container<const INDEX: usize>;

// #[cfg(test)]
// mod tests {
//     use crate as retrieval;
//     use retrieval::prelude::*;

//     #[retrieve]
//     trait RetrieveTester {

//     }

//     #[retrieve]
//     trait One {
//         const STR: &str = "";
//     }
// }
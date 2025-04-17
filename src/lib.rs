#![no_std]
#![warn(clippy::pedantic)]

pub mod prelude {
    pub use retrieval_proc_macros::{iterate, retrieve, send};
}

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
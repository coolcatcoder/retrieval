//! Tests all kinds of items inside retrieval traits.

#![allow(dead_code)]
use retrieval::*;

#[retrieve]
trait One {
    const A: () = ();
    type B = ();
    type C: Send = ();
    type D<T> = ();
    fn e() {}
}

#[send]
impl One {
    const A: () = ();
    type B = ();
    type C = ();
    type D<T> = ();
    fn e() {}
}

// Tests that due to defaults we can choose to not implement everything except the types.
#[send]
impl One {
    type B = ();
    type C = ();
    type D<T> = ();
}

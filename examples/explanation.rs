//! This example shows how this crate works and what it roughly expands to.

use std::marker::PhantomData;

/// Contains the retrieved implementations.
/// Each implementation is stored under a different INDEX.
struct Container<const INDEX: usize>;

/// The trait that we collect implementations of.
trait Message {
    // The actually useful information we want to retrieve.
    const STR: &str = "";

    /// The next type in the chain.
    type NEXT: Message;
    /// Is this the end of the chain?
    const END: bool = false;
}

/// Self is the same type as T.
/// Used to bypass trivial bounds.
trait Is<T> {}
impl<T> Is<T> for T {}

/// The final implementation.
/// Only implemented once on Container<INDEX>, at the highest INDEX that implements Message.
trait Final {}

/// Abuse inference to find the INDEX that implements Final, and therefore is the highest index.
const LENGTH: usize = {
    const fn get_length<const INDEX: usize>() -> usize
    where
        Container<INDEX>: Final,
    {
        INDEX
    }
    get_length()
};

// Switches allow us to perform a slightly modified auto trait specialisation. (https://github.com/coolcatcoder/rust_techniques/issues/1)
// Basically by implementing Unpin for Switch0<T, false>, then Switch0<T, true> is no longer Unpin.
// T is simply so we can avoid using trivial bounds which aren't allowed.
// Also, because we do not know how many implementations we are going to collect, we generate 1000 switches by default.
struct Switch0<T, const BOOL: bool>(PhantomData<T>);
struct Switch1<T, const BOOL: bool>(PhantomData<T>);
struct Switch2<T, const BOOL: bool>(PhantomData<T>);

// The required 0th implementation, so that we know once we have iterated over every implementation.
impl Message for Container<0> {
    type NEXT = Self;
    const END: bool = true;
}
// Mark INDEX 0 as the final implemention, but only when Switch0<T, true> implements unpin.
impl<T: Is<Container<0>>> Final for T where Switch0<T, true>: Unpin {}

/// Recursively iterate each Message implementation.
fn __internal_collect_messages<T: Message>() {
    if T::END {
        return;
    }
    {
        println!("{}", T::STR);
    };
    __internal_collect_messages::<T::NEXT>();
}
/// Prints every message by starting the chain of recursive __internal_collect_messages calls.
fn collect_messages() {
    __internal_collect_messages::<Container<{ LENGTH }>>();
}

// The first message, so we store it in INDEX 1.
impl Message for Container<1> {
    const STR: &str = "Hello world!";
    type NEXT = Container<0>;
}
// By implementing Unpin for Switch0<T, false> we cause Switch0<T, true> to not implement Unpin.
// This causes Container<0> to no longer be marked as Final.
impl<T> Unpin for Switch0<T, false> {}
// Mark INDEX 1 as the final implemention, but only when Switch1<T, true> implements unpin.
impl<T: Is<Container<1>>> Final for T where Switch1<T, true>: Unpin {}

/// The entry point to the program.
/// Simply calls our collect_messages function in order to print every collected message.
fn main() {
    collect_messages();
}

// The second message, so we store it in INDEX 2.
impl Message for Container<2> {
    const STR: &str = "Hello again!";
    type NEXT = Container<1>;
}
// By implementing Unpin for Switch1<T, false> we cause Switch1<T, true> to not implement Unpin.
// This causes Container<1> to no longer be marked as Final.
impl<T> Unpin for Switch1<T, false> {}
// Mark INDEX 2 as the final implemention, but only when Switch2<T, true> implements unpin.
impl<T: Is<Container<2>>> Final for T where Switch2<T, true>: Unpin {}

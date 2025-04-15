//! This example shows how this crate works and what it roughly expands to.

/// Contains the retrieved implementations.
/// INDEX has 2 purposes.
/// The first being to store each implementation under different indices.
/// The second is tricking inference into finding the last index.
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

/// The final implementation.
/// Only implemented once on Container<INDEX>, at the highest INDEX that implements Message.
trait Final {}

/// Abuse inference to find the INDEX that implements Final, and therefore is the highest index.
const LENGTH: usize = {
    const fn get_length<const INDEX: usize>() -> usize
    where
        crate::Container<INDEX>: Final,
    {
        INDEX
    }
    get_length()
};

// Switches allow us to perform a slightly modified auto trait specialisation.
// For more info see: https://github.com/coolcatcoder/rust_techniques/issues/1
// Basically by implementing Unpin for Switch0<false>, then Switch0<true> is no longer Unpin.
// Also, because we do not know how many implementations we are going to collect, we generate 1000 switches by default.
struct Switch0<const BOOL: bool>;
struct Switch1<const BOOL: bool>;
struct Switch2<const BOOL: bool>;

// The required 0th implementation, so that we know once we have iterated over every implementation.
impl Message for crate::Container<0> {
    type NEXT = Self;
    const END: bool = true;
}
// Mark INDEX 0 as the final implemention, but only when Switch0<true> implements unpin.
impl Final for Container<0> where for<'a> Switch0<true>: Unpin {}

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
impl Message for crate::Container<1> {
    const STR: &str = "Hello world!";
    type NEXT = crate::Container<0>;
}
// By implementing Unpin for Switch0<false> we cause Switch0<true> to not implement Unpin.
// This causes Container<0> to no longer be marked as Final.
impl Unpin for Switch0<false> {}
// Mark INDEX 1 as the final implemention, but only when Switch1<true> implements unpin.
impl Final for Container<1> where for<'a> Switch1<true>: Unpin {}

/// The entry point to the program.
/// Simply calls our collect_messages function in order to print every collected message.
fn main() {
    collect_messages();
}

// The second message, so we store it in INDEX 2.
impl Message for crate::Container<2> {
    const STR: &str = "Hello again!";
    type NEXT = crate::Container<1>;
}
// By implementing Unpin for Switch1<false> we cause Switch1<true> to not implement Unpin.
// This causes Container<1> to no longer be marked as Final.
impl Unpin for Switch1<false> {}
// Mark INDEX 2 as the final implemention, but only when Switch2<true> implements unpin.
impl Final for Container<2> where for<'a> Switch2<true>: Unpin {}

Allows the retrieval of trait implementations.  
[Documentation](https://docs.rs/retrieval/)  
[Crate](https://crates.io/crates/retrieval)  
[Change Log](./CHANGELOG.md)
## Example
```rust
use retrieval::*;

#[retrieve]
trait Message {
    const STR: &str = "";
}

#[iterate]
fn collect_messages<T: Message>() {
    println!("{}", T::STR);
}

#[send]
impl Message {
    const STR: &str = "Hello world!";
}

fn main() {
    // Will print "Hello world!", and "Hello again!".
    collect_messages();
}

#[send]
impl Message {
    const STR: &str = "Hello again!";
}
```
## Explanation
Imagine if you could store a list of types, consts, and functions, all at compile time.  
There are various ways of accomplishing that, but now what if instead you could automatically generate that list from desired items located anywhere in your crate?  
It is possible, using this crate.  

How? Simple, we create a trait that holds the items we want to collect. Then we use an attribute proc macro, that you put on each trait implementation containing the items you want to send to the list.  
Every invocation of the attribute proc macro assumes that it is the last. When it gets invoked again, it simply unimplements the last invocation.

Here is roughly an example of the techniques we use for this: [Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=575d81e8174d148d03a9ac906be03b60)
```rust
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
// Basically by implementing Unpin for Switch0<false>, then Switch0<true> is no longer Unpin.
// Also, because we do not know how many implementations we are going to collect, we generate 1000 switches by default.
struct Switch0<const BOOL: bool>;
struct Switch1<const BOOL: bool>;
struct Switch2<const BOOL: bool>;

// The required 0th implementation, so that we know once we have iterated over every implementation.
impl Message for Container<0> {
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
impl Message for Container<1> {
    const STR: &str = "Hello world!";
    type NEXT = Container<0>;
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
impl Message for Container<2> {
    const STR: &str = "Hello again!";
    type NEXT = Container<1>;
}
// By implementing Unpin for Switch1<false> we cause Switch1<true> to not implement Unpin.
// This causes Container<1> to no longer be marked as Final.
impl Unpin for Switch1<false> {}
// Mark INDEX 2 as the final implemention, but only when Switch2<true> implements unpin.
impl Final for Container<2> where for<'a> Switch2<true>: Unpin {}
```
This unfortunately does require us to be able to count in the proc macro, which sadly means we have to use static abuse...
This may not work with proc macro caching, and in fact could stop working at any moment. We are hopeful that rust will add proper state to proc macros before they break this trick.

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

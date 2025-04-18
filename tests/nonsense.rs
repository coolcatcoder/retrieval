//! Tests any random nonsense.

use retrieval::*;

#[retrieve(10)]
trait Message {
    const STR: &str = "";
}

#[retrieve]
trait Different {
    type Blah = ();
    type Two = ();
    fn bubble() {}
}

#[send]
impl Different {
    type Blah = ();
    type Two = i32;
    fn bubble() {
        println!("Bubble!!");
    }
}

#[iterate]
fn do_bubble<B: Different>() {
    B::bubble();
}

#[iterate]
const fn collect_messages<T: Message>(messages: &mut [&str], index: &mut usize) {
    messages[*index] = T::STR;
    *index += 1;
}

#[send]
impl Message {
    const STR: &str = "Hello world!";
}

#[send]
impl Message {
    const STR: &str = "So cool!";
}
#[send]
impl Message {
    const STR: &str = "Wow!";
}

/// Outputs:
/// Wow!
/// So cool!
/// Hello world!
fn main() {
    let mut messages = [""; message::QUANTITY];
    let mut index = 0;
    collect_messages(&mut messages, &mut index);
    messages[0..index].into_iter().for_each(|message| {
        println!("{}", message);
    });
    do_bubble();
}

mod grah {
    use crate::{Different, Message};
    use retrieval::*;

    #[send]
    impl Message {
        const STR: &str = "Hello from grah!";
    }

    #[iterate]
    const fn wow<T: Different>(messages: &mut [&str], index: &mut usize) {}
}

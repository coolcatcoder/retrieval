use retrieval::prelude::*;

#[retrieve]
trait Message {
    const STR: &str = "";
}

#[retrieve]
trait Different {
    //type Blah = ();
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
    let mut messages = [""; message::LENGTH];
    let mut index = 0;
    collect_messages(&mut messages, &mut index);
    messages[0..index].into_iter().for_each(|message| {
        println!("{}", message);
    });
}

mod grah {
    use crate::{Different, Message};
    use retrieval_proc_macros::{iterate, send};

    #[send]
    impl Message {
        const STR: &str = "Hello from grah!";
    }

    #[iterate]
    const fn wow<T: Different>(messages: &mut [&str], index: &mut usize) {}
}

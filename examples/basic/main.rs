use retrieval::prelude::*;

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

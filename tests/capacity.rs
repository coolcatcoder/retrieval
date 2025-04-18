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

#[send]
impl Message {
    const STR: &str = "Hello again!";
}

fn main() {
    // Will print "Hello world!", and "Hello again!".
    collect_messages();
}

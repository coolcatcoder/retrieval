use retrieval::{
    collect_experiment,
    deref::{Checkpoint, DerefOnly, Reason},
};
use retrieval_proc_macros::something;

#[collect_experiment]
trait Has {
    const VALUE: &str = "";
}

fn print_messages<T: Has>() {
    println!("{}", T::VALUE);
}

const fn collect<T: Has>(messages: &mut [Option<&str>], index: &mut usize, _: &dyn FnOnce() -> T) {
    match T::REASON {
        Reason::Add => {
            messages[*index] = Some(T::VALUE);
            *index += 1;
        }
        Reason::Checkpoint => (),
        Reason::End => return,
    }

    collect(messages, index, &|| T::__next());
}

const fn collect_messages() -> [Option<&'static str>; 100] {
    let mut messages = [None; 100];
    let mut index = 0;
    collect(&mut messages, &mut index, &|| {
        DerefOnly::<&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&Checkpoint<0>>::new().__get_self()
    });
    messages
}

const MESSAGES: [Option<&str>; 100] = collect_messages();

fn main() {
    MESSAGES.iter().for_each(|message| {
        if let Some(message) = message {
            println!("{}", message);
        }
    });
}

#[something]
impl Has {
    const VALUE: &'static str = "Automatic!";
}
#[something]
impl Has {
    const VALUE: &'static str = "Automatic!";
}
#[something]
impl Has {
    const VALUE: &'static str = "Automatic!";
}
#[something]
impl Has {
    const VALUE: &'static str = "Automatic!";
}

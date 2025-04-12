use retrieval_proc_macros::{collect_unimpl, drop_send, something_unimpl};

struct __RetrievalBlah;

// SETUP
//testing_drop!();
#[collect_unimpl]
trait TraitChain {
    const STR: &str = "";
}

// FAR AWAY IMPLS
#[something_unimpl]
impl TraitChain {
    const STR: &str = "Hello world!";
}

// COLLECTION:
const fn get_length<const INDEX: usize>() -> usize
where
    Container<INDEX>: Final,
{
    INDEX
}

const fn collect_messages_1<T: TraitChain>(messages: &mut [&str], index: &mut usize) {
    if T::END {
        return;
    }

    messages[*index] = T::STR;
    *index += 1;

    collect_messages_2::<T::NEXT>(messages, index);
}

const fn collect_messages_2<T: TraitChain>(messages: &mut [&str], index: &mut usize) {
    if T::END {
        return;
    }

    messages[*index] = T::STR;
    *index += 1;

    collect_messages_1::<T::NEXT>(messages, index);
}

const GOTTEN_LENGTH: usize = get_length();

fn for_each_collected<T>() {}

fn main() {
    println!("Hello world! {}", GOTTEN_LENGTH);
    let mut messages = [""; 1000];
    let mut index = 0;
    collect_messages_1::<Container<GOTTEN_LENGTH>>(&mut messages, &mut index);
    messages[0..index].into_iter().for_each(|message| {
        //println!("{}", message);
    });
}

#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
#[something_unimpl]
impl TraitChain {
    const STR: &str = "So cool!";
}
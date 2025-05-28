//! Tests that every macro except retrieve works in any location.
#![allow(clippy::module_inception)]
#![allow(dead_code)]
#![allow(non_local_definitions)]
use retrieval::*;

mod other;

trait Cake {}
pub struct RedVelvet;
impl Cake for RedVelvet {}
#[retrieve]
trait Something {
    type Has: Cake = RedVelvet;
}

#[retrieve]
trait Number {
    const N: u8 = 0;
}

#[iterate]
const fn collect_messages<T: Number>(messages: &mut [u8], index: &mut usize) {
    messages[*index] = T::N;
    *index += 1;
}

#[test]
fn main() {
    let mut messages = [0; Number::QUANTITY as usize];
    let mut index = 0;
    collect_messages(&mut messages, &mut index);

    let mut other_messages = [0; Number::QUANTITY as usize];
    let mut other_index = 0;
    other::blah::blah::collect_messages_other(&mut other_messages, &mut other_index);

    [1, 2, 3, 4, 5, 6, 7, 8].iter().for_each(|value| {
        assert!(messages.contains(value));
        assert!(other_messages.contains(value));
    });
}

#[send]
impl Number {
    const N: u8 = 1;
}

mod blah {
    use super::Number;
    use retrieval::send;

    #[send]
    impl Number {
        const N: u8 = 2;
    }

    mod blah {
        use super::Number;
        use retrieval::send;

        #[send]
        impl Number {
            const N: u8 = 3;
        }
    }
}

fn grah() {
    #[send]
    impl Number {
        const N: u8 = 4;
    }
}

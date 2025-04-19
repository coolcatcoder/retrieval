use crate::Number;
use retrieval::send;

#[send]
impl Number {
    const N: u8 = 5;
}

pub mod blah {
    use crate::Number;
    use retrieval::send;

    #[send]
    impl Number {
        const N: u8 = 6;
    }

    pub mod blah {
        use retrieval::{iterate, send};

        #[send]
        impl crate::Number {
            const N: u8 = 7;
        }

        #[iterate]
        pub const fn collect_messages_other<T: crate::Number>(
            messages: &mut [u8],
            index: &mut usize,
        ) {
            messages[*index] = T::N;
            *index += 1;
        }
    }
}

fn grah() {
    #[send]
    impl Number {
        const N: u8 = 8;
    }
}

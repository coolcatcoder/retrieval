use crate::Number;
use retrieval::send;

#[send]
impl Number {
    const N: u8 = 5;
}

mod blah {
    use crate::Number;
    use retrieval::send;

    #[send]
    impl Number {
        const N: u8 = 6;
    }

    mod blah {
        use crate::Number;
        use retrieval::send;

        #[send]
        impl Number {
            const N: u8 = 7;
        }
    }
}

fn grah() {
    #[send]
    impl Number {
        const N: u8 = 8;
    }
}

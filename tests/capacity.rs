use retrieval::*;

#[retrieve(2)]
trait Number {
    const NUMBER: u8 = 0;
}

#[iterate(2)]
const fn testing<T: Number>() {
    assert!((T::NUMBER == 5) || (T::NUMBER == 112));
}

#[send]
impl Number {
    const NUMBER: u8 = 5;
}

#[send]
impl Number {
    const NUMBER: u8 = 112;
}

#[test]
fn main() {
    testing();
}

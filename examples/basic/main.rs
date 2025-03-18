use retrieval::prelude::*;

mod other;

macro_rules! magic {
    ($($_:tt)*) => {};
}

magic! {
    retrieval HasStr {
        const VALUE: &str;
    }

    retrieval fn collect_value<T: HasStr>(vec: &mut Vec<&str>) {
        if let Some(value) = T::VALUE {
            vec.push(value);
        }
    }

    #[collect]
    trait HasStr {
        const VALUE: Option<&str> = None;
    }

    #[iterate]
    fn collect_values<T: HasStr>(values: &mut Vec<&str>) {

    }

    fn main() {
        let mut values = vec![];
        collect_values(&mut values);
    }
}

#[collect]
trait HasStr {
    const VALUE: Option<&str> = None;
}

#[iterate]
fn collect_values<T: HasStr>(values: &mut Vec<&str>) {
    if let Some(value) = T::VALUE {
        values.push(value);
    }
}

fn main() {
    let mut values = vec![];
    collect_values(&mut values);
    dbg!(values);
}

impl crate::HasStr for retrieval::core::Element<1> {
    const VALUE: Option<&str> = Some("Hello!!");
}

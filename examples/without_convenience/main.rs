mod other;

trait HasStr: Sized {
    fn get_self(self) -> Self {
        self
    }

    const VALUE: Option<&str> = None;
}

impl HasStr for retrieval::core::DefaultElement {}

fn collect_value<T: HasStr>(_: &impl FnOnce() -> T, vec: &mut Vec<&str>) {
    if let Some(value) = T::VALUE {
        vec.push(value);
    }
}

fn main() {
    let mut values = vec![];

    // TODO: I want this to be retrieval::core::retrieve.
    retrieval::retrieve!(100, collect_value, &mut values);

    dbg!(values);
}

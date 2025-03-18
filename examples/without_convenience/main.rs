mod other;

trait HasStr: Sized {
    fn __get_self(self) -> Self {
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

// fn collect_value<T: HasStr>(_: &impl FnOnce() -> T, vec: &mut Vec<&str>) {
//     if let Some(value) = T::VALUE {
//         vec.push(value);
//     }
// }

fn main() {
    let mut values = vec![];

    // TODO: I want this to be retrieval::core::retrieve.
    //retrieval::retrieve!(100, collect_value, &mut values);
    macro_rules! repeat_function {
        ($i:expr) => {
            collect_value(&|| retrieval::core::Element::<$i>.__get_self(), &mut values);
        };
    }

    retrieval::core::macro_counter!(repeat_function 100);

    dbg!(values);

    for_each_element::<Number0>();
    //for_each_element_part_1::<Number0>();
}

trait NextElement: HasValue {
    const ROOT: bool = false;
    type Next: NextElement;
}

trait HasValue {
    const VALUE: Option<&str> = None;
}

trait CanHaveValue {}

impl<T: CanHaveValue + Unpin> HasValue for T {}

macro_rules! create_structs {
    ($i:ident, $i_plus_one:ident) => {
        struct $i;
        impl CanHaveValue for $i {}
        impl NextElement for $i {
            type Next = $i_plus_one;
        }
    };
}

const fn uncallable()
where
    for<'dummy> [()]: Sized,
{
}

// Errors!
//const _: () = uncallable();

retrieval::core::macro_counter_ident!(create_structs 10000);

struct Number10000;

impl CanHaveValue for Number10000 {}
impl NextElement for Number10000 {
    const ROOT: bool = true;
    type Next = Self;
}

impl Unpin for Number101 where for<'dummy> [()]: Sized {}
impl HasValue for Number101 {
    const VALUE: Option<&str> = Some("Wow!");
}

fn for_each_element<T: NextElement + HasValue>() {
    if let Some(value) = T::VALUE {
        dbg!(value);
    }

    if T::ROOT {
        return;
    }

    for_each_element::<T::Next>();
}

const fn for_each_element_part_1<T: NextElement + HasValue>() {
    if T::ROOT {
        return;
    }

    for_each_element_part_2::<T::Next>();
}

const fn for_each_element_part_2<T: NextElement + HasValue>() {
    if T::ROOT {
        return;
    }

    for_each_element_part_3::<T::Next>();
}

const fn for_each_element_part_3<T: NextElement + HasValue>() {
    if T::ROOT {
        return;
    }

    for_each_element_part_4::<T::Next>();
}

const fn for_each_element_part_4<T: NextElement + HasValue>() {
    if T::ROOT {
        return;
    }

    for_each_element_part_5::<T::Next>();
}

const fn for_each_element_part_5<T: NextElement + HasValue>() {
    if T::ROOT {
        return;
    }

    for_each_element_part_6::<T::Next>();
}

const fn for_each_element_part_6<T: NextElement + HasValue>() {
    if T::ROOT {
        return;
    }

    for_each_element_part_7::<T::Next>();
}

const fn for_each_element_part_7<T: NextElement + HasValue>() {
    if T::ROOT {
        return;
    }

    for_each_element_part_8::<T::Next>();
}

const fn for_each_element_part_8<T: NextElement + HasValue>() {
    if T::ROOT {
        return;
    }

    for_each_element_part_1::<T::Next>();
}

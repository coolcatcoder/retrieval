#![feature(generic_arg_infer)]

use std::marker::PhantomData;

trait Want {}

trait Multiple<A, B=()> {
    const IS: bool;
}

impl<T:Want> Multiple<T, ()> for () {
    const IS: bool = true;
}
impl<T> Multiple<T, bool> for () {
    const IS: bool = false;
}

struct Nonsense;
impl Want for Nonsense {}

const fn work_out<T, O>() where (): Multiple<T, O> {}

//const _:() = work_out::<Nonsense>();

mod fn_traits {
    pub trait FnOnce<Args> {
        type Output;
    }

    impl<F, R> FnOnce<()> for F
    where
        F: ::core::ops::FnOnce() -> R,
    {
        type Output = R;
    }
}

/// Taken from never-say-never
pub type Never = <fn() -> ! as fn_traits::FnOnce<()>>::Output;

//struct Mess<N, const I: N>(PhantomData<N>);

trait NotTraitAPart1 {
    const CLONER: bool;
}
trait NotTraitA {}
impl<T> NotTraitAPart1 for T {
    const CLONER: bool = {
        const fn has_trait<H: Holder<T>, T>(_: &H) -> bool {
            H::HAS_TRAIT_A
        }

        has_trait::<_, T>(&0)
    };
}
//impl<N: Holder<T>, T> NotTraitAPart1<T> for N {}
//impl<T> NotTraitA for T where {0}: {}
//impl<T, H: Holder<T>> NotTraitA for T {}

trait TraitA {}
trait TraitB {}

trait Holder<T> {
    const HAS_TRAIT_A: bool;
}

impl<T: TraitA> Holder<T> for i32 {
    const HAS_TRAIT_A: bool = true;
}
impl<T> Holder<T> for u32 {
    const HAS_TRAIT_A: bool = false;
}

trait HolderB<T> {
    const HAS_TRAIT_B: bool;
}

impl<T: TraitB> HolderB<T> for Never {
    const HAS_TRAIT_B: bool = true;
}
impl<T> HolderB<T> for () {
    const HAS_TRAIT_B: bool = false;
}

struct Struct1;
impl TraitA for Struct1 {}
impl TraitB for Struct1 {}

struct Struct2;

const fn has_trait_a<T>() -> bool {
    const fn inner<H: Holder<T>, T>(_: &H) -> bool {
        H::HAS_TRAIT_A
    }

    inner::<_, T>(&0)
}

const fn special_b<H: HolderB<T>, T>(_: &impl FnOnce() -> H) -> bool {
    H::HAS_TRAIT_B
}

const fn special<H: Holder<T>, T>(_: &H) -> bool {
    H::HAS_TRAIT_A
}

fn main() {
    let struct_1_has_trait_a = special::<_, Struct1>(&0);
    let struct_1_new = has_trait_a::<Struct1>();
    let struct_1_impl = Struct1::CLONER;
    //let struct_1_never = special_b::<_, Struct2>(&||{let foo = todo!(); foo});
    let struct_2_has_trait_a = special::<_, Struct2>(&0);
    println!("Struct1 {}", struct_1_has_trait_a);
    println!("Struct1 {} new", struct_1_new);
    println!("Struct1 {} impl", struct_1_impl);
    //println!("Struct2 {} never", struct_1_never);
    println!("Struct2 {}", struct_2_has_trait_a);
}

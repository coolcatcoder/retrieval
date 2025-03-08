pub use retrieval_proc_macros::macro_counter;
use core::ops::Deref;

/// An element of the trait collection.
/// It may help to think of this as an Option. It may contain your collected traits, or it may fall back to your implementation on [`DefaultElement`].
#[derive(Clone, Copy)]
pub struct Element<const INDEX: u32>;

/// An element that has not been overwritten.
/// It may help to think of this as the None variant of an Option.
/// You still need to implement your collected traits for `DefaultElement`, so that it can be passed on to your functions.
#[derive(Clone, Copy)]
pub struct DefaultElement;

impl<const INDEX: u32> Deref for Element<INDEX> {
    type Target = DefaultElement;

    fn deref(&self) -> &Self::Target {
        &DefaultElement
    }
}

#[macro_export]
macro_rules! retrieve {
    ($max:expr, $function:ident, $($argument:expr),*) => {
        {
            macro_rules! repeat_function {
                ($i:expr) => {
                    $function(&||{$crate::core::Element::<$i>.internal_do_not_use_directly_get_self()}, $($argument),*);
                };
            }

            $crate::core::macro_counter!(repeat_function $max);
        }
    };
}

// trait ReturnType {
//     type Return;
// }

// impl<T> ReturnType for FnOnce() -> T {
//     type Return = T;
// }

// #[derive(Clone, Copy)]
// struct Root;

// trait HasStr: Sized {
//     fn get_self(self) -> Self {
//         self
//     }

//     type Next;
//     fn next() -> Self::Next;

//     const IS_ROOT: bool = false;

//     const STR: &str;
// }

// impl HasStr for Root {
//     type Next = Root;
//     fn next() -> Self::Next {
//         Root
//     }

//     const IS_ROOT: bool = true;

//     const STR: &str = "Default.";
// }

// impl HasStr for &&&Root {
//     type Next = &'static &'static Root;
//     fn next() -> Self::Next {
//         &&Root
//     }

//     const STR: &'static str = "Default.";
// }

// // const fn do_something<T: HasStr>(_: &impl FnOnce() -> T, messages: &mut Vec<&str>) {
// //     if T::IS_ROOT {
// //         return;
// //     }

// //     do_something(&|| {T::next().get_self()}, messages);
// // }
// fn do_something<T: HasStr>(_: &dyn FnOnce() -> T, messages: &mut Vec<&str>) {
//     messages.push(T::STR);

//     if T::IS_ROOT {
//         return;
//     }

//     do_something(&|| {T::next().get_self()}, messages);
// }

// // const fn tester() {
// //     do_something(&|| {(&&&&&&&&&&&&&Root).get_self()});
// // }

// pub fn check() {
//     let mut messages = vec![];
//     do_something(&|| {(&&&&&&&&&&&&&Root).get_self()}, &mut messages);

//     println!("Messages: {:?}", messages);
// }

// OTHER FAIL: !!!!!!!!!!!

// trait HasStr {
//     const STR: &str;
// }

// struct BoolStruct<const BOOL: bool>;

// trait BoolStructToBool {
//     const BOOL: bool;
// }

// impl<const BOOL: bool> BoolStruct<BOOL> {
//     const BOOL: bool = BOOL;
// }

// struct HasImplementation<T>(PhantomData<T>);
// struct NoImplementation;

// impl<T> Deref for HasImplementation<T> {
//     type Target = NoImplementation;
//     fn deref(&self) -> &Self::Target {
//         &NoImplementation
//     }
// }

// impl<T: HasStr> HasImplementation<T> {
//     fn check(&self) -> BoolStruct<true> {
//         BoolStruct
//     }
// }

// impl NoImplementation {
//     fn check(&self) -> BoolStruct<false> {
//         BoolStruct
//     }
// }

// THIS MAY WORK !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

// pub trait GetDeref: Deref where <Self as Deref>::Target: GetDeref {
//     fn get() -> <Self as Deref>::Target;
// }

// pub trait GetDeref: Deref {
//     type Target: GetDeref + ?Sized;
//     fn get(&self) -> &<Self as GetDeref>::Target;
// }

// impl<T: Deref> GetDeref for T where <T as Deref>::Target: GetDeref {
//     type Target = <Self as Deref>::Target;
//     fn get(&self) -> &<Self as GetDeref>::Target {
//         self
//     }
// }

// fn do_something<T: GetDeref>(_: &dyn FnOnce() -> T, messages: &mut Vec<&str>) {
//     messages.push("occurred");

//     do_something(&|| {
//         // Panic can create anything.
//         // Because this closure will never be called!
//         // So cursed!
//         let value: T = panic!();

//         value.get()
//     }, messages);
// }

// pub fn check() {

// }

// trait Unless {}

// trait Bad {}

// trait HasBool {
//     const BOOL: bool;
// }

// impl<T> Unless for T where
//     [(); {
//         struct Foo;

//         impl Unpin for Foo where for<'dummy> [()]: Sized {}

//         0 + 5
//     }]:
// {
// }

// struct Root;

// impl GetDeref for Root {
//     type Target = Root;
// }

// struct W<T>(PhantomData<T>);
// struct Wrapper<T>(PhantomData<T>);

// pub trait HasStr {
//     const IS_ROOT: bool = false;
//     const STR: &str;
// }

// impl<T: Unpin> HasStr for T {
//     const STR: &str = "Default";
// }

// trait Foo {}

// impl<T: Foo> Unpin for W<T> where for<'dummy> [()]: Sized {}

// trait FakeUnpin {}
// impl<T: Unpin> FakeUnpin for T {}

//impl FakeUnpin for PhantomData<&&&Root> where for<'dummy> [()]: Sized {}

// impl Unpin for Root where for<'dummy> [()]: Sized {}
// impl HasStr for Root {
//     const IS_ROOT: bool = true;
//     const STR: &'static str = "Root";
// }

// // TODO: Try phantom data instead of wrapper.
// impl Unpin for Wrapper<&&&Root> where for<'dummy> [()]: Sized {}
// impl HasStr for Wrapper<&&&Root> {
//     const STR: &'static str = "New!";
// }

// pub trait GetDeref: HasStr + Sized {
//     type Target: GetDeref;
// }

// impl<T: Deref + HasStr> GetDeref for T
// where
//     <Self as Deref>::Target: GetDeref,
// {
//     type Target = <Self as Deref>::Target;
// }

// // impl<T: GetDeref + Sync> GetDeref for W<T> where Self: HasStr {
// //     type Target = T;
// // }

// fn recursive<T: GetDeref>() {
//     println!("{}", T::STR);
//     return;
//     if T::IS_ROOT {
//         return;
//     }
//     recursive::<T::Target>();
// }

// pub fn check() {
//     // TODO: Make root have a const generic index, and then call this once for every index. Gets around recursion limit.
//     //recursive::<&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&Root>();
//     //recursive::<Wrapper<&&&Root>>();
//     //recursive::<W<Root>>();
//     //println!("{}", WHAT);
// }

// macro_rules! create_structs {
//     ($i:expr) => {
//         paste::paste! {
//             struct [<Wrap $i>];

//             impl<[<Wrap $i>]: Rewrite> !Unpin for [<Wrap $i>] {}
//         }
//     };
// }

// macro_counter!(create_structs 5000);

// struct Blah;

// impl Unpin for Blah where
//     [(); {
//         impl !Unpin for Blah where for<'dummy> [()]: Sized {}
//         0
//     }]:
// {
// }

// struct Wrap<const INDEX: u32>;

// impl Unpin for Wrap<0> where for<'dummy> [()]: Sized {}
// // This has the same effect if you want to use the unstable feature negative_impls.
// //impl !Unpin for Wrap<NotUnpin> {}

// trait NotImplemented {}

// struct Blah;

// impl !Unpin for Blah where Self: NotImplemented {}

// const fn needs_unpin<T: Unpin>() {}

// // Error, as Blah does not implement Unpin.
// const _: () = needs_unpin::<Blah>();

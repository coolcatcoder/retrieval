pub use retrieval_proc_macros::{collect, iterate};

macro_rules! dollar_sign {
    () => {};
}

macro_rules! trait_collection {
    ($name:ident {$($body:item)*} ) => {
        mod $name {
            pub trait Storage: $crate::core::GetSelf {
                $($body)*
            }
            impl Storage for $crate::core::DefaultElement {}

            // #[macro_export]
            // macro_rules! for_each_element {
            //     ($function:ident, $crate::dollar_sign!()($argument:expr),*) => {
            //         // Doesn't quite work.
            //         // fn call<T: $trait>(_: &impl FnOnce() -> T) {
            //         //     $function<T>($($argument),*);
            //         // }

            //         macro_rules! repeat_function {
            //             ($i:expr) => {
            //                 //call(&||{Element::<$i>.check()})
            //                 $function(&||{Element::<$i>.get_self()}, $crate::dollar_sign!()($argument),*);
            //             };
            //         }

            //         macro_counter!(repeat_function 1000);
            //     };
            // }
            // pub use for_each_element;
        }
    };
}

use core::{marker::PhantomData, ops::Deref};

/// DerefOnly<T> will dereference into <T as Deref>::Target.
/// This wrapper exists so that rust can only make the root type less wrapped, not more.
#[derive(Clone, Copy)]
pub struct DerefOnly<T: ?Sized>(PhantomData<T>);

impl<T> DerefOnly<T> {
    pub fn new() -> Self {
        DerefOnly(PhantomData)
    }
}

impl<T: Deref> Deref for DerefOnly<T> {
    type Target = DerefOnly<T::Target>;

    fn deref(&self) -> &Self::Target {
        &DerefOnly(PhantomData)
    }
}

/// Rust has a recursion limit of 256, to get around that we have a recursion 'checkpoint' every 256 dereferences.
#[derive(Clone, Copy)]
pub struct Checkpoint<const INDEX: u32>;

/// For what reason was this implementation added?
pub enum Reason {
    /// User implementation
    Add,
    /// Deal with recursion limit.
    /// See [Checkpoint].
    Checkpoint,
    /// All elements have been checked.
    End,
}

use core::{Plug, Unplug};

pub trait Functor: Unplug + Plug<unplug!(Self, A)> {
    fn fmap<B, F>(self, f: F) -> plug!(Self[B])
    where
        Self: Plug<B>,
        F: Fn(Self::A) -> B;
}

impl<T> Functor for Box<T> {
    fn fmap<B, F>(self, f: F) -> plug!(Self[B])
    where
        F: Fn(Self::A) -> B,
    {
        Box::new(f(*self))
    }
}

impl<T> Functor for Vec<T> {
    fn fmap<B, F>(self, f: F) -> plug!(Self[B])
    where
        F: Fn(Self::A) -> B,
    {
        self.into_iter().map(f).collect()
    }
}

impl<T> Functor for Option<T> {
    fn fmap<B, F>(self, f: F) -> plug!(Self[B])
    where
        F: Fn(Self::A) -> B,
    {
        self.map(f)
    }
}

#[cfg(test)]
pub fn it_compiles<A, B, C, F, G, T>(x: T, f: F, g: G) -> plug!(T[C])
where
    T: Functor + Plug<A> + Plug<B> + Plug<C> + Unplug<A = A>,
    unplug!(T, F): Plug<A>,
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    // Would ideally also be writable as x.fmap(f).fmap(g), but Rust's type
    // checker is not quite able to manage that through this kludge.
    x.fmap(|x| g(f(x)))
}

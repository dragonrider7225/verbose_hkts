use core::{Plug, Unplug};
use functor::Functor;

pub trait Applicative: Functor {
    fn pure(s: Self::A) -> Self;
    fn ap<B, F>(self, f: plug!(Self[F])) -> plug!(Self[B])
    where
        F: Fn(Self::A) -> B,
        Self: Plug<F> + Plug<B> + Unplug,
        plug!(Self[F]): Unplug<F = Self::F, A = F> + Plug<F> + Clone,
        Self::F: Plug<F>;
}

impl<T> Applicative for Box<T> {
    fn pure(x: T) -> Box<T> {
        Box::new(x)
    }

    fn ap<B, F>(self, f: plug!(Self[F])) -> plug!(Self[B])
    where
        F: Fn(Self::A) -> B,
    {
        Box::new((*f)(*self))
    }
}

impl<T: Clone> Applicative for Vec<T> {
    fn pure(x: T) -> Vec<T> {
        vec![x]
    }

    fn ap<B, F>(self, f: plug!(Self[F])) -> plug!(Self[B])
    where
        F: Fn(Self::A) -> B,
        plug!(Self[F]): Clone,
    {
        let ret: Vec<B> = self.into_iter()
            .flat_map(|x: T| f.clone().fmap(|f: F| f(x.clone())))
            .collect();
        ret
    }
}

impl<T> Applicative for Option<T> {
    fn pure(x: T) -> Option<T> {
        Some(x)
    }

    fn ap<B, F>(self, f: plug!(Self[F])) -> plug!(Self[B])
    where
        F: Fn(Self::A) -> B,
    {
        match (self, f) {
            (Some(x), Some(f)) => Some(f(x)),
            _ => None,
        }
    }
}

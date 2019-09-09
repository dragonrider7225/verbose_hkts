use applicative::Applicative;
use core::Plug;

pub trait Monad: Applicative {
    fn bind<F, B>(self, f: F) -> plug!(Self[B])
    where
        Self: Plug<F> + Plug<B>,
        F: Fn(Self::A) -> plug!(Self[B]);
}

impl<T> Monad for Box<T> {
    fn bind<F, B>(self, f: F) -> plug!(Self[B])
    where
        Self: Plug<F> + Plug<B>,
        F: Fn(Self::A) -> plug!(Self[B]),
    {
        f(*self)
    }
}

impl<T: Clone> Monad for Vec<T> {
    fn bind<F, B>(self, f: F) -> plug!(Self[B])
    where
        F: Fn(Self::A) -> plug!(Self[B]),
    {
        let res: Vec<B> = self.into_iter().flat_map(f).collect();
        res
    }
}

impl<T> Monad for Option<T> {
    fn bind<F, B>(self, f: F) -> plug!(Self[B])
    where
        F: Fn(Self::A) -> plug!(Self[B]),
    {
        match self {
            Some(x) => f(x),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use core::{Plug, Unplug};
    use super::*;

    /// wew lad
    fn higher_poly_demo<'a, M, A, B, F>(m: M, f: F) -> plug!(M[B])
    where
        M: Monad + Plug<A> + Plug<B> + Plug<F> + Unplug<A = A>
            + Plug<Box<dyn Fn(A) -> plug!(M[B])>>,
        A: 'a + Clone,
        B: 'a + Clone,
        F: 'static + Fn(A) -> B,
        // F: Fn(A) -> plug!(M[B]) + Clone,
        unplug!(M, F): Plug<A> + Plug<B>,
        plug!(M[B]): 'a + Monad + Unplug<A = B>,
        unplug!(plug!(M[B]), F): Plug<B>,
    {
        let cl = Box::new(move |x| Applicative::pure(f(x)));
        m.bind::<Box<dyn Fn(A) -> _>, B>(cl)
    }

    #[test]
    fn use_higher_poly() {
        let f = |x| x + 1;
        let p1 = Some(5);
        let p2 = vec![5];
        let p3 = Box::new(5);
        assert_eq!(higher_poly_demo(p1, f), Some(6));
        assert_eq!(higher_poly_demo(p2, f), vec![6]);
        assert_eq!(higher_poly_demo(p3, f), Box::new(6));
    }
}

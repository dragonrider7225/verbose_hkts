pub struct Concrete<M, A>
where
    M: Unplug + Plug<A>,
{
    pub unwrap: <M as Plug<A>>::result_t,
}

impl<M, A> Concrete<M, A>
where
    M: Unplug + Plug<A>,
{
    pub fn of<MA>(x: MA) -> Concrete<M, A>
    where
        MA: Unplug<F = M, A = A> + Plug<A>,
        M: Plug<A, result_t = MA>,
    {
        Concrete { unwrap: x }
    }
}

impl<M, A> Clone for Concrete<M, A>
where
    M: Unplug + Plug<A>,
    <M as Plug<A>>::result_t: Clone + Unplug<F = M, A = A>,
{
    fn clone(&self) -> Concrete<M, A> {
        Concrete::of(self.unwrap.clone())
    }
}

pub trait Unplug: Sized {
    type F: Unplug + Plug<Self::A>;
    type A;
}

pub trait Plug<A>: Sized {
    type result_t: Plug<A> + Unplug;
}

impl<M, A, B> Plug<B> for Concrete<M, A>
where
    M: Plug<A> + Plug<B> + Unplug,
{
    type result_t = Concrete<M, B>;
}

impl<M, A> Unplug for Concrete<M, A>
where
    M: Plug<A> + Unplug,
{
    type F = M;
    type A = A;
}

macro_rules! plug {
    ($t1:ty [ $t2:ty ]) => {
        <$t1 as Plug<$t2>>::result_t
    };
}

macro_rules! unplug {
    ($t:ty, $v:ident) => {
        <$t as Unplug>::$v
    };
}

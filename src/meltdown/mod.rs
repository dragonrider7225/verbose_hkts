use applicative::Applicative;
use core::Concrete;
use functor::Functor;

#[test]
fn main() {
    let xs = Concrete::of(vec![1, 2, 3i32]);
    // Concrete::of helps the compiler infer the types through constraint manipulation;
    // simply using the naked constructor might fail to resolve the types
    // let fv = Concrete{unwrap:vec![1,2,3i32]};
    let xs = Functor::map(|x: i32| x as i64 + 1, xs);
    let fs = Concrete::of(vec![|x: i64| x + 1, |x: i64| -x]);
    let xs = Applicative::app(fs, xs);
    println!("{:?}", xs.unwrap);
}

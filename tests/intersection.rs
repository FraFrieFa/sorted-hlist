pub trait TypeEq<T> {}
impl<T> TypeEq<T> for T {}

const fn type_eq<A, B>()
where
    A: TypeEq<B>,
{
}

use sorted_hlist::{mk_hlist, Intersect};
use typenum::{U1, U2, U3, U4};

#[test]
fn intersection_two_lists() {
    type A = mk_hlist!(U1, U2, U3);
    type B = mk_hlist!(U2, U3, U4);
    type Expected = mk_hlist!(U2, U3);
    type Computed = <A as Intersect<B>>::Output;
    type_eq::<Computed, Expected>();
}

#[test]
fn intersection_empty_and_nonempty() {
    type A = mk_hlist!();
    type B = mk_hlist!(U1);
    type Expected = mk_hlist!();
    type Computed = <A as Intersect<B>>::Output;
    type_eq::<Computed, Expected>();
}

#[test]
fn intersection_disjoint() {
    type A = mk_hlist!(U1, U2);
    type B = mk_hlist!(U3, U4);
    type Expected = mk_hlist!();
    type Computed = <A as Intersect<B>>::Output;
    type_eq::<Computed, Expected>();
}

#[test]
fn intersection_identical_lists() {
    type A = mk_hlist!(U1, U2, U3);
    type B = mk_hlist!(U1, U2, U3);
    type Expected = mk_hlist!(U1, U2, U3);
    type Computed = <A as Intersect<B>>::Output;
    type_eq::<Computed, Expected>();
}

#[test]
fn intersection_subset() {
    type A = mk_hlist!(U2, U3);
    type B = mk_hlist!(U1, U2, U3, U4);
    type Expected = mk_hlist!(U2, U3);
    type Computed = <A as Intersect<B>>::Output;
    type_eq::<Computed, Expected>();
}

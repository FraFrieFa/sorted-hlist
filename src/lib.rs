#![no_std]

use typenum::{Cmp, Equal, Greater, Less};

use core::marker::PhantomData;

pub struct HNil;

pub struct HCons<H, T>(PhantomData<(H, T)>);

pub trait HList {}

impl HList for HNil {}
impl<H, T: HList> HList for HCons<H, T> {}

#[macro_export]
macro_rules! mk_hlist {
	() => {
		$crate::HNil
	};
    ($head:ty) => {
        $crate::HCons<$head, $crate::HNil>
    };
    ($head:ty, $($tail:ty),+) => {
        $crate::HCons<$head, $crate::mk_hlist!($($tail),+)>
    };
}

pub trait SortedHList: HList {}

impl SortedHList for HNil {}
impl<H> SortedHList for HCons<H, HNil> {}

pub trait LeOrEq {}
impl LeOrEq for Equal {}
impl LeOrEq for Less {}

impl<H, HT, TT> SortedHList for HCons<H, HCons<HT, TT>>
where
    HCons<HT, TT>: SortedHList,
    H: Cmp<HT>,
    <H as Cmp<HT>>::Output: LeOrEq,
{
}

pub trait NonEmptyHList: HList {}

impl<H, T: HList> NonEmptyHList for HCons<H, T> {}

pub trait Intersect<Other: SortedHList>: SortedHList {
    type Output: SortedHList;
}

impl<H, T> Intersect<HNil> for HCons<H, T>
where
    HCons<H, T>: SortedHList,
{
    type Output = HNil;
}

impl<List: SortedHList> Intersect<List> for HNil {
    type Output = HNil;
}

pub trait IntersectByOrder<Rhs: SortedHList, Ord>: SortedHList {
    type Output: SortedHList;
}

impl<HA, TA, HB, TB> IntersectByOrder<HCons<HB, TB>, Less> for HCons<HA, TA>
where
    Self: SortedHList,
    HCons<HB, TB>: SortedHList,
    TA: Intersect<HCons<HB, TB>>,
{
    type Output = <TA as Intersect<HCons<HB, TB>>>::Output;
}

impl<HA, TA, HB, TB: SortedHList> IntersectByOrder<HCons<HB, TB>, Greater> for HCons<HA, TA>
where
    HCons<HA, TA>: SortedHList,
    HCons<HB, TB>: SortedHList,
    Self: Intersect<TB>,
{
    type Output = <Self as Intersect<TB>>::Output;
}

impl<HA, TA, HB, TB: SortedHList> IntersectByOrder<HCons<HB, TB>, Equal> for HCons<HA, TA>
where
    Self: SortedHList,
    HCons<HB, TB>: SortedHList,
    TA: Intersect<TB>,
    HCons<HA, <TA as Intersect<TB>>::Output>: SortedHList,
{
    type Output = HCons<HA, <TA as Intersect<TB>>::Output>;
}

impl<HA, TA, HB, TB, Ordering> Intersect<HCons<HB, TB>> for HCons<HA, TA>
where
    HA: Cmp<HB, Output = Ordering>,
    HCons<HA, TA>: IntersectByOrder<HCons<HB, TB>, Ordering>,
    HCons<HB, TB>: SortedHList,
{
    type Output = <Self as IntersectByOrder<HCons<HB, TB>, Ordering>>::Output;
}

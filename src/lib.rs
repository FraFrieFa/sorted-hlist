#![no_std]

//! A minimal, no-std library for type-level heterogeneous lists (HLists) and
//! their compile-time intersection.
//!
//! You can build an HList via the [`mk_hlist!`] macro, mark lists as
//! [`SortedHList`] when their element types are in non-decreasing order (via
//! `typenum::Cmp`), and compute the intersection of two sorted lists using
//! the [`Intersect`] trait (which under the hood uses
//! [`IntersectUnchecked`]).

use core::marker::PhantomData;
use typenum::{Cmp, Equal, Greater, Less};

/// The empty type-level list.
pub struct HNil;

/// A non-empty type-level list, with head of type `H` and tail `T`.
///
/// # Type Parameters
/// - `H`: the type of the first element.
/// - `T`: the rest of the list (must itself be an `HList`).
pub struct HCons<H, T>(PhantomData<(H, T)>);

/// Marker trait for all HLists.
pub trait HList {}

impl HList for HNil {}
impl<H, T: HList> HList for HCons<H, T> {}

/// Build a type-level `HList` from a comma-separated list of types.
///
/// # Examples
///
/// ```rust
/// # use sorted_hlist::mk_hlist;
/// type L = mk_hlist!(u8, bool, char);
/// // Equivalent to HCons<u8, HCons<bool, HCons<char, HNil>>>
/// ```
#[macro_export]
macro_rules! mk_hlist {
    () => { $crate::HNil };
    ($head:ty) => { $crate::HCons<$head, $crate::HNil> };
    ($head:ty, $($tail:ty),+) => {
        $crate::HCons<$head, $crate::mk_hlist!($($tail),+)>
    };
}

/// Marker trait for lists whose element types are in non-decreasing order.
///
/// A `SortedHList` must satisfy at compile time that each head `H` compares
/// leq the next element `HT` via `typenum::Cmp<H, HT>`.
pub trait SortedHList: HList {}

impl SortedHList for HNil {}
impl<H> SortedHList for HCons<H, HNil> {}
impl<H, HT, TT> SortedHList for HCons<H, HCons<HT, TT>>
where
    // tail is already sorted...
    HCons<HT, TT>: SortedHList,
    // and head leq next element
    H: Cmp<HT>,
    <H as Cmp<HT>>::Output: LeOrEq,
{
}

/// Internal helper trait indicating a type-level "leq" relationship for `Cmp`.
pub trait LeOrEq {}
impl LeOrEq for Equal {}
impl LeOrEq for Less {}

/// Marker trait for non-empty HLists (i.e. `HCons<_, _>`).
pub trait NonEmptyHList: HList {}

impl<H, T: HList> NonEmptyHList for HCons<H, T> {}

/// Compute the intersection of two arbitrary HLists, with no sortedness
/// requirements.  Yields an `HList` of the common elements (in the order of
/// the left list).
///
/// This trait does *not* check that its inputs are sorted; it simply runs the
/// single-pass intersect algorithm on any `HList`.
pub trait IntersectUnchecked<Other: HList>: HList {
    /// The resulting list of elements present in both `Self` and `Other`.
    type Output: HList;
}

impl<H, T: HList> IntersectUnchecked<HNil> for HCons<H, T> {
    type Output = HNil;
}

impl<List: HList> IntersectUnchecked<List> for HNil {
    type Output = HNil;
}

/// Internal dispatch by comparing the heads of two lists.  Chooses one of three
/// branches (Less, Equal, Greater) and recurses accordingly.
pub trait IntersectByOrder<Rhs: HList, Ord>: HList {
    /// The resulting intersected list after ordering dispatch.
    type Output: HList;
}

impl<HA, TA: HList, HB, TB: HList> IntersectByOrder<HCons<HB, TB>, Less> for HCons<HA, TA>
where
    // HA < HB -> drop HA, keep intersecting TA and RHS
    TA: IntersectUnchecked<HCons<HB, TB>>,
{
    type Output = <TA as IntersectUnchecked<HCons<HB, TB>>>::Output;
}

impl<HA, TA: HList, HB, TB: HList> IntersectByOrder<HCons<HB, TB>, Greater> for HCons<HA, TA>
where
    // HA > HB -> drop HB, intersect (HA::TA) and TB
    HCons<HA, TA>: IntersectUnchecked<TB>,
{
    type Output = <HCons<HA, TA> as IntersectUnchecked<TB>>::Output;
}

impl<HA, TA: HList, HB, TB: HList> IntersectByOrder<HCons<HB, TB>, Equal> for HCons<HA, TA>
where
    // HA == HB -> keep HA, then intersect TA and TB
    TA: IntersectUnchecked<TB>,
{
    type Output = HCons<HA, <TA as IntersectUnchecked<TB>>::Output>;
}

impl<HA, TA: HList, HB, TB: HList, Ordering> IntersectUnchecked<HCons<HB, TB>> for HCons<HA, TA>
where
    // Compare the two heads at compile time, then dispatch
    HA: Cmp<HB, Output = Ordering>,
    HCons<HA, TA>: IntersectByOrder<HCons<HB, TB>, Ordering>,
{
    type Output = <Self as IntersectByOrder<HCons<HB, TB>, Ordering>>::Output;
}

// TODO: In an ideal world, `Intersect` would itself be constrained on
// `SortedHList` and guarantee a `SortedHList` output.  However, binding
// `Self: SortedHList` directly to the trait causes certain two-element
// intersections (e.g. `(U2, U3)` and `(U2, U3)`) to overflow the compiler's
// recursion limit while others (e.g. `(U1, U2, U3)` and `(U2, U3, U4)`) work.
// Until a robust solution is found, we provide a checked impl only for
// sorted lists via `Intersect` below.

/// **Checked** intersection of two *sorted* HLists.
///
/// This trait *assumes* `Self` and `Other` are `SortedHList`s, and yields
/// an `HList` of their intersection.  It does *not* re-check sortedness
/// of the result (to avoid deep recursion in the compiler).
pub trait Intersect<Other: HList>: HList {
    /// Intersection of two sorted lists.  Must itself be an `HList`.
    type Output: HList;
}

impl<LA, LB> Intersect<LB> for LA
where
    // Only sorted lists may use this impl
    LA: SortedHList + IntersectUnchecked<LB>,
    LB: SortedHList,
{
    type Output = <LA as IntersectUnchecked<LB>>::Output;
}

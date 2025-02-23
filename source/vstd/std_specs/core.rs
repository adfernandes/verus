use super::super::prelude::*;

verus! {

#[verifier::external_trait_specification]
pub trait ExInteger: Copy {
    type ExternalTraitSpecificationFor: Integer;
}

#[verifier::external_trait_specification]
pub trait ExSpecOrd<Rhs> {
    type ExternalTraitSpecificationFor: SpecOrd<Rhs>;
}

#[verifier::external_trait_specification]
pub trait ExAllocator {
    type ExternalTraitSpecificationFor: core::alloc::Allocator;
}

#[verifier::external_trait_specification]
pub trait ExFreeze {
    type ExternalTraitSpecificationFor: core::marker::Freeze;
}

#[verifier::external_trait_specification]
pub trait ExDebug {
    type ExternalTraitSpecificationFor: core::fmt::Debug;
}

#[verifier::external_trait_specification]
pub trait ExFrom<T>: Sized {
    type ExternalTraitSpecificationFor: core::convert::From<T>;
}

#[verifier::external_trait_specification]
pub trait ExPartialEq<Rhs: ?Sized> {
    type ExternalTraitSpecificationFor: core::cmp::PartialEq<Rhs>;
}

#[verifier::external_trait_specification]
pub trait ExEq: PartialEq {
    type ExternalTraitSpecificationFor: core::cmp::Eq;
}

#[verifier::external_trait_specification]
pub trait ExPartialOrd<Rhs: ?Sized>: PartialEq<Rhs> {
    type ExternalTraitSpecificationFor: core::cmp::PartialOrd<Rhs>;
}

#[verifier::external_trait_specification]
pub trait ExOrd: Eq + PartialOrd {
    type ExternalTraitSpecificationFor: Ord;
}

#[verifier::external_trait_specification]
pub trait ExHash {
    type ExternalTraitSpecificationFor: core::hash::Hash;
}

#[verifier::external_trait_specification]
pub trait ExPtrPointee {
    type ExternalTraitSpecificationFor: core::ptr::Pointee;

    type Metadata:
        Copy + Send + Sync + Ord + core::hash::Hash + Unpin + core::fmt::Debug + Sized + core::marker::Freeze;
}

#[verifier::external_trait_specification]
pub trait ExIterator {
    type ExternalTraitSpecificationFor: core::iter::Iterator;
}

#[verifier::external_trait_specification]
pub trait ExIntoIterator {
    type ExternalTraitSpecificationFor: core::iter::IntoIterator;
}

#[verifier::external_trait_specification]
pub trait ExIterStep: Clone + PartialOrd + Sized {
    type ExternalTraitSpecificationFor: core::iter::Step;
}

#[verifier::external_trait_specification]
pub trait ExBorrow<Borrowed> where Borrowed: ?Sized {
    type ExternalTraitSpecificationFor: core::borrow::Borrow<Borrowed>;
}

#[verifier::external_trait_specification]
pub trait ExStructural {
    type ExternalTraitSpecificationFor: Structural;
}

pub assume_specification<T>[ core::mem::swap::<T> ](a: &mut T, b: &mut T)
    ensures
        *a == *old(b),
        *b == *old(a),
    opens_invariants none
    no_unwind
;

#[verifier::external_type_specification]
#[verifier::accept_recursive_types(V)]
#[verifier::ext_equal]
pub struct ExOption<V>(core::option::Option<V>);

#[verifier::external_type_specification]
#[verifier::accept_recursive_types(T)]
#[verifier::reject_recursive_types_in_ground_variants(E)]
pub struct ExResult<T, E>(core::result::Result<T, E>);

pub open spec fn iter_into_iter_spec<I: Iterator>(i: I) -> I {
    i
}

#[verifier::when_used_as_spec(iter_into_iter_spec)]
pub assume_specification<I: Iterator>[ I::into_iter ](i: I) -> (r: I)
    ensures
        r == i,
;

// I don't really expect this to be particularly useful;
// this is mostly here because I wanted an easy way to test
// the combination of external_type_specification & external_body
// in a cross-crate context.
#[verifier::external_type_specification]
#[verifier::external_body]
pub struct ExDuration(core::time::Duration);

#[verifier::external_type_specification]
#[verifier::accept_recursive_types(V)]
pub struct ExPhantomData<V: ?Sized>(core::marker::PhantomData<V>);

pub assume_specification[ core::intrinsics::likely ](b: bool) -> (c: bool)
    ensures
        c == b,
;

pub assume_specification[ core::intrinsics::unlikely ](b: bool) -> (c: bool)
    ensures
        c == b,
;

#[verifier::external_type_specification]
#[verifier::external_body]
#[verifier::reject_recursive_types_in_ground_variants(V)]
pub struct ExManuallyDrop<V: ?Sized>(core::mem::ManuallyDrop<V>);

} // verus!

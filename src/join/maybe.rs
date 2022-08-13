use super::{Join, LendJoin, ParJoin};

/// Returns a structure that implements `Join`/`LendJoin`/`MaybeJoin` if the
/// contained `T` does and that yields all indices, returning `None` for all
/// missing elements and `Some(T)` for found elements.
///
/// For usage see [`LendJoin::maybe()`](LendJoin::Maybe).
///
/// WARNING: Do not have a join of only `MaybeJoin`s. Otherwise the join will
/// iterate over every single index of the bitset. If you want a join with
/// all `MaybeJoin`s, add an `EntitiesRes` to the join as well to bound the
/// join to all entities that are alive.
pub struct MaybeJoin(pub J);

// SAFETY: We return a mask containing all items, but check the original mask in
// the `get` implementation.
unsafe impl LendJoin for MaybeJoin<T>
where
    T: LendJoin,
{
    type Mask = BitSetAll;
    type Type = Option<<T as LendJoin>::Type>;
    type Value = (<T as LendJoin>::Mask, <T as LendJoin>::Value);

    unsafe fn open(self) -> (Self::Mask, Self::Value) {
        // SAFETY: While we do expose the mask and the values and therefore
        // would allow swapping them, this method is `unsafe` and relies on the
        // same invariants.
        let (mask, value) = unsafe { self.0.open() };
        (BitSetAll, (mask, value))
    }

    unsafe fn get((mask, value): &mut Self::Value, id: Index) -> Self::Type {
        if mask.contains(id) {
            // SAFETY: The mask was just checked for `id`.
            Some(unsafe { <T as LendJoin>::get(value, id) })
        } else {
            None
        }
    }

    #[inline]
    fn is_unconstrained() -> bool {
        true
    }
}

// SAFETY: We return a mask containing all items, but check the original mask in
// the `get` implementation.
unsafe impl<T> Join for MaybeJoin<T>
where
    T: Join,
{
    type Mask = BitSetAll;
    type Type = Option<<T as Join>::Type>;
    type Value = (<T as Join>::Mask, <T as Join>::Value);

    unsafe fn open(self) -> (Self::Mask, Self::Value) {
        // SAFETY: While we do expose the mask and the values and therefore
        // would allow swapping them, this method is `unsafe` and relies on the
        // same invariants.
        let (mask, value) = unsafe { self.0.open() };
        (BitSetAll, (mask, value))
    }

    unsafe fn get((mask, value): &mut Self::Value, id: Index) -> Self::Type {
        if mask.contains(id) {
            // SAFETY: The mask was just checked for `id`. This has the same
            // requirements on the caller to not call with the same `id` until
            // the previous value is no longer in use.
            Some(unsafe { <T as Join>::get(value, id) })
        } else {
            None
        }
    }

    #[inline]
    fn is_unconstrained() -> bool {
        true
    }
}

// SAFETY: This is safe as long as `T` implements `ParJoin` safely. The `get`
// implementation here makes no assumptions about being called from a single
// thread.
//
// We return a mask containing all items, but check the original mask in
// the `get` implementation.
#[cfg(feature = "parallel")]
unsafe impl<T> ParJoin for MaybeJoin<T>
where
    T: ParJoin,
{
    type Mask = BitSetAll;
    type Type = Option<<T as ParJoin>::Type>;
    type Value = (<T as ParJoin>::Mask, <T as ParJoin>::Value);

    unsafe fn open(self) -> (Self::Mask, Self::Value) {
        // SAFETY: While we do expose the mask and the values and therefore
        // would allow swapping them, this method is `unsafe` and relies on the
        // same invariants.
        let (mask, value) = unsafe { self.0.open() };
        (BitSetAll, (mask, value))
    }

    unsafe fn get((mask, value): &Self::Value, id: Index) -> Self::Type {
        if mask.contains(id) {
            // SAFETY: The mask was just checked for `id`. This has the same
            // requirements on the caller to not call with the same `id` until
            // the previous value is no longer in use.
            Some(unsafe { <T as ParJoin>::get(value, id) })
        } else {
            None
        }
    }

    #[inline]
    fn is_unconstrained() -> bool {
        true
    }
}

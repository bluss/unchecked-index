
// Because pain and SliceExt
//#![no_std]

//extern crate core as std;

/// Wrapper type for unchecked indexing through the regular index syntax
///
/// Note that the indexing is checked with debug assertions, but unchecked
/// in release mode. Test your code responsibly.
pub struct UncheckedIndex<S>(S);

/// Create a new unchecked indexing wrapper.
///
/// This function is `unsafe` to call because it allows all further indexing
/// on the wrapper to omit bounds checks.
pub unsafe fn unchecked_index<T>(v: T) -> UncheckedIndex<T>
{
    UncheckedIndex(v)
}

use std::ops::{Deref, DerefMut, Index, IndexMut};

impl<T> Deref for UncheckedIndex<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> DerefMut for UncheckedIndex<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T, I> Index<I> for UncheckedIndex<T>
    where T: GetUnchecked<I>
{
    type Output = T::Output;
    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        #[cfg(debug_assertions)]
        self.assert_indexable_with(&index);
        unsafe {
            self.0.get_unchecked(index)
        }
    }
}

impl<T, I> IndexMut<I> for UncheckedIndex<T>
    where T: GetUncheckedMut<I>
{
    #[inline]
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        #[cfg(debug_assertions)]
        self.assert_indexable_with(&index);
        unsafe {
            self.0.get_unchecked_mut(index)
        }
    }
}

pub trait CheckIndex<I> {
    /// Assert (using a regular assertion) that the index is valid.
    /// Must not return if the index is invalid for indexing self.
    ///
    /// ***Panics*** if `index` is invalid.
    fn assert_indexable_with(&self, index: &I);
}

impl<'a, T: ?Sized, I> CheckIndex<I> for &'a T where T: CheckIndex<I> {
    fn assert_indexable_with(&self, index: &I) {
        (**self).assert_indexable_with(index)
    }
}

impl<'a, T: ?Sized, I> CheckIndex<I> for &'a mut T where T: CheckIndex<I> {
    fn assert_indexable_with(&self, index: &I) {
        (**self).assert_indexable_with(index)
    }
}

impl<T> CheckIndex<usize> for [T] {
    fn assert_indexable_with(&self, &index: &usize) {
        assert!(index < self.len(),
                "index {} is out of bounds in slice of len {}",
                index, self.len())
    }
}

pub trait GetUnchecked<I>: CheckIndex<I> {
    type Output;
    unsafe fn get_unchecked(&self, index: I) -> &Self::Output;
}

pub trait GetUncheckedMut<I>: GetUnchecked<I> {
    unsafe fn get_unchecked_mut(&mut self, index: I) -> &mut Self::Output;
}

impl<T> GetUnchecked<usize> for [T] {
    type Output = T;
    unsafe fn get_unchecked(&self, index: usize) -> &Self::Output {
        (*self).get_unchecked(index)
    }
}

impl<T> GetUncheckedMut<usize> for [T] {
    unsafe fn get_unchecked_mut(&mut self, index: usize) -> &mut Self::Output {
        (*self).get_unchecked_mut(index)
    }
}

impl<'a, T: ?Sized, I> GetUnchecked<I> for &'a T
    where T: GetUnchecked<I>
{
    type Output = T::Output;
    unsafe fn get_unchecked(&self, index: I) -> &Self::Output {
        (**self).get_unchecked(index)
    }
}

impl<'a, T: ?Sized, I> GetUnchecked<I> for &'a mut T
    where T: GetUnchecked<I>
{
    type Output = T::Output;
    unsafe fn get_unchecked(&self, index: I) -> &Self::Output {
        (**self).get_unchecked(index)
    }
}

impl<'a, T: ?Sized, I> GetUncheckedMut<I> for &'a mut T
    where T: GetUncheckedMut<I>
{
    unsafe fn get_unchecked_mut(&mut self, index: I) -> &mut Self::Output {
        (**self).get_unchecked_mut(index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut data = [0; 8];
        unsafe {
            let mut data = unchecked_index(&mut data);
            for i in 0..data.len() {
                data[i] = i;
            }
        }
        assert_eq!(data, [0, 1, 2, 3, 4, 5, 6, 7]);
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic]
    fn debug_oob_check_write() {
        let mut data = [0; 8];
        unsafe {
            let mut data = unchecked_index(&mut data[..7]);
            data[7] = 1;
        }
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic]
    fn debug_oob_check_read() {
        let mut data = [0; 8];
        unsafe {
            let data = unchecked_index(&mut data[..7]);
            println!("{}", data[17]);
        }
    }

    #[cfg(not(debug_assertions))]
    #[test]
    fn non_debug_oob() {
        // outside bounds of the slice but not the data -- should be ok
        let mut data = [0; 8];
        unsafe {
            let mut data = unchecked_index(&mut data[..7]);
            data[7] = 1;
        }
        assert_eq!(data, [0, 0, 0, 0, 0, 0, 0, 1]);
    }
}

use num_traits::{NumAssignRef, NumRef};

use crate::*;
use core::{
    borrow::{Borrow, BorrowMut},
    iter::Sum,
    ops::{Index, IndexMut},
};

pub trait MathTraits: Copy + PartialOrd + Default + Sum + NumRef + NumAssignRef {}
impl<T> MathTraits for T where T: Copy + PartialOrd + Default + Sum + NumRef + NumAssignRef {}

/// A trait used when returning arrays from the two pixel traits due to the lack of the const
/// generic expression feature on stable rust.
///
/// A blanket implementation is provided but only for item types which implement
/// [`PixelComponent`].
pub trait ArrayLike<T>:
    Copy
    + AsRef<[T]>
    + AsMut<[T]>
    + Index<usize, Output = T>
    + IndexMut<usize>
    + Borrow<[T]>
    + BorrowMut<[T]>
    + IntoIterator<Item = T>
{
}
impl<T, const N: usize> ArrayLike<T> for [T; N] where T: PixelComponent {}

pub trait HomPixelSuper<T>:
    HetPixel<ColorComponent = T, AlphaComponent = T> + IntoIterator<Item = T>
{
}
impl<M, T> HomPixelSuper<T> for M where
    M: HetPixel<ColorComponent = T, AlphaComponent = T> + IntoIterator<Item = T>
{
}

pub trait HetPixelSuper<T, A>: Copy {}
impl<M, T, A> HetPixelSuper<T, A> for M where M: Copy + Default {}

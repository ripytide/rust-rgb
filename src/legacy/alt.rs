use crate::legacy::internal::pixel::*;
use core::slice;

pub use crate::formats::gray::Gray_v08 as Gray;
pub use crate::formats::gray_alpha::GrayAlpha_v08 as GrayAlpha;

/// Renamed to `Bgra`
#[doc(hidden)]
pub use crate::formats::bgra::Bgra as BGRA;

/// Renamed to `Bgr`
#[doc(hidden)]
pub use crate::formats::bgr::Bgr as BGR;

/// Renamed to `Abgr`
#[cfg(feature = "argb")]
#[doc(hidden)]
pub use crate::formats::abgr::Abgr as ABGR;

/// Renamed to `Argb`
#[cfg(feature = "argb")]
#[doc(hidden)]
pub use crate::formats::argb::Argb as ARGB;

/// Renamed to `Grb`
#[cfg(feature = "grb")]
#[doc(hidden)]
pub use crate::formats::grb::Grb as GRB;

/// 8-bit BGR
pub type BGR8 = crate::formats::bgr::Bgr<u8>;

/// 16-bit BGR in machine's native endian
pub type BGR16 = crate::formats::bgr::Bgr<u16>;

/// 8-bit BGRA
pub type BGRA8 = crate::formats::bgra::Bgra<u8>;

/// 8-bit ABGR, alpha is first. 0 = transparent, 255 = opaque.
#[cfg(feature = "argb")]
pub type ABGR8 = crate::formats::abgr::Abgr<u8>;

/// 8-bit ARGB, alpha is first. 0 = transparent, 255 = opaque.
#[cfg(feature = "argb")]
pub type ARGB8 = crate::Argb<u8>;

/// 16-bit BGR in machine's native endian
pub type BGRA16 = crate::formats::bgra::Bgra<u16>;

/// 16-bit ABGR in machine's native endian. 0 = transparent, 65535 = opaque.
#[cfg(feature = "argb")]
pub type ABGR16 = crate::formats::abgr::Abgr<u16>;

/// 16-bit ARGB in machine's native endian. 0 = transparent, 65535 = opaque.
#[cfg(feature = "argb")]
pub type ARGB16 = crate::Argb<u16>;

/// 8-bit GRB
#[cfg(feature = "grb")]
pub type GRB8 = crate::formats::grb::Grb<u8>;

/// 8-bit gray
pub type GRAY8 = Gray<u8>;

/// 16-bit gray in machine's native endian
pub type GRAY16 = Gray<u16>;

/// 8-bit gray with alpha in machine's native endian
pub type GRAYA8 = GrayAlpha<u8>;

/// 16-bit gray with alpha in machine's native endian
pub type GRAYA16 = GrayAlpha<u16>;

impl<T> Gray<T> {
    /// New grayscale pixel
    #[inline(always)]
    pub const fn new(brightness: T) -> Self {
        Self(brightness)
    }
}

impl<T: Copy> From<T> for Gray<T> {
    #[inline(always)]
    fn from(component: T) -> Self {
        Gray(component)
    }
}

impl<T: Clone, A> GrayAlpha<T, A> {
    /// Copy `Gray` component out of the `GrayAlpha` struct
    #[inline(always)]
    pub fn gray(&self) -> Gray<T> {
        Gray(self.0.clone())
    }
}

impl<T, A> GrayAlpha<T, A> {
    /// New grayscale+alpha pixel
    #[inline(always)]
    pub const fn new(brightness: T, alpha: A) -> Self {
        Self(brightness, alpha)
    }

    /// Provide a mutable view of only `Gray` component (leaving out alpha).
    #[inline(always)]
    pub fn gray_mut(&mut self) -> &mut Gray<T> {
        unsafe { &mut *(self as *mut _ as *mut _) }
    }
}

impl<T: Copy, A: Clone> GrayAlpha<T, A> {
    /// Create a new `GrayAlpha` with the new alpha value, but same gray value
    #[doc(hidden)]
    #[deprecated(note = "use .with_alpha(a) instead")]
    pub fn alpha(&self, a: A) -> Self {
        self.with_alpha(a)
    }

    /// Create a new `GrayAlpha` with the new alpha value, but same gray value
    #[inline(always)]
    pub fn with_alpha(&self, a: A) -> Self {
        Self(self.0, a)
    }

    /// Create a new `GrayAlpha` with a new alpha value created by the callback.
    #[inline(always)]
    pub fn map_alpha<F, B>(&self, f: F) -> GrayAlpha<T, B>
        where F: FnOnce(A) -> B
    {
        GrayAlpha(self.0, f(self.1.clone()))
    }

    /// Create new `GrayAlpha` with the same alpha value, but different `Gray` value
    #[inline(always)]
    pub fn map_gray<F, U, B>(&self, f: F) -> GrayAlpha<U, B>
        where F: FnOnce(T) -> U, U: Clone, B: From<A> + Clone {
        GrayAlpha(f(self.0), self.1.clone().into())
    }
}

impl<T: Copy, B> ColorComponentMap<Gray<B>, T, B> for Gray<T> {
    #[inline(always)]
    fn map_c<F>(&self, mut f: F) -> Gray<B> where F: FnMut(T) -> B {
        Gray(f(self.0))
    }
}

impl<T: Copy, A: Copy, B> ColorComponentMap<GrayAlpha<B, A>, T, B> for GrayAlpha<T, A> {
    #[inline(always)]
    fn map_c<F>(&self, mut f: F) -> GrayAlpha<B, A>
    where F: FnMut(T) -> B {
        GrayAlpha(f(self.0), self.1)
    }
}

impl<T> ComponentSlice<T> for GrayAlpha<T> {
    #[inline(always)]
    fn as_slice(&self) -> &[T] {
        unsafe {
            slice::from_raw_parts(self as *const Self as *const T, 2)
        }
    }

    #[inline(always)]
    fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe {
            slice::from_raw_parts_mut(self as *mut Self as *mut T, 2)
        }
    }
}

impl<T> ComponentSlice<T> for [GrayAlpha<T>] {
    #[inline]
    fn as_slice(&self) -> &[T] {
        unsafe {
            slice::from_raw_parts(self.as_ptr().cast(), self.len() * 2)
        }
    }

    #[inline]
    fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe {
            slice::from_raw_parts_mut(self.as_ptr() as *mut _, self.len() * 2)
        }
    }
}

impl<T> ComponentSlice<T> for Gray<T> {
    #[inline(always)]
    fn as_slice(&self) -> &[T] {
        slice::from_ref(&self.0)
    }

    #[inline(always)]
    fn as_mut_slice(&mut self) -> &mut [T] {
        slice::from_mut(&mut self.0)
    }
}

impl<T> ComponentSlice<T> for [Gray<T>] {
    #[inline]
    fn as_slice(&self) -> &[T] {
        unsafe {
            slice::from_raw_parts(self.as_ptr().cast(), self.len())
        }
    }

    #[inline]
    fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe {
            slice::from_raw_parts_mut(self.as_ptr() as *mut _, self.len())
        }
    }
}

/// Assumes 255 is opaque
impl<T: Copy> From<Gray<T>> for GrayAlpha<T, u8> {
    #[inline(always)]
    fn from(other: Gray<T>) -> Self {
        GrayAlpha(other.0, 0xFF)
    }
}

/// Assumes 65535 is opaque
impl<T: Copy> From<Gray<T>> for GrayAlpha<T, u16> {
    #[inline(always)]
    fn from(other: Gray<T>) -> Self {
        GrayAlpha(other.0, 0xFFFF)
    }
}

#[test]
fn gray() {
    use crate::Pixel;

    let rgb: crate::RGB<_> = Gray(1).into();
    assert_eq!(rgb.r, 1);
    assert_eq!(rgb.g, 1);
    assert_eq!(rgb.b, 1);

    let rgba: crate::RGBA<_> = Gray(1u8).into();
    assert_eq!(rgba.r, 1);
    assert_eq!(rgba.g, 1);
    assert_eq!(rgba.b, 1);
    assert_eq!(rgba.a, 255);

    let g: GRAY8 = 200.into();
    let g = g.map(|c| c / 2);
    assert_eq!(110, g.v + 10);
    assert_eq!(110, 10 + Gray(100).as_ref());

    let ga: GRAYA8 = GrayAlpha(1, 2);
    assert_eq!(ga.gray(), Gray::new(1));
    let mut g2 = ga.clone();
    *g2.gray_mut() = Gray(3);
    assert_eq!(g2.map_gray(|g| g + 1), GRAYA8::new(4, 2));
    assert_eq!(g2.map(|g| g + 1), GrayAlpha(4, 3));
    assert_eq!(g2.0, 3);
    assert_eq!(g2.as_slice(), &[3, 2]);
    assert_eq!(g2.as_mut_slice(), &[3, 2]);
    assert_eq!(g2.with_alpha(13), GrayAlpha(3, 13));
    assert_eq!(g2.map_alpha(|x| x + 3), GrayAlpha(3, 5));

    assert_eq!((&[Gray(1u16), Gray(2)][..]).as_slice(), &[1, 2]);
    assert_eq!((&[GrayAlpha(1u16, 2), GrayAlpha(3, 4)][..]).as_slice(), &[1, 2, 3, 4]);

    let rgba: crate::RGBA<_> = ga.into();
    assert_eq!(rgba.r, 1);
    assert_eq!(rgba.g, 1);
    assert_eq!(rgba.b, 1);
    assert_eq!(rgba.a, 2);

    let ga: GRAYA16 = GrayAlpha(1, 2);
    let rgba: crate::RGBA<u16, u16> = ga.into();
    assert_eq!(rgba.r, 1);
    assert_eq!(rgba.g, 1);
    assert_eq!(rgba.b, 1);
    assert_eq!(rgba.a, 2);
}

//! `no_std`環境向けの浮動小数点数学関数シム
//!
//! `std`有効時は組み込みメソッドを、無効時は`libm`を使用します。

#[cfg(feature = "std")]
pub(crate) fn floor(x: f64) -> f64 {
    x.floor()
}

#[cfg(not(feature = "std"))]
pub(crate) fn floor(x: f64) -> f64 {
    libm::floor(x)
}

#[cfg(feature = "std")]
pub(crate) fn sin(x: f64) -> f64 {
    x.sin()
}

#[cfg(not(feature = "std"))]
pub(crate) fn sin(x: f64) -> f64 {
    libm::sin(x)
}

#[cfg(feature = "std")]
pub(crate) fn cos(x: f64) -> f64 {
    x.cos()
}

#[cfg(not(feature = "std"))]
pub(crate) fn cos(x: f64) -> f64 {
    libm::cos(x)
}

#[cfg(feature = "std")]
pub(crate) fn sqrt(x: f64) -> f64 {
    x.sqrt()
}

#[cfg(not(feature = "std"))]
pub(crate) fn sqrt(x: f64) -> f64 {
    libm::sqrt(x)
}

#[cfg(feature = "std")]
pub(crate) fn atan2(y: f64, x: f64) -> f64 {
    y.atan2(x)
}

#[cfg(not(feature = "std"))]
pub(crate) fn atan2(y: f64, x: f64) -> f64 {
    libm::atan2(y, x)
}

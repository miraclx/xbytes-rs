/// converts a value to a potentially lossless float
macro_rules! f {
    ($value:expr) => {{
        #[cfg(feature = "lossless")]
        let val = Float::from($value);
        #[cfg(not(feature = "lossless"))]
        let val = $value as Float;
        val
    }};
}

/// converts a value to the appropriate unsigned integer
macro_rules! i {
    ($value:expr) => {{
        #[cfg(feature = "lossless")]
        let val = if let fraction::GenericFraction::Rational(fraction::Sign::Plus, r) = $value {
            r.numer() / r.denom()
        } else {
            panic!("conversion to Int failed: expected unsigned rational float")
        };
        #[cfg(not(feature = "lossless"))]
        let val = $value as Int;
        val
    }};
}

/// checks if the potentially lossless float value is equal to zero
macro_rules! f_is_zero {
    ($value:expr) => {{
        #[cfg(feature = "lossless")]
        let res = fraction::Zero::is_zero(&$value);
        #[cfg(not(feature = "lossless"))]
        let res = $value == 0.0;
        res
    }};
}

/// checks if the potentially lossless float value is equal to one
macro_rules! f_is_one {
    ($value:expr) => {{
        #[cfg(feature = "lossless")]
        let res = fraction::One::is_one(&$value);
        #[cfg(not(feature = "lossless"))]
        let res = $value == 1.0;
        res
    }};
}

/// Allows the provision of alternate execution paths for the same logic.
///
/// ### `bits` / `nobits`
///
/// The `bits` and `nobits` variants are used to provide alternate execution paths depending on whether
/// the "bits" feature flag is set.
///
/// ### `safely` / `unsafe`
///
/// Provides an alternative "safe" implementation for an otherwise potentially panickable execution.
///
/// Set the `no-panic` feature flag to use the "safely" variant.
///
/// For example, arithmetic overflows that could simply just saturate the result to the maximum value
///
/// This takes in two branches, that should always return the same result except in the case of where
/// one panics and the other implements a fallback.
///
///  - Without "no-panic", the "unsafe" branch is executed, meaning there's a possibility of panicing
///    naturally we only expect to panic from arithmetic over/underflows
///  - With "no-panic", the "safely" branch is executed, meaning the contained code must not panic on
///    the same conditions but implements a fallback, like saturating or returning `Default` if applicable.
macro_rules! exec {
    (@ safely $expr:block) => {
        #[cfg(all(feature = "no-panic", feature = "lossless"))] {
            #[allow(unused_imports)] use fraction::{CheckedDiv, CheckedMul};
            break $expr
        }
    };
    (@ unsafe $expr:block) => {
        #[cfg(any(not(feature = "no-panic"), not(feature = "lossless")))]
        break $expr
    };
    (@ bits $expr:block) => {
        #[cfg(feature = "bits")] break $expr
    };
    (@ nobits $expr:block) => {
        #[cfg(not(feature = "bits"))] break $expr
    };
    ($($term:tt { $expr:expr }),+) => {
        loop { $( exec!(@ $term { $expr }); )+ }
    };
}

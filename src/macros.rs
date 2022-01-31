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

/// Patch for the cfg! macro with support for conditional compilation with if-else syntax
macro_rules! chk_feat {
    (@[] if cfg!( $($cfg:tt)+ ) $body:block $($rest:tt)*) => {
        #[cfg( $($cfg)+ )]
        break $body;

        chk_feat!(@[ $($cfg)+ ] $($rest)*)
    };
    (@[$($pre_cfgs:tt)+] else if cfg!( $($cfg:tt)+ ) $body:block $($rest:tt)*) => {
        #[cfg( all( not( $($pre_cfgs)+ ), $($cfg)+ ) )]
        break $body;

        chk_feat!(@[ any($($pre_cfgs)+, $($cfg)+) ] $($rest)*)
    };
    (@[$($pre_cfgs:tt)+] else $else_block:block) => {
        #[cfg( not( $($pre_cfgs)+ ) )]
        break $else_block;
    };
    (@ $($junk:tt)+) => {};
    ($($all:tt)+ ) => {
        loop { chk_feat!(@[] $($all)+); }
    };
}

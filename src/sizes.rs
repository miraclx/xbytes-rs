use super::{SizeVariant::*, Unit, UnitPrefix::*};

pub mod unprefixed {
    use super::*;
    pub const BIT: Unit = Unit(None, Bit);
    pub const BYTE: Unit = Unit(None, Byte);
}

#[rustfmt::skip]
pub mod decimal {
    use super::*;

    pub mod all {
        pub use super::bits::*;
        pub use super::bytes::*;
    }

    pub mod bits {
        use super::*;

        pub const KILO_BIT : Unit = Unit::of(Kilo , Bit);
        pub const MEGA_BIT : Unit = Unit::of(Mega , Bit);
        pub const GIGA_BIT : Unit = Unit::of(Giga , Bit);
        pub const TERA_BIT : Unit = Unit::of(Tera , Bit);
        pub const PETA_BIT : Unit = Unit::of(Peta , Bit);
        pub const EXA_BIT  : Unit = Unit::of(Exa  , Bit);
        #[cfg(feature = "u128")]
        pub const ZETTA_BIT: Unit = Unit::of(Zetta, Bit);
        #[cfg(feature = "u128")]
        pub const YOTTA_BIT: Unit = Unit::of(Yotta, Bit);
    }

    pub mod bytes {
        use super::*;

        pub const KILO_BYTE : Unit = Unit::of(Kilo , Byte);
        pub const MEGA_BYTE : Unit = Unit::of(Mega , Byte);
        pub const GIGA_BYTE : Unit = Unit::of(Giga , Byte);
        pub const TERA_BYTE : Unit = Unit::of(Tera , Byte);
        pub const PETA_BYTE : Unit = Unit::of(Peta , Byte);
        pub const EXA_BYTE  : Unit = Unit::of(Exa  , Byte);
        #[cfg(feature = "u128")]
        pub const ZETTA_BYTE: Unit = Unit::of(Zetta, Byte);
        #[cfg(feature = "u128")]
        pub const YOTTA_BYTE: Unit = Unit::of(Yotta, Byte);
    }
}

// #[rustfmt::skip]
pub mod binary {
    use super::*;

    pub mod all {
        pub use super::bits::*;
        pub use super::bytes::*;
    }

    pub mod bits {

        use super::*;
        pub const KIBI_BIT: Unit = Unit::of(Kibi, Bit);
        pub const MEBI_BIT: Unit = Unit::of(Mebi, Bit);
        pub const GIBI_BIT: Unit = Unit::of(Gibi, Bit);
        pub const TEBI_BIT: Unit = Unit::of(Tebi, Bit);
        pub const PEBI_BIT: Unit = Unit::of(Pebi, Bit);
        pub const EXBI_BIT: Unit = Unit::of(Exbi, Bit);
        #[cfg(feature = "u128")]
        pub const ZEBI_BIT: Unit = Unit::of(Zebi, Bit);
        #[cfg(feature = "u128")]
        pub const YOBI_BIT: Unit = Unit::of(Yobi, Bit);
    }
    pub mod bytes {
        use super::*;
        pub const KIBI_BYTE: Unit = Unit::of(Kibi, Byte);
        pub const MEBI_BYTE: Unit = Unit::of(Mebi, Byte);
        pub const GIBI_BYTE: Unit = Unit::of(Gibi, Byte);
        pub const TEBI_BYTE: Unit = Unit::of(Tebi, Byte);
        pub const PEBI_BYTE: Unit = Unit::of(Pebi, Byte);
        pub const EXBI_BYTE: Unit = Unit::of(Exbi, Byte);
        #[cfg(feature = "u128")]
        pub const ZEBI_BYTE: Unit = Unit::of(Zebi, Byte);
        #[cfg(feature = "u128")]
        pub const YOBI_BYTE: Unit = Unit::of(Yobi, Byte);
    }
}

#[rustfmt::skip]
pub mod bits {
    pub use super::unprefixed::BIT;
    pub use super::binary::bits::{KIBI_BIT, MEBI_BIT, GIBI_BIT, TEBI_BIT, PEBI_BIT, EXBI_BIT};
    pub use super::decimal::bits::{KILO_BIT, MEGA_BIT, GIGA_BIT, TERA_BIT, PETA_BIT, EXA_BIT};

    #[cfg(feature = "u128")]
    pub use super::binary::bits::{ZEBI_BIT, YOBI_BIT};
    #[cfg(feature = "u128")]
    pub use super::decimal::bits::{ZETTA_BIT, YOTTA_BIT};
}

#[rustfmt::skip]
pub mod bytes {
    pub use super::unprefixed::BYTE;
    pub use super::binary::bytes::{KIBI_BYTE, MEBI_BYTE, GIBI_BYTE, TEBI_BYTE, PEBI_BYTE, EXBI_BYTE};
    pub use super::decimal::bytes::{KILO_BYTE, MEGA_BYTE, GIGA_BYTE, TERA_BYTE, PETA_BYTE, EXA_BYTE};

    #[cfg(feature = "u128")]
    pub use super::binary::bytes::{ZEBI_BYTE, YOBI_BYTE};
    #[cfg(feature = "u128")]
    pub use super::decimal::bytes::{ZETTA_BYTE, YOTTA_BYTE};
}

pub mod prefixed {
    pub use super::binary::all::*;
    pub use super::decimal::all::*;
}

pub mod all {
    pub use super::bits::*;
    pub use super::bytes::*;
}

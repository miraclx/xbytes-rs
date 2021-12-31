use super::*;
pub use inner::*;

#[rustfmt::skip]
mod inner {
    use super::{SizeVariant::*, Unit, UnitPrefix::*};

    pub mod noprefix {
        use super::*;
        pub const BIT: Unit = Unit(None, Bit);
        pub const BYTE: Unit = Unit(None, Byte);
    }

    pub mod decimal {
        use super::*;
        pub const KILO_BIT : Unit = Unit::of(Kilo, Bit );
        pub const MEGA_BIT : Unit = Unit::of(Mega, Bit );
        pub const GIGA_BIT : Unit = Unit::of(Giga, Bit );
        pub const TERA_BIT : Unit = Unit::of(Tera, Bit );
        pub const PETA_BIT : Unit = Unit::of(Peta, Bit );
        pub const EXA_BIT  : Unit = Unit::of(Exa , Bit );
        pub const KILO_BYTE: Unit = Unit::of(Kilo, Byte);
        pub const MEGA_BYTE: Unit = Unit::of(Mega, Byte);
        pub const GIGA_BYTE: Unit = Unit::of(Giga, Byte);
        pub const TERA_BYTE: Unit = Unit::of(Tera, Byte);
        pub const PETA_BYTE: Unit = Unit::of(Peta, Byte);
        pub const EXA_BYTE : Unit = Unit::of(Exa , Byte);
        #[cfg(feature = "u128")] pub const ZETTA_BIT : Unit = Unit::of(Zetta, Bit );
        #[cfg(feature = "u128")] pub const YOTTA_BIT : Unit = Unit::of(Yotta, Bit );
        #[cfg(feature = "u128")] pub const ZETTA_BYTE: Unit = Unit::of(Zetta, Byte);
        #[cfg(feature = "u128")] pub const YOTTA_BYTE: Unit = Unit::of(Yotta, Byte);
    }

    pub mod binary {
        use super::*;
        pub const KIBI_BIT : Unit = Unit::of(Kibi, Bit );
        pub const MEBI_BIT : Unit = Unit::of(Mebi, Bit );
        pub const GIBI_BIT : Unit = Unit::of(Gibi, Bit );
        pub const TEBI_BIT : Unit = Unit::of(Tebi, Bit );
        pub const PEBI_BIT : Unit = Unit::of(Pebi, Bit );
        pub const EXBI_BIT : Unit = Unit::of(Exbi, Bit );
        pub const KIBI_BYTE: Unit = Unit::of(Kibi, Byte);
        pub const MEBI_BYTE: Unit = Unit::of(Mebi, Byte);
        pub const GIBI_BYTE: Unit = Unit::of(Gibi, Byte);
        pub const TEBI_BYTE: Unit = Unit::of(Tebi, Byte);
        pub const PEBI_BYTE: Unit = Unit::of(Pebi, Byte);
        pub const EXBI_BYTE: Unit = Unit::of(Exbi, Byte);
        #[cfg(feature = "u128")] pub const ZEBI_BIT : Unit = Unit::of(Zebi, Bit );
        #[cfg(feature = "u128")] pub const YOBI_BIT : Unit = Unit::of(Yobi, Bit );
        #[cfg(feature = "u128")] pub const ZEBI_BYTE: Unit = Unit::of(Zebi, Byte);
        #[cfg(feature = "u128")] pub const YOBI_BYTE: Unit = Unit::of(Yobi, Byte);
    }

    pub mod bits {
        pub use super::noprefix::BIT;
        pub use super::binary::{KIBI_BIT, MEBI_BIT, GIBI_BIT, TEBI_BIT, PEBI_BIT, EXBI_BIT};
        pub use super::decimal::{KILO_BIT, MEGA_BIT, GIGA_BIT, TERA_BIT, PETA_BIT, EXA_BIT};

        #[cfg(feature = "u128")]
        pub use super::binary::{ZEBI_BIT, YOBI_BIT};
        #[cfg(feature = "u128")]
        pub use super::decimal::{ZETTA_BIT, YOTTA_BIT};
    }

    pub mod bytes {
        pub use super::noprefix::BYTE;
        pub use super::binary::{KIBI_BYTE, MEBI_BYTE, GIBI_BYTE, TEBI_BYTE, PEBI_BYTE, EXBI_BYTE};
        pub use super::decimal::{KILO_BYTE, MEGA_BYTE, GIGA_BYTE, TERA_BYTE, PETA_BYTE, EXA_BYTE};

        #[cfg(feature = "u128")]
        pub use super::binary::{ZEBI_BYTE, YOBI_BYTE};
        #[cfg(feature = "u128")]
        pub use super::decimal::{ZETTA_BYTE, YOTTA_BYTE};
    }

    pub mod prefixed {
        pub use super::binary::*;
        pub use super::decimal::*;
    }

    pub mod all {
        pub use super::noprefix::*;
        pub use super::bits::*;
        pub use super::bytes::*;
    }

    use all::*;

    pub const NOPREFIX: [Unit; 2] = [BIT, BYTE];

    pub const DECIMAL: [Unit; {
        #[cfg(feature = "u128")] { 16 }
        #[cfg(not(feature = "u128"))] { 12 }
    }] = [
        KILO_BIT, KILO_BYTE, MEGA_BIT, MEGA_BYTE, GIGA_BIT, GIGA_BYTE,
        TERA_BIT, TERA_BYTE, PETA_BIT, PETA_BYTE, EXA_BIT, EXA_BYTE,
        #[cfg(feature = "u128")] ZETTA_BIT,
        #[cfg(feature = "u128")] ZETTA_BYTE,
        #[cfg(feature = "u128")] YOTTA_BIT,
        #[cfg(feature = "u128")] YOTTA_BYTE,
    ];

    pub const BINARY: [Unit; {
        #[cfg(feature = "u128")] { 16 }
        #[cfg(not(feature = "u128"))] { 12 }
    }] = [
        KIBI_BIT, KIBI_BYTE, MEBI_BIT, MEBI_BYTE, GIBI_BIT, GIBI_BYTE,
        TEBI_BIT, TEBI_BYTE, PEBI_BIT, PEBI_BYTE, EXBI_BIT, EXBI_BYTE,
        #[cfg(feature = "u128")] ZEBI_BIT,
        #[cfg(feature = "u128")] ZEBI_BYTE,
        #[cfg(feature = "u128")] YOBI_BIT,
        #[cfg(feature = "u128")] YOBI_BYTE,
    ];

    pub const BITS: [Unit; {
        #[cfg(feature = "u128")] { 17 }
        #[cfg(not(feature = "u128"))] { 13 }
    }] = [
        BIT, KILO_BIT, KIBI_BIT, MEGA_BIT, MEBI_BIT, GIGA_BIT, GIBI_BIT,
        TERA_BIT, TEBI_BIT, PETA_BIT, PEBI_BIT, EXA_BIT, EXBI_BIT,
        #[cfg(feature = "u128")] ZETTA_BIT,
        #[cfg(feature = "u128")] ZEBI_BIT,
        #[cfg(feature = "u128")] YOTTA_BIT,
        #[cfg(feature = "u128")] YOBI_BIT,
    ];

    pub const BYTES: [Unit; {
        #[cfg(feature = "u128")] { 17 }
        #[cfg(not(feature = "u128"))] { 13 }
    }] = [
        BYTE, KILO_BYTE, KIBI_BYTE, MEGA_BYTE, MEBI_BYTE, GIGA_BYTE, GIBI_BYTE,
        TERA_BYTE, TEBI_BYTE, PETA_BYTE, PEBI_BYTE, EXA_BYTE, EXBI_BYTE,
        #[cfg(feature = "u128")] ZETTA_BYTE,
        #[cfg(feature = "u128")] ZEBI_BYTE,
        #[cfg(feature = "u128")] YOTTA_BYTE,
        #[cfg(feature = "u128")] YOBI_BYTE,
    ];

    pub const PREFIXED: [Unit; {
        #[cfg(feature = "u128")] { 32 }
        #[cfg(not(feature = "u128"))] { 24 }
    }] = [
        KILO_BIT, KIBI_BIT, KILO_BYTE, KIBI_BYTE, MEGA_BIT, MEBI_BIT, MEGA_BYTE, MEBI_BYTE,
        GIGA_BIT, GIBI_BIT, GIGA_BYTE, GIBI_BYTE, TERA_BIT, TEBI_BIT, TERA_BYTE, TEBI_BYTE,
        PETA_BIT, PEBI_BIT, PETA_BYTE, PEBI_BYTE, EXA_BIT, EXBI_BIT, EXA_BYTE, EXBI_BYTE,
        #[cfg(feature = "u128")] ZETTA_BIT,
        #[cfg(feature = "u128")] ZEBI_BIT,
        #[cfg(feature = "u128")] ZETTA_BYTE,
        #[cfg(feature = "u128")] ZEBI_BYTE,
        #[cfg(feature = "u128")] YOTTA_BIT,
        #[cfg(feature = "u128")] YOBI_BIT,
        #[cfg(feature = "u128")] YOTTA_BYTE,
        #[cfg(feature = "u128")] YOBI_BYTE,
    ];

    pub const ALL: [Unit; {
        #[cfg(feature = "u128")] { 34 }
        #[cfg(not(feature = "u128"))] { 26 }
    }] = [
        BIT, BYTE,
        KILO_BIT, KIBI_BIT, KILO_BYTE, KIBI_BYTE, MEGA_BIT, MEBI_BIT, MEGA_BYTE, MEBI_BYTE,
        GIGA_BIT, GIBI_BIT, GIGA_BYTE, GIBI_BYTE, TERA_BIT, TEBI_BIT, TERA_BYTE, TEBI_BYTE,
        PETA_BIT, PEBI_BIT, PETA_BYTE, PEBI_BYTE, EXA_BIT, EXBI_BIT, EXA_BYTE, EXBI_BYTE,
        #[cfg(feature = "u128")] ZETTA_BIT,
        #[cfg(feature = "u128")] ZEBI_BIT,
        #[cfg(feature = "u128")] ZETTA_BYTE,
        #[cfg(feature = "u128")] ZEBI_BYTE,
        #[cfg(feature = "u128")] YOTTA_BIT,
        #[cfg(feature = "u128")] YOBI_BIT,
        #[cfg(feature = "u128")] YOTTA_BYTE,
        #[cfg(feature = "u128")] YOBI_BYTE,
    ];
}

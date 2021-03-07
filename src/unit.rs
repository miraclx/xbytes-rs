use super::{
    int,
    UnitPrefix::{self, *},
};
use std::fmt;

#[derive(Eq, Copy, Clone, Debug, PartialEq)]
pub enum SizeVariant {
    Bit,
    Byte,
}

use SizeVariant::*;

#[derive(Eq, Copy, Clone, Debug, PartialEq)]
pub struct Unit(Option<UnitPrefix>, SizeVariant);

mod decimal {
    use super::Unit;

    #[rustfmt::skip]
    pub(super) mod _exported {
        use super::super::*;
        pub const KILO_BIT  : Unit = Unit::of(Kilo , Bit );
        pub const MEGA_BIT  : Unit = Unit::of(Mega , Bit );
        pub const GIGA_BIT  : Unit = Unit::of(Giga , Bit );
        pub const TERA_BIT  : Unit = Unit::of(Tera , Bit );
        pub const PETA_BIT  : Unit = Unit::of(Peta , Bit );
        pub const EXA_BIT   : Unit = Unit::of(Exa  , Bit );
        pub const ZETTA_BIT : Unit = Unit::of(Zetta, Bit );
        pub const YOTTA_BIT : Unit = Unit::of(Yotta, Bit );
        pub const KILO_BYTE : Unit = Unit::of(Kilo , Byte);
        pub const MEGA_BYTE : Unit = Unit::of(Mega , Byte);
        pub const GIGA_BYTE : Unit = Unit::of(Giga , Byte);
        pub const TERA_BYTE : Unit = Unit::of(Tera , Byte);
        pub const PETA_BYTE : Unit = Unit::of(Peta , Byte);
        pub const EXA_BYTE  : Unit = Unit::of(Exa  , Byte);
        pub const ZETTA_BYTE: Unit = Unit::of(Zetta, Byte);
        pub const YOTTA_BYTE: Unit = Unit::of(Yotta, Byte);
    }

    pub use _exported::*;

    #[rustfmt::skip]
    pub const SIZES: [Unit; 16] = [
        KILO_BIT, MEGA_BIT, GIGA_BIT, TERA_BIT, PETA_BIT, EXA_BIT, ZETTA_BIT, YOTTA_BIT,
        KILO_BYTE, MEGA_BYTE, GIGA_BYTE, TERA_BYTE, PETA_BYTE, EXA_BYTE, ZETTA_BYTE, YOTTA_BYTE,
    ];
}

mod binary {
    use super::Unit;
    #[rustfmt::skip]
    pub(super) mod _exported {
        use super::super::*;
        pub const KIBI_BIT : Unit = Unit::of(Kibi, Bit );
        pub const MEBI_BIT : Unit = Unit::of(Mebi, Bit );
        pub const GIBI_BIT : Unit = Unit::of(Gibi, Bit );
        pub const TEBI_BIT : Unit = Unit::of(Tebi, Bit );
        pub const PEBI_BIT : Unit = Unit::of(Pebi, Bit );
        pub const EXBI_BIT : Unit = Unit::of(Exbi, Bit );
        pub const ZEBI_BIT : Unit = Unit::of(Zebi, Bit );
        pub const YOBI_BIT : Unit = Unit::of(Yobi, Bit );
        pub const KIBI_BYTE: Unit = Unit::of(Kibi, Byte);
        pub const MEBI_BYTE: Unit = Unit::of(Mebi, Byte);
        pub const GIBI_BYTE: Unit = Unit::of(Gibi, Byte);
        pub const TEBI_BYTE: Unit = Unit::of(Tebi, Byte);
        pub const PEBI_BYTE: Unit = Unit::of(Pebi, Byte);
        pub const EXBI_BYTE: Unit = Unit::of(Exbi, Byte);
        pub const ZEBI_BYTE: Unit = Unit::of(Zebi, Byte);
        pub const YOBI_BYTE: Unit = Unit::of(Yobi, Byte);
    }

    pub use _exported::*;

    #[rustfmt::skip]
    pub const SIZES: [Unit; 16] = [
        KIBI_BIT, MEBI_BIT, GIBI_BIT, TEBI_BIT, PEBI_BIT, EXBI_BIT, ZEBI_BIT, YOBI_BIT,
        KIBI_BYTE, MEBI_BYTE, GIBI_BYTE, TEBI_BYTE, PEBI_BYTE, EXBI_BYTE, ZEBI_BYTE, YOBI_BYTE,
    ];
}

pub mod sizes {
    use super::*;
    pub use super::{
        binary::{_exported::*, SIZES as BINARY},
        decimal::{_exported::*, SIZES as DECIMAL},
    };

    pub const BIT: Unit = Unit(None, Bit);
    pub const BYTE: Unit = Unit(None, Byte);

    #[rustfmt::skip]
    pub const BITS: [Unit; 17] = [
        BIT, KILO_BIT, MEGA_BIT, GIGA_BIT, TERA_BIT, PETA_BIT, EXA_BIT, ZETTA_BIT, YOTTA_BIT,
        KIBI_BIT, MEBI_BIT, GIBI_BIT, TEBI_BIT, PEBI_BIT, EXBI_BIT, ZEBI_BIT, YOBI_BIT,
    ];

    #[rustfmt::skip]
    pub const BYTES: [Unit; 17] = [
        BYTE, KILO_BYTE, MEGA_BYTE, GIGA_BYTE, TERA_BYTE, PETA_BYTE, EXA_BYTE, ZETTA_BYTE, YOTTA_BYTE,
        KIBI_BYTE, MEBI_BYTE, GIBI_BYTE, TEBI_BYTE, PEBI_BYTE, EXBI_BYTE, ZEBI_BYTE, YOBI_BYTE,
    ];

    #[rustfmt::skip]
    pub const ALL: [Unit; 34] = [
        BIT, BYTE,
        KILO_BIT, MEGA_BIT, GIGA_BIT, TERA_BIT, PETA_BIT, EXA_BIT, ZETTA_BIT, YOTTA_BIT,
        KILO_BYTE, MEGA_BYTE, GIGA_BYTE, TERA_BYTE, PETA_BYTE, EXA_BYTE, ZETTA_BYTE, YOTTA_BYTE,
        KIBI_BIT, MEBI_BIT, GIBI_BIT, TEBI_BIT, PEBI_BIT, EXBI_BIT, ZEBI_BIT, YOBI_BIT,
        KIBI_BYTE, MEBI_BYTE, GIBI_BYTE, TEBI_BYTE, PEBI_BYTE, EXBI_BYTE, ZEBI_BYTE, YOBI_BYTE,
    ];
}

impl SizeVariant {
    pub const fn is_bit(&self) -> bool {
        *self as u8 == 0
    }

    pub const fn is_byte(&self) -> bool {
        *self as u8 == 1
    }

    pub const fn symbol(&self) -> &'static str {
        match self {
            Bit => "b",
            Byte => "B",
        }
    }

    pub const fn symbol_long(&self, plural: bool) -> &'static str {
        match (self, plural) {
            (Bit, true) => "Bits",
            (Bit, false) => "Bit",
            (Byte, true) => "Bytes",
            (Byte, false) => "Byte",
        }
    }

    pub const fn effective_value(&self) -> u8 {
        match self {
            Bit => 1,
            Byte => 8,
        }
    }
}

impl fmt::Display for SizeVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let variant = if f.alternate() {
            self.symbol_long(f.sign_plus())
        } else {
            self.symbol()
        };
        f.write_str(variant)
    }
}

impl From<(UnitPrefix, SizeVariant)> for Unit {
    fn from((prefix, variant): (UnitPrefix, SizeVariant)) -> Self {
        Self::of(prefix, variant)
    }
}

impl From<SizeVariant> for Unit {
    fn from(variant: SizeVariant) -> Self {
        Self(None, variant)
    }
}

impl Unit {
    pub const MIN: Unit = Unit(None, Bit);
    pub const MAX: Unit = Unit::of(UnitPrefix::MAX, Byte);

    #[inline(always)]
    pub const fn of(prefix: UnitPrefix, size_variant: SizeVariant) -> Self {
        Self(Some(prefix), size_variant)
    }

    pub const fn is_decimal(&self) -> bool {
        if let Some(prefix) = self.0 {
            return prefix.is_decimal();
        }
        true
    }

    pub const fn is_binary(&self) -> bool {
        if let Some(prefix) = self.0 {
            return prefix.is_binary();
        }
        true
    }

    pub const fn is_bit(&self) -> bool {
        self.1.is_bit()
    }

    pub const fn is_byte(&self) -> bool {
        self.1.is_byte()
    }

    pub const fn index(&self) -> usize {
        if let Some(prefix) = self.0 {
            return prefix.index() + 1;
        }
        0
    }

    pub const fn decimal(&self) -> Self {
        Self(
            match self.0 {
                Some(prefix) => Some(prefix.decimal()),
                None => None,
            },
            self.1,
        )
    }

    pub const fn binary(&self) -> Self {
        Self(
            match self.0 {
                Some(prefix) => Some(prefix.binary()),
                None => None,
            },
            self.1,
        )
    }

    pub const fn bit(&self) -> Self {
        Self(self.0, Bit)
    }

    pub const fn byte(&self) -> Self {
        Self(self.0, Byte)
    }

    pub const fn effective_value(&self) -> int {
        (match self.0 {
            Some(prefix) => prefix.effective_value(),
            None => 1,
        } * self.1.effective_value() as int)
    }

    pub const fn symbols(&self) -> (&'static str, &'static str) {
        (
            match self.0 {
                Some(prefix) => prefix.symbol(),
                None => "",
            },
            self.1.symbol(),
        )
    }

    pub fn symbol(&self) -> String {
        let (prefix, size_variant) = self.symbols();
        format!("{}{}", prefix, size_variant)
    }

    pub const fn symbols_long(&self, plural: bool) -> (&'static str, &'static str) {
        (
            match self.0 {
                Some(prefix) => prefix.symbol_long(),
                None => "",
            },
            self.1.symbol_long(plural),
        )
    }

    pub fn symbol_long(&self, plural: bool) -> String {
        let (prefix, size_variant) = self.symbols_long(plural);
        format!("{}{}", prefix, size_variant)
    }
}

#[cfg(test)]
mod tests {
    use super::{sizes::*, *};

    #[test]
    fn size_variant() {
        let bit = Bit;
        assert!(bit.is_bit());
        assert!(!bit.is_byte());

        let byte = Byte;
        assert!(byte.is_byte());
        assert!(!byte.is_bit());
    }

    #[test]
    fn format_display_size_variant_symbol() {
        let map = [(Bit, "b"), (Byte, "B")];

        for (unit, repr) in map.iter() {
            assert_eq!(
                *repr,
                unit.symbol(),
                "expected [{:?}] to be represented as {}",
                unit,
                repr
            );
            assert_eq!(
                *repr,
                format!("{}", unit),
                "expected [{:?}] to be represented as {}",
                unit,
                repr
            );
        }
    }

    #[test]
    fn format_display_size_variant_symbol_long() {
        #[rustfmt::skip]
        let map = [
            (Bit, "Bit"), (Bit, "Bits"),
            (Byte, "Byte"), (Byte, "Bytes")
        ];

        for (index, (unit, repr)) in map.iter().enumerate() {
            assert_eq!(
                *repr,
                unit.symbol_long(index % 2 != 0),
                "expected [{:?}] to be represented in long form as [{}]",
                unit,
                repr
            );
            let value = if index % 2 == 0 {
                format!("{:#}", unit)
            } else {
                format!("{:+#}", unit)
            };
            assert_eq!(
                *repr, value,
                "expected [{:?}] to be represented in long form as [{}]",
                unit, repr
            );
        }
    }

    #[test]
    fn size_variant_effective_value() {
        let map = [(Bit, 1), (Byte, 8)];

        for (size_variant, value) in map.iter() {
            assert_eq!(
                *value,
                size_variant.effective_value(),
                "expected [{:?}] to have the value [{}]",
                size_variant,
                value
            );
        }
    }

    #[test]
    #[rustfmt::skip]
    fn unit_decimal() {
        assert_eq!(KILO_BIT , Unit::of(Kilo , Bit));
        assert_eq!(MEGA_BIT , Unit::of(Mega , Bit));
        assert_eq!(GIGA_BIT , Unit::of(Giga , Bit));
        assert_eq!(TERA_BIT , Unit::of(Tera , Bit));
        assert_eq!(PETA_BIT , Unit::of(Peta , Bit));
        assert_eq!(EXA_BIT  , Unit::of(Exa  , Bit));
        assert_eq!(ZETTA_BIT, Unit::of(Zetta, Bit));
        assert_eq!(YOTTA_BIT, Unit::of(Yotta, Bit));
        // --
        assert_eq!(KILO_BYTE , Unit::of(Kilo , Byte));
        assert_eq!(MEGA_BYTE , Unit::of(Mega , Byte));
        assert_eq!(GIGA_BYTE , Unit::of(Giga , Byte));
        assert_eq!(TERA_BYTE , Unit::of(Tera , Byte));
        assert_eq!(PETA_BYTE , Unit::of(Peta , Byte));
        assert_eq!(EXA_BYTE  , Unit::of(Exa  , Byte));
        assert_eq!(ZETTA_BYTE, Unit::of(Zetta, Byte));
        assert_eq!(YOTTA_BYTE, Unit::of(Yotta, Byte));
    }

    #[test]
    fn unit_binary() {
        assert_eq!(KIBI_BIT, Unit::of(Kibi, Bit));
        assert_eq!(MEBI_BIT, Unit::of(Mebi, Bit));
        assert_eq!(GIBI_BIT, Unit::of(Gibi, Bit));
        assert_eq!(TEBI_BIT, Unit::of(Tebi, Bit));
        assert_eq!(PEBI_BIT, Unit::of(Pebi, Bit));
        assert_eq!(EXBI_BIT, Unit::of(Exbi, Bit));
        assert_eq!(ZEBI_BIT, Unit::of(Zebi, Bit));
        assert_eq!(YOBI_BIT, Unit::of(Yobi, Bit));
        // --
        assert_eq!(KIBI_BYTE, Unit::of(Kibi, Byte));
        assert_eq!(MEBI_BYTE, Unit::of(Mebi, Byte));
        assert_eq!(GIBI_BYTE, Unit::of(Gibi, Byte));
        assert_eq!(TEBI_BYTE, Unit::of(Tebi, Byte));
        assert_eq!(PEBI_BYTE, Unit::of(Pebi, Byte));
        assert_eq!(EXBI_BYTE, Unit::of(Exbi, Byte));
        assert_eq!(ZEBI_BYTE, Unit::of(Zebi, Byte));
        assert_eq!(YOBI_BYTE, Unit::of(Yobi, Byte));
    }

    #[test]
    #[rustfmt::skip]
    fn unit_to_decimal() {
        assert_eq!(KILO_BIT , KIBI_BIT.decimal());
        assert_eq!(MEGA_BIT , MEBI_BIT.decimal());
        assert_eq!(GIGA_BIT , GIBI_BIT.decimal());
        assert_eq!(TERA_BIT , TEBI_BIT.decimal());
        assert_eq!(PETA_BIT , PEBI_BIT.decimal());
        assert_eq!(EXA_BIT  , EXBI_BIT.decimal());
        assert_eq!(ZETTA_BIT, ZEBI_BIT.decimal());
        assert_eq!(YOTTA_BIT, YOBI_BIT.decimal());
        // --
        assert_eq!(KILO_BYTE , KIBI_BYTE.decimal());
        assert_eq!(MEGA_BYTE , MEBI_BYTE.decimal());
        assert_eq!(GIGA_BYTE , GIBI_BYTE.decimal());
        assert_eq!(TERA_BYTE , TEBI_BYTE.decimal());
        assert_eq!(PETA_BYTE , PEBI_BYTE.decimal());
        assert_eq!(EXA_BYTE  , EXBI_BYTE.decimal());
        assert_eq!(ZETTA_BYTE, ZEBI_BYTE.decimal());
        assert_eq!(YOTTA_BYTE, YOBI_BYTE.decimal());
    }

    #[test]
    #[rustfmt::skip]
    fn unit_to_binary() {
        assert_eq!(KIBI_BIT, KILO_BIT .binary());
        assert_eq!(MEBI_BIT, MEGA_BIT .binary());
        assert_eq!(GIBI_BIT, GIGA_BIT .binary());
        assert_eq!(TEBI_BIT, TERA_BIT .binary());
        assert_eq!(PEBI_BIT, PETA_BIT .binary());
        assert_eq!(EXBI_BIT, EXA_BIT  .binary());
        assert_eq!(ZEBI_BIT, ZETTA_BIT.binary());
        assert_eq!(YOBI_BIT, YOTTA_BIT.binary());
        // --
        assert_eq!(KIBI_BYTE, KILO_BYTE .binary());
        assert_eq!(MEBI_BYTE, MEGA_BYTE .binary());
        assert_eq!(GIBI_BYTE, GIGA_BYTE .binary());
        assert_eq!(TEBI_BYTE, TERA_BYTE .binary());
        assert_eq!(PEBI_BYTE, PETA_BYTE .binary());
        assert_eq!(EXBI_BYTE, EXA_BYTE  .binary());
        assert_eq!(ZEBI_BYTE, ZETTA_BYTE.binary());
        assert_eq!(YOBI_BYTE, YOTTA_BYTE.binary());
    }

    #[test]
    #[rustfmt::skip]
    fn unit_to_bits() {
        assert_eq!(KILO_BIT , KILO_BIT .bit());
        assert_eq!(MEGA_BIT , MEGA_BIT .bit());
        assert_eq!(GIGA_BIT , GIGA_BIT .bit());
        assert_eq!(TERA_BIT , TERA_BIT .bit());
        assert_eq!(PETA_BIT , PETA_BIT .bit());
        assert_eq!(EXA_BIT  , EXA_BIT  .bit());
        assert_eq!(ZETTA_BIT, ZETTA_BIT.bit());
        assert_eq!(YOTTA_BIT, YOTTA_BIT.bit());
        // --
        assert_eq!(KILO_BIT , KILO_BYTE .bit());
        assert_eq!(MEGA_BIT , MEGA_BYTE .bit());
        assert_eq!(GIGA_BIT , GIGA_BYTE .bit());
        assert_eq!(TERA_BIT , TERA_BYTE .bit());
        assert_eq!(PETA_BIT , PETA_BYTE .bit());
        assert_eq!(EXA_BIT  , EXA_BYTE  .bit());
        assert_eq!(ZETTA_BIT, ZETTA_BYTE.bit());
        assert_eq!(YOTTA_BIT, YOTTA_BYTE.bit());
    }

    #[test]
    fn unit_to_bytes() {
        assert_eq!(KIBI_BYTE, KIBI_BIT.byte());
        assert_eq!(MEBI_BYTE, MEBI_BIT.byte());
        assert_eq!(GIBI_BYTE, GIBI_BIT.byte());
        assert_eq!(TEBI_BYTE, TEBI_BIT.byte());
        assert_eq!(PEBI_BYTE, PEBI_BIT.byte());
        assert_eq!(EXBI_BYTE, EXBI_BIT.byte());
        assert_eq!(ZEBI_BYTE, ZEBI_BIT.byte());
        assert_eq!(YOBI_BYTE, YOBI_BIT.byte());
        // --
        assert_eq!(KIBI_BYTE, KIBI_BYTE.byte());
        assert_eq!(MEBI_BYTE, MEBI_BYTE.byte());
        assert_eq!(GIBI_BYTE, GIBI_BYTE.byte());
        assert_eq!(TEBI_BYTE, TEBI_BYTE.byte());
        assert_eq!(PEBI_BYTE, PEBI_BYTE.byte());
        assert_eq!(EXBI_BYTE, EXBI_BYTE.byte());
        assert_eq!(ZEBI_BYTE, ZEBI_BYTE.byte());
        assert_eq!(YOBI_BYTE, YOBI_BYTE.byte());
    }

    #[test]
    fn unit_is_decimal() {
        for unit in sizes::DECIMAL.iter() {
            assert!(unit.is_decimal())
        }
        for unit in sizes::BINARY.iter() {
            assert!(!unit.is_decimal())
        }
    }

    #[test]
    fn unit_is_binary() {
        for unit in sizes::BINARY.iter() {
            assert!(unit.is_binary())
        }
        for unit in sizes::DECIMAL.iter() {
            assert!(!unit.is_binary())
        }
    }

    #[test]
    #[rustfmt::skip]
    fn unit_from_prefix_and_variant() {
        assert_eq!(GIGA_BIT , Unit::from((Giga, Bit)) ); // Gb
        assert_eq!(KILO_BYTE, Unit::from((Kilo, Byte))); // KB
        assert_eq!(PEBI_BIT , (Pebi, Bit).into()      ); // Pib
        assert_eq!(EXBI_BYTE, Unit::of(Exbi, Byte)    ); // EiB
    }

    #[test]
    #[rustfmt::skip]
    fn unit_from_variant() {
        assert_eq!(BIT , Unit::from(Bit)); // b
        assert_eq!(BYTE, Byte.into()    ); // B
    }

    #[test]
    fn unit_effective_value() {
        #[rustfmt::skip]
        let map = [
            (KILO_BIT  , 1000),                      (KIBI_BIT , 1024),
            (MEGA_BIT  , 1000000),                   (MEBI_BIT , 1048576),
            (GIGA_BIT  , 1000000000),                (GIBI_BIT , 1073741824),
            (TERA_BIT  , 1000000000000),             (TEBI_BIT , 1099511627776),
            (PETA_BIT  , 1000000000000000),          (PEBI_BIT , 1125899906842624),
            (EXA_BIT   , 1000000000000000000),       (EXBI_BIT , 1152921504606846976),
            (ZETTA_BIT , 1000000000000000000000),    (ZEBI_BIT , 1180591620717411303424),
            (YOTTA_BIT , 1000000000000000000000000), (YOBI_BIT , 1208925819614629174706176),
            (KILO_BYTE , 8000),                      (KIBI_BYTE, 8192),
            (MEGA_BYTE , 8000000),                   (MEBI_BYTE, 8388608),
            (GIGA_BYTE , 8000000000),                (GIBI_BYTE, 8589934592),
            (TERA_BYTE , 8000000000000),             (TEBI_BYTE, 8796093022208),
            (PETA_BYTE , 8000000000000000),          (PEBI_BYTE, 9007199254740992),
            (EXA_BYTE  , 8000000000000000000),       (EXBI_BYTE, 9223372036854775808),
            (ZETTA_BYTE, 8000000000000000000000),    (ZEBI_BYTE, 9444732965739290427392),
            (YOTTA_BYTE, 8000000000000000000000000), (YOBI_BYTE, 9671406556917033397649408),
        ];

        for (unit, value) in map.iter() {
            assert_eq!(
                *value,
                unit.effective_value(),
                "expected [{:?}] to have the value [{}]",
                unit,
                value
            );
        }
    }

    #[test]
    fn unit_min_max() {
        assert_eq!(Unit(None, Bit), Unit::MIN);
        assert_eq!(Unit(Some(Yobi), Byte), Unit::MAX);
    }
}

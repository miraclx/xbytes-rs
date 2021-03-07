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

pub mod decimal {
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

pub mod binary {
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
    use super::*;

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
    #[rustfmt::skip]
    fn size_variant_effective_value() {
        let map = [(Bit, 1), (Byte, 8)];

        for (size_variant, value) in map.iter() {
            assert_eq!(*value, size_variant.effective_value())
        }
    }

    #[test]
    #[rustfmt::skip]
    fn unit_decimal() {
        assert_eq!(sizes::KILO_BIT , Unit::of(Kilo , Bit));
        assert_eq!(sizes::MEGA_BIT , Unit::of(Mega , Bit));
        assert_eq!(sizes::GIGA_BIT , Unit::of(Giga , Bit));
        assert_eq!(sizes::TERA_BIT , Unit::of(Tera , Bit));
        assert_eq!(sizes::PETA_BIT , Unit::of(Peta , Bit));
        assert_eq!(sizes::EXA_BIT  , Unit::of(Exa  , Bit));
        assert_eq!(sizes::ZETTA_BIT, Unit::of(Zetta, Bit));
        assert_eq!(sizes::YOTTA_BIT, Unit::of(Yotta, Bit));
        // --
        assert_eq!(sizes::KILO_BYTE , Unit::of(Kilo , Byte));
        assert_eq!(sizes::MEGA_BYTE , Unit::of(Mega , Byte));
        assert_eq!(sizes::GIGA_BYTE , Unit::of(Giga , Byte));
        assert_eq!(sizes::TERA_BYTE , Unit::of(Tera , Byte));
        assert_eq!(sizes::PETA_BYTE , Unit::of(Peta , Byte));
        assert_eq!(sizes::EXA_BYTE  , Unit::of(Exa  , Byte));
        assert_eq!(sizes::ZETTA_BYTE, Unit::of(Zetta, Byte));
        assert_eq!(sizes::YOTTA_BYTE, Unit::of(Yotta, Byte));
    }

    #[test]
    #[rustfmt::skip]
    fn unit_binary() {
        assert_eq!(sizes::KIBI_BIT, Unit::of(Kibi, Bit));
        assert_eq!(sizes::MEBI_BIT, Unit::of(Mebi, Bit));
        assert_eq!(sizes::GIBI_BIT, Unit::of(Gibi, Bit));
        assert_eq!(sizes::TEBI_BIT, Unit::of(Tebi, Bit));
        assert_eq!(sizes::PEBI_BIT, Unit::of(Pebi, Bit));
        assert_eq!(sizes::EXBI_BIT, Unit::of(Exbi, Bit));
        assert_eq!(sizes::ZEBI_BIT, Unit::of(Zebi, Bit));
        assert_eq!(sizes::YOBI_BIT, Unit::of(Yobi, Bit));
        // --
        assert_eq!(sizes::KIBI_BYTE, Unit::of(Kibi, Byte));
        assert_eq!(sizes::MEBI_BYTE, Unit::of(Mebi, Byte));
        assert_eq!(sizes::GIBI_BYTE, Unit::of(Gibi, Byte));
        assert_eq!(sizes::TEBI_BYTE, Unit::of(Tebi, Byte));
        assert_eq!(sizes::PEBI_BYTE, Unit::of(Pebi, Byte));
        assert_eq!(sizes::EXBI_BYTE, Unit::of(Exbi, Byte));
        assert_eq!(sizes::ZEBI_BYTE, Unit::of(Zebi, Byte));
        assert_eq!(sizes::YOBI_BYTE, Unit::of(Yobi, Byte));
    }

    #[test]
    #[rustfmt::skip]
    fn unit_to_decimal() {
        assert_eq!(sizes::KILO_BIT , sizes::KIBI_BIT.decimal());
        assert_eq!(sizes::MEGA_BIT , sizes::MEBI_BIT.decimal());
        assert_eq!(sizes::GIGA_BIT , sizes::GIBI_BIT.decimal());
        assert_eq!(sizes::TERA_BIT , sizes::TEBI_BIT.decimal());
        assert_eq!(sizes::PETA_BIT , sizes::PEBI_BIT.decimal());
        assert_eq!(sizes::EXA_BIT  , sizes::EXBI_BIT.decimal());
        assert_eq!(sizes::ZETTA_BIT, sizes::ZEBI_BIT.decimal());
        assert_eq!(sizes::YOTTA_BIT, sizes::YOBI_BIT.decimal());
        // --
        assert_eq!(sizes::KILO_BYTE , sizes::KIBI_BYTE.decimal());
        assert_eq!(sizes::MEGA_BYTE , sizes::MEBI_BYTE.decimal());
        assert_eq!(sizes::GIGA_BYTE , sizes::GIBI_BYTE.decimal());
        assert_eq!(sizes::TERA_BYTE , sizes::TEBI_BYTE.decimal());
        assert_eq!(sizes::PETA_BYTE , sizes::PEBI_BYTE.decimal());
        assert_eq!(sizes::EXA_BYTE  , sizes::EXBI_BYTE.decimal());
        assert_eq!(sizes::ZETTA_BYTE, sizes::ZEBI_BYTE.decimal());
        assert_eq!(sizes::YOTTA_BYTE, sizes::YOBI_BYTE.decimal());
    }

    #[test]
    #[rustfmt::skip]
    fn unit_to_binary() {
        assert_eq!(sizes::KIBI_BIT, sizes::KILO_BIT .binary());
        assert_eq!(sizes::MEBI_BIT, sizes::MEGA_BIT .binary());
        assert_eq!(sizes::GIBI_BIT, sizes::GIGA_BIT .binary());
        assert_eq!(sizes::TEBI_BIT, sizes::TERA_BIT .binary());
        assert_eq!(sizes::PEBI_BIT, sizes::PETA_BIT .binary());
        assert_eq!(sizes::EXBI_BIT, sizes::EXA_BIT  .binary());
        assert_eq!(sizes::ZEBI_BIT, sizes::ZETTA_BIT.binary());
        assert_eq!(sizes::YOBI_BIT, sizes::YOTTA_BIT.binary());
        // --
        assert_eq!(sizes::KIBI_BYTE, sizes::KILO_BYTE .binary());
        assert_eq!(sizes::MEBI_BYTE, sizes::MEGA_BYTE .binary());
        assert_eq!(sizes::GIBI_BYTE, sizes::GIGA_BYTE .binary());
        assert_eq!(sizes::TEBI_BYTE, sizes::TERA_BYTE .binary());
        assert_eq!(sizes::PEBI_BYTE, sizes::PETA_BYTE .binary());
        assert_eq!(sizes::EXBI_BYTE, sizes::EXA_BYTE  .binary());
        assert_eq!(sizes::ZEBI_BYTE, sizes::ZETTA_BYTE.binary());
        assert_eq!(sizes::YOBI_BYTE, sizes::YOTTA_BYTE.binary());
    }

    #[test]
    #[rustfmt::skip]
    fn unit_to_bits() {
        assert_eq!(sizes::KILO_BIT , sizes::KILO_BIT .bit());
        assert_eq!(sizes::MEGA_BIT , sizes::MEGA_BIT .bit());
        assert_eq!(sizes::GIGA_BIT , sizes::GIGA_BIT .bit());
        assert_eq!(sizes::TERA_BIT , sizes::TERA_BIT .bit());
        assert_eq!(sizes::PETA_BIT , sizes::PETA_BIT .bit());
        assert_eq!(sizes::EXA_BIT  , sizes::EXA_BIT  .bit());
        assert_eq!(sizes::ZETTA_BIT, sizes::ZETTA_BIT.bit());
        assert_eq!(sizes::YOTTA_BIT, sizes::YOTTA_BIT.bit());
        // --
        assert_eq!(sizes::KILO_BIT , sizes::KILO_BYTE .bit());
        assert_eq!(sizes::MEGA_BIT , sizes::MEGA_BYTE .bit());
        assert_eq!(sizes::GIGA_BIT , sizes::GIGA_BYTE .bit());
        assert_eq!(sizes::TERA_BIT , sizes::TERA_BYTE .bit());
        assert_eq!(sizes::PETA_BIT , sizes::PETA_BYTE .bit());
        assert_eq!(sizes::EXA_BIT  , sizes::EXA_BYTE  .bit());
        assert_eq!(sizes::ZETTA_BIT, sizes::ZETTA_BYTE.bit());
        assert_eq!(sizes::YOTTA_BIT, sizes::YOTTA_BYTE.bit());
    }

    #[test]
    #[rustfmt::skip]
    fn unit_to_bytes() {
        assert_eq!(sizes::KIBI_BYTE, sizes::KIBI_BIT.byte());
        assert_eq!(sizes::MEBI_BYTE, sizes::MEBI_BIT.byte());
        assert_eq!(sizes::GIBI_BYTE, sizes::GIBI_BIT.byte());
        assert_eq!(sizes::TEBI_BYTE, sizes::TEBI_BIT.byte());
        assert_eq!(sizes::PEBI_BYTE, sizes::PEBI_BIT.byte());
        assert_eq!(sizes::EXBI_BYTE, sizes::EXBI_BIT.byte());
        assert_eq!(sizes::ZEBI_BYTE, sizes::ZEBI_BIT.byte());
        assert_eq!(sizes::YOBI_BYTE, sizes::YOBI_BIT.byte());
        // --
        assert_eq!(sizes::KIBI_BYTE, sizes::KIBI_BYTE.byte());
        assert_eq!(sizes::MEBI_BYTE, sizes::MEBI_BYTE.byte());
        assert_eq!(sizes::GIBI_BYTE, sizes::GIBI_BYTE.byte());
        assert_eq!(sizes::TEBI_BYTE, sizes::TEBI_BYTE.byte());
        assert_eq!(sizes::PEBI_BYTE, sizes::PEBI_BYTE.byte());
        assert_eq!(sizes::EXBI_BYTE, sizes::EXBI_BYTE.byte());
        assert_eq!(sizes::ZEBI_BYTE, sizes::ZEBI_BYTE.byte());
        assert_eq!(sizes::YOBI_BYTE, sizes::YOBI_BYTE.byte());
    }

    #[test]
    #[rustfmt::skip]
    fn unit_from_prefix_and_variant() {
        assert_eq!(sizes::GIGA_BIT , Unit::from((Giga, Bit)) ); // Gb
        assert_eq!(sizes::KILO_BYTE, Unit::from((Kilo, Byte))); // KB
        assert_eq!(sizes::PEBI_BIT , (Pebi, Bit).into()      ); // Pib
        assert_eq!(sizes::EXBI_BYTE, Unit::of(Exbi, Byte)    ); // EiB
    }

    #[test]
    #[rustfmt::skip]
    fn unit_from_variant() {
        assert_eq!(sizes::BIT , Unit::from(Bit)); // b
        assert_eq!(sizes::BYTE, Byte.into()    ); // B
    }

    #[test]
    #[rustfmt::skip]
    fn unit_effective_value() {
        let map = [
            (sizes::KILO_BIT  , 1000),                      (sizes::KIBI_BIT , 1024),
            (sizes::MEGA_BIT  , 1000000),                   (sizes::MEBI_BIT , 1048576),
            (sizes::GIGA_BIT  , 1000000000),                (sizes::GIBI_BIT , 1073741824),
            (sizes::TERA_BIT  , 1000000000000),             (sizes::TEBI_BIT , 1099511627776),
            (sizes::PETA_BIT  , 1000000000000000),          (sizes::PEBI_BIT , 1125899906842624),
            (sizes::EXA_BIT   , 1000000000000000000),       (sizes::EXBI_BIT , 1152921504606846976),
            (sizes::ZETTA_BIT , 1000000000000000000000),    (sizes::ZEBI_BIT , 1180591620717411303424),
            (sizes::YOTTA_BIT , 1000000000000000000000000), (sizes::YOBI_BIT , 1208925819614629174706176),
            (sizes::KILO_BYTE , 8000),                      (sizes::KIBI_BYTE, 8192),
            (sizes::MEGA_BYTE , 8000000),                   (sizes::MEBI_BYTE, 8388608),
            (sizes::GIGA_BYTE , 8000000000),                (sizes::GIBI_BYTE, 8589934592),
            (sizes::TERA_BYTE , 8000000000000),             (sizes::TEBI_BYTE, 8796093022208),
            (sizes::PETA_BYTE , 8000000000000000),          (sizes::PEBI_BYTE, 9007199254740992),
            (sizes::EXA_BYTE  , 8000000000000000000),       (sizes::EXBI_BYTE, 9223372036854775808),
            (sizes::ZETTA_BYTE, 8000000000000000000000),    (sizes::ZEBI_BYTE, 9444732965739290427392),
            (sizes::YOTTA_BYTE, 8000000000000000000000000), (sizes::YOBI_BYTE, 9671406556917033397649408),
        ];

        for (size_variant, value) in map.iter() {
            assert_eq!(*value, size_variant.effective_value())
        }
    }

    #[test]
    fn unit_min_max() {
        assert_eq!(Unit(None, Bit), Unit::MIN);
        assert_eq!(Unit(Some(Yobi), Byte), Unit::MAX);
    }
}

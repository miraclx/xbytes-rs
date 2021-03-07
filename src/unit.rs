use super::UnitPrefix::{self, *};
use std::fmt;

#[derive(Eq, Copy, Clone, Debug, PartialEq)]
pub enum SizeVariant {
    Bit,
    Byte,
}

use SizeVariant::*;

#[derive(Eq, Copy, Clone, Debug, PartialEq)]
pub struct Unit(Option<UnitPrefix>, SizeVariant);

pub mod Decimal {
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

pub mod Binary {
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

pub mod Sizes {
    pub use super::{
        Binary::{_exported::*, SIZES as BINARY},
        Decimal::{_exported::*, SIZES as DECIMAL},
    };
    #[rustfmt::skip]
    pub const ALL: [super::Unit; 32] = [
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

    pub fn symbols_long(&self, plural: bool) -> (&'static str, &'static str) {
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
}

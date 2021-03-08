use super::{
    Int, ParseError,
    UnitPrefix::{self, *},
};
use std::{fmt, str::FromStr};

#[derive(Eq, Copy, Clone, Debug, PartialEq)]
pub enum SizeVariant {
    Bit,
    Byte,
}

use SizeVariant::*;

#[derive(Eq, Copy, Clone, Debug, PartialEq)]
pub struct Unit(Option<UnitPrefix>, SizeVariant);

pub mod sizes {
    use super::*;

    pub const BIT: Unit = Unit(None, Bit);
    pub const BYTE: Unit = Unit(None, Byte);

    #[rustfmt::skip]
    pub mod decimal {
        use super::*;
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
    #[rustfmt::skip]
    pub mod binary {
        use super::*;
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

    pub use {binary::*, decimal::*};

    pub const NOPREFIX: [Unit; 2] = [BIT, BYTE];

    #[rustfmt::skip]
    pub const DECIMAL: [Unit; 16] = [
        KILO_BIT, MEGA_BIT, GIGA_BIT, TERA_BIT, PETA_BIT, EXA_BIT, ZETTA_BIT, YOTTA_BIT,
        KILO_BYTE, MEGA_BYTE, GIGA_BYTE, TERA_BYTE, PETA_BYTE, EXA_BYTE, ZETTA_BYTE, YOTTA_BYTE,
    ];

    #[rustfmt::skip]
    pub const BINARY: [Unit; 16] = [
        KIBI_BIT, MEBI_BIT, GIBI_BIT, TEBI_BIT, PEBI_BIT, EXBI_BIT, ZEBI_BIT, YOBI_BIT,
        KIBI_BYTE, MEBI_BYTE, GIBI_BYTE, TEBI_BYTE, PEBI_BYTE, EXBI_BYTE, ZEBI_BYTE, YOBI_BYTE,
    ];

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

impl FromStr for SizeVariant {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Err(ParseError::EmptyInput),
            "b" => Ok(Bit),
            "B" => Ok(Byte),
            s => match s.to_lowercase().as_str() {
                "bit" | "bits" => Ok(Bit),
                "byte" | "bytes" => Ok(Byte),
                _ => Err(ParseError::SizeVariantParseError),
            },
        }
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
        false
    }

    pub const fn is_binary(&self) -> bool {
        if let Some(prefix) = self.0 {
            return prefix.is_binary();
        }
        false
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

    pub const fn effective_value(&self) -> Int {
        (match self.0 {
            Some(prefix) => prefix.effective_value(),
            None => 1,
        } * self.1.effective_value() as Int)
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

    // 'b', 'B', 'K', 'M', 'G', 'T', 'P', 'E', 'Z', 'Y'
    pub const fn symbol_condensed(&self) -> &'static str {
        match self.0 {
            Some(prefix) => prefix.symbol_initials(),
            None => self.1.symbol(),
        }
    }

    pub const fn symbols_initials(&self) -> (&'static str, &'static str) {
        (
            match self.0 {
                Some(prefix) => prefix.symbol_initials(),
                None => "",
            },
            self.1.symbol(),
        )
    }

    pub fn symbol_initials(&self) -> String {
        let (prefix, size_variant) = self.symbols_initials();
        format!("{}{}", prefix, size_variant)
    }
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let unit = if f.sign_plus() {
            self.symbol_long(f.alternate())
        } else if f.sign_minus() {
            if f.alternate() {
                self.symbol_condensed().to_string()
            } else {
                self.symbol_initials()
            }
        } else {
            self.symbol()
        };
        write!(f, "{}", unit)
    }
}

impl FromStr for Unit {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Err(ParseError::EmptyInput)
        } else {
            let index = s.rfind(|c| matches!(c, 'b' | 'B')).unwrap_or(0);
            let (prefix, size_variant) = s.split_at(index);
            let size_variant = size_variant.parse::<SizeVariant>()?;
            let prefix = (!prefix.is_empty())
                .then(|| prefix.parse::<UnitPrefix>())
                .transpose()?;
            Ok(Unit(prefix, size_variant))
        }
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
    fn size_variant_str_parse() {
        assert_eq!(Ok(Bit), "b".parse::<SizeVariant>());
        assert_eq!(Ok(Byte), "B".parse::<SizeVariant>());
        assert_eq!(Ok(Bit), "bit".parse::<SizeVariant>());
        assert_eq!(Ok(Bit), "bits".parse::<SizeVariant>());
        assert_eq!(Ok(Bit), "Bit".parse::<SizeVariant>());
        assert_eq!(Ok(Bit), "Bits".parse::<SizeVariant>());
        assert_eq!(Ok(Byte), "byte".parse::<SizeVariant>());
        assert_eq!(Ok(Byte), "bytes".parse::<SizeVariant>());
        assert_eq!(Ok(Byte), "Byte".parse::<SizeVariant>());
        assert_eq!(Ok(Byte), "Bytes".parse::<SizeVariant>());
        assert_eq!(Err(ParseError::EmptyInput), "".parse::<SizeVariant>());
        assert_eq!(
            Err(ParseError::SizeVariantParseError),
            " b ".parse::<SizeVariant>()
        );
        assert_eq!(
            Err(ParseError::SizeVariantParseError),
            "B ".parse::<SizeVariant>()
        );
        assert_eq!(
            Err(ParseError::SizeVariantParseError),
            " Bytes".parse::<SizeVariant>()
        );
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
    fn unit_from_prefix_and_variant() {
        assert_eq!(GIGA_BIT , Unit::from((Giga, Bit)) ); // Gb
        assert_eq!(KILO_BYTE, Unit::from((Kilo, Byte))); // KB
        assert_eq!(PEBI_BIT , (Pebi, Bit).into()      ); // Pib
        assert_eq!(EXBI_BYTE, Unit::of(Exbi, Byte)    ); // EiB
    }

    #[test]
    fn index() {
        #[rustfmt::skip]
        let map  = [
            (BIT,        0), (BYTE,        0),
            // --
            (KILO_BIT,   1), (KILO_BYTE,   1),
            (MEGA_BIT,   2), (MEGA_BYTE,   2),
            (GIGA_BIT,   3), (GIGA_BYTE,   3),
            (TERA_BIT,   4), (TERA_BYTE,   4),
            (PETA_BIT,   5), (PETA_BYTE,   5),
            (EXA_BIT,    6), (EXA_BYTE,    6),
            (ZETTA_BIT,  7), (ZETTA_BYTE,  7),
            (YOTTA_BIT,  8), (YOTTA_BYTE,  8),
            // --
            (KIBI_BIT,   1), (KIBI_BYTE,   1),
            (MEBI_BIT,   2), (MEBI_BYTE,   2),
            (GIBI_BIT,   3), (GIBI_BYTE,   3),
            (TEBI_BIT,   4), (TEBI_BYTE,   4),
            (PEBI_BIT,   5), (PEBI_BYTE,   5),
            (EXBI_BIT,   6), (EXBI_BYTE,   6),
            (ZEBI_BIT,   7), (ZEBI_BYTE,   7),
            (YOBI_BIT,   8), (YOBI_BYTE,   8),
        ];

        for (unit, index) in map.iter() {
            assert_eq!(
                *index,
                unit.index(),
                "expected [{:?}] to have the index {}",
                unit,
                index
            );
        }
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
        assert!(!BIT.is_decimal());
        assert!(!BYTE.is_decimal());
        for unit in sizes::DECIMAL.iter() {
            assert!(unit.is_decimal())
        }
        for unit in sizes::BINARY.iter() {
            assert!(!unit.is_decimal())
        }
    }

    #[test]
    fn unit_is_binary() {
        assert!(!BIT.is_binary());
        assert!(!BYTE.is_binary());
        for unit in sizes::BINARY.iter() {
            assert!(unit.is_binary())
        }
        for unit in sizes::DECIMAL.iter() {
            assert!(!unit.is_binary())
        }
    }

    #[test]
    fn unit_is_bit() {
        for unit in sizes::BITS.iter() {
            assert!(unit.is_bit())
        }
        for unit in sizes::BYTES.iter() {
            assert!(!unit.is_bit())
        }
    }

    #[test]
    fn unit_is_byte() {
        for unit in sizes::BYTES.iter() {
            assert!(unit.is_byte())
        }
        for unit in sizes::BITS.iter() {
            assert!(!unit.is_byte())
        }
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
            (BIT      , 1),                          (BYTE      , 8),
            (KILO_BIT , 1000),                       (KILO_BYTE , 8000),
            (MEGA_BIT , 1000000),                    (MEGA_BYTE , 8000000),
            (GIGA_BIT , 1000000000),                 (GIGA_BYTE , 8000000000),
            (TERA_BIT , 1000000000000),              (TERA_BYTE , 8000000000000),
            (PETA_BIT , 1000000000000000),           (PETA_BYTE , 8000000000000000),
            (EXA_BIT  , 1000000000000000000),        (EXA_BYTE  , 8000000000000000000),
            (ZETTA_BIT, 1000000000000000000000),     (ZETTA_BYTE, 8000000000000000000000),
            (YOTTA_BIT, 1000000000000000000000000),  (YOTTA_BYTE, 8000000000000000000000000),
            (KIBI_BIT , 1024),                       (KIBI_BYTE , 8192),
            (MEBI_BIT , 1048576),                    (MEBI_BYTE , 8388608),
            (GIBI_BIT , 1073741824),                 (GIBI_BYTE , 8589934592),
            (TEBI_BIT , 1099511627776),              (TEBI_BYTE , 8796093022208),
            (PEBI_BIT , 1125899906842624),           (PEBI_BYTE , 9007199254740992),
            (EXBI_BIT , 1152921504606846976),        (EXBI_BYTE , 9223372036854775808),
            (ZEBI_BIT , 1180591620717411303424),     (ZEBI_BYTE , 9444732965739290427392),
            (YOBI_BIT , 1208925819614629174706176),  (YOBI_BYTE , 9671406556917033397649408),
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

    #[test]
    fn unit_format_and_display_symbol() {
        #[rustfmt::skip]
        let map = [
            (BIT       , "b", "b" , "b"  , "Bit"      , "Bits"      ),
            (BYTE      , "B", "B" , "B"  , "Byte"     , "Bytes"     ),
            // --
            (KILO_BIT  , "K", "Kb", "Kb" , "KiloBit"  , "KiloBits"  ),
            (KILO_BYTE , "K", "KB", "KB" , "KiloByte" , "KiloBytes" ),
            (MEGA_BIT  , "M", "Mb", "Mb" , "MegaBit"  , "MegaBits"  ),
            (MEGA_BYTE , "M", "MB", "MB" , "MegaByte" , "MegaBytes" ),
            (GIGA_BIT  , "G", "Gb", "Gb" , "GigaBit"  , "GigaBits"  ),
            (GIGA_BYTE , "G", "GB", "GB" , "GigaByte" , "GigaBytes" ),
            (TERA_BIT  , "T", "Tb", "Tb" , "TeraBit"  , "TeraBits"  ),
            (TERA_BYTE , "T", "TB", "TB" , "TeraByte" , "TeraBytes" ),
            (PETA_BIT  , "P", "Pb", "Pb" , "PetaBit"  , "PetaBits"  ),
            (PETA_BYTE , "P", "PB", "PB" , "PetaByte" , "PetaBytes" ),
            (EXA_BIT   , "E", "Eb", "Eb" , "ExaBit"   , "ExaBits"   ),
            (EXA_BYTE  , "E", "EB", "EB" , "ExaByte"  , "ExaBytes"  ),
            (ZETTA_BIT , "Z", "Zb", "Zb" , "ZettaBit" , "ZettaBits" ),
            (ZETTA_BYTE, "Z", "ZB", "ZB" , "ZettaByte", "ZettaBytes"),
            (YOTTA_BIT , "Y", "Yb", "Yb" , "YottaBit" , "YottaBits" ),
            (YOTTA_BYTE, "Y", "YB", "YB" , "YottaByte", "YottaBytes"),
            // --
            (KIBI_BIT  , "K", "Kb", "Kib", "KibiBit"  , "KibiBits"  ),
            (KIBI_BYTE , "K", "KB", "KiB", "KibiByte" , "KibiBytes" ),
            (MEBI_BIT  , "M", "Mb", "Mib", "MebiBit"  , "MebiBits"  ),
            (MEBI_BYTE , "M", "MB", "MiB", "MebiByte" , "MebiBytes" ),
            (GIBI_BIT  , "G", "Gb", "Gib", "GibiBit"  , "GibiBits"  ),
            (GIBI_BYTE , "G", "GB", "GiB", "GibiByte" , "GibiBytes" ),
            (TEBI_BIT  , "T", "Tb", "Tib", "TebiBit"  , "TebiBits"  ),
            (TEBI_BYTE , "T", "TB", "TiB", "TebiByte" , "TebiBytes" ),
            (PEBI_BIT  , "P", "Pb", "Pib", "PebiBit"  , "PebiBits"  ),
            (PEBI_BYTE , "P", "PB", "PiB", "PebiByte" , "PebiBytes" ),
            (EXBI_BIT  , "E", "Eb", "Eib", "ExbiBit"  , "ExbiBits"  ),
            (EXBI_BYTE , "E", "EB", "EiB", "ExbiByte" , "ExbiBytes" ),
            (ZEBI_BIT  , "Z", "Zb", "Zib", "ZebiBit"  , "ZebiBits"  ),
            (ZEBI_BYTE , "Z", "ZB", "ZiB", "ZebiByte" , "ZebiBytes" ),
            (YOBI_BIT  , "Y", "Yb", "Yib", "YobiBit"  , "YobiBits"  ),
            (YOBI_BYTE , "Y", "YB", "YiB", "YobiByte" , "YobiBytes" ),
        ];

        for (unit, condensed, initials, normal, long, long_extra) in map.iter() {
            assert_eq!(
                (*condensed, *condensed),
                (unit.symbol_condensed(), format!("{:-#}", unit).as_str()),
                "expected [{:?}] to be condensed as [{}]",
                unit,
                condensed
            );
            assert_eq!(
                (initials.to_string(), initials.to_string()),
                (unit.symbol_initials(), format!("{:-}", unit)),
                "expected [{:?}] to have initials [{}]",
                unit,
                initials
            );
            assert_eq!(
                (normal.to_string(), normal.to_string()),
                (unit.symbol(), format!("{}", unit)),
                "expected [{:?}] to be represented as [{}]",
                unit,
                normal
            );
            assert_eq!(
                (long.to_string(), long.to_string()),
                (unit.symbol_long(false), format!("{:+}", unit)),
                "expected [{:?}] to be represented in long form as [{}]",
                unit,
                long
            );
            assert_eq!(
                (long_extra.to_string(), long_extra.to_string()),
                (unit.symbol_long(true), format!("{:+#}", unit)),
                "expected [{:?}] to be represented in plural long form as [{}]",
                unit,
                long_extra
            );
        }
    }

    #[test]
    fn unit_str_parse() {
        assert_eq!(Ok(BIT), "b".parse::<Unit>());
        assert_eq!(Ok(BYTE), "B".parse::<Unit>());
        assert_eq!(Ok(KILO_BIT), "kb".parse::<Unit>());
        assert_eq!(Ok(KILO_BYTE), "kB".parse::<Unit>()); // small caps 'k' is only valid as decimal
        assert_eq!(Ok(KILO_BYTE), "KB".parse::<Unit>());
        assert_eq!(Ok(KIBI_BIT), "Kib".parse::<Unit>());
        assert_eq!(Ok(KIBI_BYTE), "KiB".parse::<Unit>());
        assert_eq!(
            Err(ParseError::InvalidPrefixCaseFormat),
            "kib".parse::<Unit>()
        );
        assert_eq!(
            Err(ParseError::InvalidPrefixCaseFormat),
            "mb".parse::<Unit>() // small caps is only valid for 'k' in the decimal format
        );
        assert_eq!(Ok(MEGA_BIT), "Mb".parse::<Unit>());
        assert_eq!(Ok(MEGA_BYTE), "MB".parse::<Unit>());
        assert_eq!(Ok(MEBI_BIT), "Mib".parse::<Unit>());
        assert_eq!(Ok(MEBI_BYTE), "MiB".parse::<Unit>());
        assert_eq!(
            Err(ParseError::InvalidPrefixCaseFormat),
            "mib".parse::<Unit>()
        );
        assert_eq!(Ok(MEGA_BIT), "MegaBit".parse::<Unit>());
        assert_eq!(Ok(MEGA_BYTE), "MegaByte".parse::<Unit>());
        assert_eq!(Ok(GIGA_BIT), "gigabit".parse::<Unit>()); // it is case insensitive in the long form
        assert_eq!(Ok(GIGA_BYTE), "gigabyte".parse::<Unit>());
        assert_eq!(Err(ParseError::EmptyInput), "".parse::<Unit>());
        assert_eq!(Err(ParseError::SizeVariantParseError), "m".parse::<Unit>());
        assert_eq!(Err(ParseError::PrefixParseError), "m b".parse::<Unit>());
    }
}

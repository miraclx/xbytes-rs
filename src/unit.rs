use std::cmp::{Ord, Ordering};
use std::fmt;
use std::str::FromStr;

use super::{Int, Mode, ParseError, ParseErrorKind, UnitPrefix};

#[derive(Eq, Ord, Copy, Hash, Clone, Debug, PartialEq, PartialOrd)]
pub enum SizeVariant {
    Bit,
    Byte,
}

use SizeVariant::*;

// MAX 64-bit value => 2 EiB
// MAX 128-bit value => 35184372088832 YiB

#[derive(Eq, Copy, Hash, Clone, Debug, PartialEq)]
pub struct Unit(Option<UnitPrefix>, SizeVariant);

pub mod sizes {
    use super::{UnitPrefix::*, *};

    #[rustfmt::skip]
    pub mod noprefix {
        use super::*;
        pub const BIT: Unit = Unit(None, Bit);
        pub const BYTE: Unit = Unit(None, Byte);
    }

    #[rustfmt::skip]
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

    #[rustfmt::skip]
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

    #[rustfmt::skip]
    pub mod bits {
        pub use super::noprefix::BIT;
        pub use super::binary::{KIBI_BIT, MEBI_BIT, GIBI_BIT, TEBI_BIT, PEBI_BIT, EXBI_BIT};
        pub use super::decimal::{KILO_BIT, MEGA_BIT, GIGA_BIT, TERA_BIT, PETA_BIT, EXA_BIT};

        #[cfg(feature = "u128")]
        pub use super::binary::{ZEBI_BIT, YOBI_BIT};
        #[cfg(feature = "u128")]
        pub use super::decimal::{ZETTA_BIT, YOTTA_BIT};
    }

    #[rustfmt::skip]
    pub mod bytes {
        pub use super::noprefix::BYTE;
        pub use super::binary::{KIBI_BYTE, MEBI_BYTE, GIBI_BYTE, TEBI_BYTE, PEBI_BYTE, EXBI_BYTE};
        pub use super::decimal::{KILO_BYTE, MEGA_BYTE, GIGA_BYTE, TERA_BYTE, PETA_BYTE, EXA_BYTE};

        #[cfg(feature = "u128")]
        pub use super::binary::{ZEBI_BYTE, YOBI_BYTE};
        #[cfg(feature = "u128")]
        pub use super::decimal::{ZETTA_BYTE, YOTTA_BYTE};
    }

    #[rustfmt::skip]
    pub mod prefixed {
        pub use super::binary::*;
        pub use super::decimal::*;
    }

    #[rustfmt::skip]
    pub mod all {
        pub use super::noprefix::*;
        pub use super::bits::*;
        pub use super::bytes::*;
    }

    use all::*;

    pub const NOPREFIX: [Unit; 2] = [BIT, BYTE];

    #[rustfmt::skip]
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

    #[rustfmt::skip]
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

    #[rustfmt::skip]
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

    #[rustfmt::skip]
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

    #[rustfmt::skip]
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

    #[rustfmt::skip]
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

impl SizeVariant {
    pub const fn is_bit(&self) -> bool {
        *self as u8 == 0
    }

    pub const fn is_byte(&self) -> bool {
        *self as u8 == 1
    }

    pub const fn mode(&self) -> Mode {
        if let Bit = self {
            return Mode::Bits;
        }
        Mode::Default
    }

    pub const fn symbol(&self) -> &'static str {
        match self {
            Bit => "b",
            Byte => "B",
        }
    }

    pub const fn symbol_long(&self, plural: bool, caps: bool) -> &'static str {
        match self {
            Bit => match (plural, caps) {
                (true, true) => "Bits",
                (false, true) => "Bit",
                (true, false) => "bits",
                (false, false) => "bit",
            },
            Byte => match (plural, caps) {
                (true, true) => "Bytes",
                (false, true) => "Byte",
                (true, false) => "bytes",
                (false, false) => "byte",
            },
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
            self.symbol_long(f.sign_plus(), true)
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
            "" => Err(ParseError {
                kind: ParseErrorKind::EmptyInput,
            }),
            "b" => Ok(Bit),
            "B" => Ok(Byte),
            s => match s.to_lowercase().as_str() {
                "bit" | "bits" => Ok(Bit),
                "byte" | "bytes" => Ok(Byte),
                _ => Err(ParseError {
                    kind: ParseErrorKind::InvalidSizeVariant,
                }),
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

impl From<UnitPrefix> for Unit {
    fn from(prefix: UnitPrefix) -> Self {
        Self::of(prefix, SizeVariant::Byte)
    }
}

impl Unit {
    pub const MIN: Unit = Unit(None, Bit);
    pub const MAX: Unit = Unit::of(UnitPrefix::MAX, Byte);

    #[inline(always)]
    pub const fn prefix(&self) -> Option<UnitPrefix> {
        self.0
    }

    #[inline(always)]
    pub const fn size_variant(&self) -> SizeVariant {
        self.1
    }

    #[inline(always)]
    const fn of(prefix: UnitPrefix, size_variant: SizeVariant) -> Self {
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

    pub const fn is_prefixed(&self) -> bool {
        self.0.is_some()
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

    pub const fn mode(&self) -> Mode {
        match (self.is_decimal(), self.is_bit()) {
            (false, false) => Mode::Default,
            (true, false) => Mode::Decimal,
            (false, true) => Mode::Bits,
            (true, true) => Mode::from_bits_truncate(Mode::Decimal.bits() | Mode::Bits.bits()),
        }
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

    pub const fn symbols_long(
        &self,
        plural: bool,
        multi_caps: bool,
    ) -> (&'static str, &'static str) {
        (
            match self.0 {
                Some(prefix) => prefix.symbol_long(),
                None => "",
            },
            self.1
                .symbol_long(plural, !self.is_prefixed() || multi_caps),
        )
    }

    pub fn symbol_long(&self, plural: bool, multi_caps: bool) -> String {
        let (prefix, size_variant) = self.symbols_long(plural, multi_caps);
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

impl Ord for Unit {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.effective_value().cmp(&other.effective_value())
    }
}

impl PartialOrd for Unit {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let unit = if f.sign_plus() {
            self.symbol_long(!f.alternate(), true)
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
            Err(ParseError {
                kind: ParseErrorKind::EmptyInput,
            })
        } else {
            let index = s.rfind(|c| matches!(c, 'b' | 'B')).unwrap_or(0);
            let (prefix, size_variant) = s.split_at(index);
            let size_variant = size_variant.parse::<SizeVariant>()?;
            #[rustfmt::skip]
            let prefix = (!prefix.is_empty())
                .then(|| {
                    let prefix = prefix.parse::<UnitPrefix>();
                    #[cfg(feature = "case-insensitive")] { prefix }
                    #[cfg(not(feature = "case-insensitive"))]
                    {
                        prefix.map_err(|err| match err {
                            ParseError {
                                kind: ParseErrorKind::InvalidPrefixCaseFormat
                            } => ParseError {
                                kind: ParseErrorKind::InvalidUnitCaseFormat,
                            },
                            err => err,
                        })
                    }
                })
                .transpose()?;
            Ok(Unit(prefix, size_variant))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{sizes::all::*, UnitPrefix::*, *};

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
    fn size_variant_cmp() {
        assert!(Bit < Byte && Byte > Bit);
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
                unit.symbol_long(index % 2 != 0, true),
                "expected [{:?}] to be represented in long form as [{}]",
                unit,
                repr
            );
            assert_eq!(
                repr.to_lowercase(),
                unit.symbol_long(index % 2 != 0, false),
                "expected [{:?}] to be represented in its long single-caps form as [{}]",
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
        assert_eq!(
            Err(ParseError {
                kind: ParseErrorKind::EmptyInput
            }),
            "".parse::<SizeVariant>()
        );
        assert_eq!(
            Err(ParseError {
                kind: ParseErrorKind::InvalidSizeVariant
            }),
            " b ".parse::<SizeVariant>()
        );
        assert_eq!(
            Err(ParseError {
                kind: ParseErrorKind::InvalidSizeVariant
            }),
            "B ".parse::<SizeVariant>()
        );
        assert_eq!(
            Err(ParseError {
                kind: ParseErrorKind::InvalidSizeVariant
            }),
            " Bytes".parse::<SizeVariant>()
        );
    }

    #[test]
    #[rustfmt::skip]
    fn unit_cmp() {
        assert!(  KILO_BIT < KIBI_BIT  &&  KIBI_BIT > KILO_BIT  );
        assert!(  KIBI_BIT < KILO_BYTE && KILO_BYTE > KIBI_BIT  );
        assert!( KILO_BYTE < KIBI_BYTE && KIBI_BYTE > KILO_BYTE );
        assert!( KIBI_BYTE < MEGA_BIT  &&  MEGA_BIT > KIBI_BYTE );
        assert!(  MEGA_BIT < MEBI_BIT  &&  MEBI_BIT > MEGA_BIT  );
        assert!(  MEBI_BIT < MEGA_BYTE && MEGA_BYTE > MEBI_BIT  );
        assert!( MEGA_BYTE < MEBI_BYTE && MEBI_BYTE > MEGA_BYTE );
        assert!( MEBI_BYTE < GIGA_BIT  &&  GIGA_BIT > MEBI_BYTE );
        assert!(  GIGA_BIT < GIBI_BIT  &&  GIBI_BIT > GIGA_BIT  );
        assert!(  GIBI_BIT < GIGA_BYTE && GIGA_BYTE > GIBI_BIT  );
        assert!( GIGA_BYTE < GIBI_BYTE && GIBI_BYTE > GIGA_BYTE );
        assert!( GIBI_BYTE < TERA_BIT  &&  TERA_BIT > GIBI_BYTE );
        assert!(  TERA_BIT < TEBI_BIT  &&  TEBI_BIT > TERA_BIT  );
        assert!(  TEBI_BIT < TERA_BYTE && TERA_BYTE > TEBI_BIT  );
        assert!( TERA_BYTE < TEBI_BYTE && TEBI_BYTE > TERA_BYTE );
        assert!( TEBI_BYTE < PETA_BIT  &&  PETA_BIT > TEBI_BYTE );
        assert!(  PETA_BIT < PEBI_BIT  &&  PEBI_BIT > PETA_BIT  );
        assert!(  PEBI_BIT < PETA_BYTE && PETA_BYTE > PEBI_BIT  );
        assert!( PETA_BYTE < PEBI_BYTE && PEBI_BYTE > PETA_BYTE );
        assert!( PEBI_BYTE < EXA_BIT   &&   EXA_BIT > PEBI_BYTE );
        assert!(   EXA_BIT < EXBI_BIT  &&  EXBI_BIT > EXA_BIT   );
        assert!(  EXBI_BIT < EXA_BYTE  &&  EXA_BYTE > EXBI_BIT  );
        assert!(  EXA_BYTE < EXBI_BYTE && EXBI_BYTE > EXA_BYTE  );

        #[cfg(feature = "u128")] assert!(  EXBI_BYTE < ZETTA_BIT  &&  ZETTA_BIT > EXBI_BYTE  );
        #[cfg(feature = "u128")] assert!(  ZETTA_BIT < ZEBI_BIT   &&   ZEBI_BIT > ZETTA_BIT  );
        #[cfg(feature = "u128")] assert!(   ZEBI_BIT < ZETTA_BYTE && ZETTA_BYTE > ZEBI_BIT   );
        #[cfg(feature = "u128")] assert!( ZETTA_BYTE < ZEBI_BYTE  &&  ZEBI_BYTE > ZETTA_BYTE );
        #[cfg(feature = "u128")] assert!(  ZEBI_BYTE < YOTTA_BIT  &&  YOTTA_BIT > ZEBI_BYTE  );
        #[cfg(feature = "u128")] assert!(  YOTTA_BIT < YOBI_BIT   &&   YOBI_BIT > YOTTA_BIT  );
        #[cfg(feature = "u128")] assert!(   YOBI_BIT < YOTTA_BYTE && YOTTA_BYTE > YOBI_BIT   );
        #[cfg(feature = "u128")] assert!( YOTTA_BYTE < YOBI_BYTE  &&  YOBI_BYTE > YOTTA_BYTE );
    }

    #[test]
    fn const_sizes_sorted() {
        fn is_sorted(prefix: &mut [Unit]) -> bool {
            let a = prefix.windows(2).all(|lr| lr[0] < lr[1]);
            prefix.reverse();
            let b = prefix.windows(2).all(|lr| lr[0] > lr[1]);
            a && b
        }

        assert!(is_sorted(&mut { sizes::NOPREFIX }));
        assert!(is_sorted(&mut { sizes::DECIMAL }));
        assert!(is_sorted(&mut { sizes::BINARY }));
        assert!(is_sorted(&mut { sizes::BITS }));
        assert!(is_sorted(&mut { sizes::BYTES }));
        assert!(is_sorted(&mut { sizes::PREFIXED }));
        assert!(is_sorted(&mut { sizes::ALL }));
    }

    #[test]
    fn unit_components() {
        let b = BIT;
        assert_eq!((None, Bit), (b.prefix(), b.size_variant()));

        let kb = KILO_BIT;
        assert_eq!((Some(Kilo), Bit), (kb.prefix(), kb.size_variant()));

        #[allow(non_snake_case)]
        let GB = GIGA_BYTE;
        assert_eq!((Some(Giga), Byte), (GB.prefix(), GB.size_variant()));
    }

    #[test]
    #[rustfmt::skip]
    fn unit_decimal() {
        assert_eq!(KILO_BIT, Unit::of(Kilo, Bit));
        assert_eq!(MEGA_BIT, Unit::of(Mega, Bit));
        assert_eq!(GIGA_BIT, Unit::of(Giga, Bit));
        assert_eq!(TERA_BIT, Unit::of(Tera, Bit));
        assert_eq!(PETA_BIT, Unit::of(Peta, Bit));
        assert_eq!(EXA_BIT , Unit::of(Exa , Bit));
        #[cfg(feature = "u128")] assert_eq!(ZETTA_BIT, Unit::of(Zetta, Bit));
        #[cfg(feature = "u128")] assert_eq!(YOTTA_BIT, Unit::of(Yotta, Bit));
        // --
        assert_eq!(KILO_BYTE, Unit::of(Kilo, Byte));
        assert_eq!(MEGA_BYTE, Unit::of(Mega, Byte));
        assert_eq!(GIGA_BYTE, Unit::of(Giga, Byte));
        assert_eq!(TERA_BYTE, Unit::of(Tera, Byte));
        assert_eq!(PETA_BYTE, Unit::of(Peta, Byte));
        assert_eq!(EXA_BYTE , Unit::of(Exa , Byte));
        #[cfg(feature = "u128")] assert_eq!(ZETTA_BYTE, Unit::of(Zetta, Byte));
        #[cfg(feature = "u128")] assert_eq!(YOTTA_BYTE, Unit::of(Yotta, Byte));
    }

    #[test]
    fn unit_binary() {
        assert_eq!(KIBI_BIT, Unit::of(Kibi, Bit));
        assert_eq!(MEBI_BIT, Unit::of(Mebi, Bit));
        assert_eq!(GIBI_BIT, Unit::of(Gibi, Bit));
        assert_eq!(TEBI_BIT, Unit::of(Tebi, Bit));
        assert_eq!(PEBI_BIT, Unit::of(Pebi, Bit));
        assert_eq!(EXBI_BIT, Unit::of(Exbi, Bit));
        #[cfg(feature = "u128")]
        assert_eq!(ZEBI_BIT, Unit::of(Zebi, Bit));
        #[cfg(feature = "u128")]
        assert_eq!(YOBI_BIT, Unit::of(Yobi, Bit));
        // --
        assert_eq!(KIBI_BYTE, Unit::of(Kibi, Byte));
        assert_eq!(MEBI_BYTE, Unit::of(Mebi, Byte));
        assert_eq!(GIBI_BYTE, Unit::of(Gibi, Byte));
        assert_eq!(TEBI_BYTE, Unit::of(Tebi, Byte));
        assert_eq!(PEBI_BYTE, Unit::of(Pebi, Byte));
        assert_eq!(EXBI_BYTE, Unit::of(Exbi, Byte));
        #[cfg(feature = "u128")]
        assert_eq!(ZEBI_BYTE, Unit::of(Zebi, Byte));
        #[cfg(feature = "u128")]
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
    fn unit_from_prefix_and_default() {
        assert_eq!(KILO_BYTE, Unit::from(Kilo));
        assert_eq!(KIBI_BYTE, Unit::from(Kibi));
        assert_eq!(GIGA_BYTE, Giga.into());
        assert_eq!(GIBI_BYTE, Gibi.into());
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
            (EXA_BIT ,   6), (EXA_BYTE ,   6),
            #[cfg(feature = "u128")] (ZETTA_BIT ,  7),
            #[cfg(feature = "u128")] (ZETTA_BYTE,  7),
            #[cfg(feature = "u128")] (YOTTA_BIT ,  8),
            #[cfg(feature = "u128")] (YOTTA_BYTE,  8),
            // --
            (KIBI_BIT,   1), (KIBI_BYTE,   1),
            (MEBI_BIT,   2), (MEBI_BYTE,   2),
            (GIBI_BIT,   3), (GIBI_BYTE,   3),
            (TEBI_BIT,   4), (TEBI_BYTE,   4),
            (PEBI_BIT,   5), (PEBI_BYTE,   5),
            (EXBI_BIT,   6), (EXBI_BYTE,   6),
            #[cfg(feature = "u128")] (ZEBI_BIT ,   7),
            #[cfg(feature = "u128")] (ZEBI_BYTE,   7),
            #[cfg(feature = "u128")] (YOBI_BIT ,   8),
            #[cfg(feature = "u128")] (YOBI_BYTE,   8),
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
        assert_eq!(KILO_BIT, KIBI_BIT.decimal());
        assert_eq!(MEGA_BIT, MEBI_BIT.decimal());
        assert_eq!(GIGA_BIT, GIBI_BIT.decimal());
        assert_eq!(TERA_BIT, TEBI_BIT.decimal());
        assert_eq!(PETA_BIT, PEBI_BIT.decimal());
        assert_eq!(EXA_BIT , EXBI_BIT.decimal());
        #[cfg(feature = "u128")] assert_eq!(ZETTA_BIT, ZEBI_BIT.decimal());
        #[cfg(feature = "u128")] assert_eq!(YOTTA_BIT, YOBI_BIT.decimal());
        // --
        assert_eq!(KILO_BYTE, KIBI_BYTE.decimal());
        assert_eq!(MEGA_BYTE, MEBI_BYTE.decimal());
        assert_eq!(GIGA_BYTE, GIBI_BYTE.decimal());
        assert_eq!(TERA_BYTE, TEBI_BYTE.decimal());
        assert_eq!(PETA_BYTE, PEBI_BYTE.decimal());
        assert_eq!(EXA_BYTE , EXBI_BYTE.decimal());
        #[cfg(feature = "u128")] assert_eq!(ZETTA_BYTE, ZEBI_BYTE.decimal());
        #[cfg(feature = "u128")] assert_eq!(YOTTA_BYTE, YOBI_BYTE.decimal());
    }

    #[test]
    #[rustfmt::skip]
    fn unit_to_binary() {
        assert_eq!(KIBI_BIT, KILO_BIT.binary());
        assert_eq!(MEBI_BIT, MEGA_BIT.binary());
        assert_eq!(GIBI_BIT, GIGA_BIT.binary());
        assert_eq!(TEBI_BIT, TERA_BIT.binary());
        assert_eq!(PEBI_BIT, PETA_BIT.binary());
        assert_eq!(EXBI_BIT, EXA_BIT .binary());
        #[cfg(feature = "u128")] assert_eq!(ZEBI_BIT, ZETTA_BIT.binary());
        #[cfg(feature = "u128")] assert_eq!(YOBI_BIT, YOTTA_BIT.binary());
        // --
        assert_eq!(KIBI_BYTE, KILO_BYTE.binary());
        assert_eq!(MEBI_BYTE, MEGA_BYTE.binary());
        assert_eq!(GIBI_BYTE, GIGA_BYTE.binary());
        assert_eq!(TEBI_BYTE, TERA_BYTE.binary());
        assert_eq!(PEBI_BYTE, PETA_BYTE.binary());
        assert_eq!(EXBI_BYTE, EXA_BYTE .binary());
        #[cfg(feature = "u128")] assert_eq!(ZEBI_BYTE, ZETTA_BYTE.binary());
        #[cfg(feature = "u128")] assert_eq!(YOBI_BYTE, YOTTA_BYTE.binary());
    }

    #[test]
    #[rustfmt::skip]
    fn unit_to_bits() {
        assert_eq!(KILO_BIT, KILO_BIT.bit());
        assert_eq!(MEGA_BIT, MEGA_BIT.bit());
        assert_eq!(GIGA_BIT, GIGA_BIT.bit());
        assert_eq!(TERA_BIT, TERA_BIT.bit());
        assert_eq!(PETA_BIT, PETA_BIT.bit());
        assert_eq!(EXA_BIT , EXA_BIT .bit());
        #[cfg(feature = "u128")] assert_eq!(ZETTA_BIT, ZETTA_BIT.bit());
        #[cfg(feature = "u128")] assert_eq!(YOTTA_BIT, YOTTA_BIT.bit());
        // --
        assert_eq!(KILO_BIT, KILO_BYTE.bit());
        assert_eq!(MEGA_BIT, MEGA_BYTE.bit());
        assert_eq!(GIGA_BIT, GIGA_BYTE.bit());
        assert_eq!(TERA_BIT, TERA_BYTE.bit());
        assert_eq!(PETA_BIT, PETA_BYTE.bit());
        assert_eq!(EXA_BIT , EXA_BYTE .bit());
        #[cfg(feature = "u128")] assert_eq!(ZETTA_BIT, ZETTA_BYTE.bit());
        #[cfg(feature = "u128")] assert_eq!(YOTTA_BIT, YOTTA_BYTE.bit());
    }

    #[test]
    fn unit_to_bytes() {
        assert_eq!(KIBI_BYTE, KIBI_BIT.byte());
        assert_eq!(MEBI_BYTE, MEBI_BIT.byte());
        assert_eq!(GIBI_BYTE, GIBI_BIT.byte());
        assert_eq!(TEBI_BYTE, TEBI_BIT.byte());
        assert_eq!(PEBI_BYTE, PEBI_BIT.byte());
        assert_eq!(EXBI_BYTE, EXBI_BIT.byte());
        #[cfg(feature = "u128")]
        assert_eq!(ZEBI_BYTE, ZEBI_BIT.byte());
        #[cfg(feature = "u128")]
        assert_eq!(YOBI_BYTE, YOBI_BIT.byte());
        // --
        assert_eq!(KIBI_BYTE, KIBI_BYTE.byte());
        assert_eq!(MEBI_BYTE, MEBI_BYTE.byte());
        assert_eq!(GIBI_BYTE, GIBI_BYTE.byte());
        assert_eq!(TEBI_BYTE, TEBI_BYTE.byte());
        assert_eq!(PEBI_BYTE, PEBI_BYTE.byte());
        assert_eq!(EXBI_BYTE, EXBI_BYTE.byte());
        #[cfg(feature = "u128")]
        assert_eq!(ZEBI_BYTE, ZEBI_BYTE.byte());
        #[cfg(feature = "u128")]
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
    fn unit_is_prefixed() {
        for unit in sizes::NOPREFIX.iter() {
            assert!(!unit.is_prefixed())
        }
        for unit in sizes::PREFIXED.iter() {
            assert!(unit.is_prefixed())
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
            (BIT     , 1),                          (BYTE     , 8),
            (KILO_BIT, 1000),                       (KILO_BYTE, 8000),
            (MEGA_BIT, 1000000),                    (MEGA_BYTE, 8000000),
            (GIGA_BIT, 1000000000),                 (GIGA_BYTE, 8000000000),
            (TERA_BIT, 1000000000000),              (TERA_BYTE, 8000000000000),
            (PETA_BIT, 1000000000000000),           (PETA_BYTE, 8000000000000000),
            (EXA_BIT , 1000000000000000000),        (EXA_BYTE , 8000000000000000000),
            #[cfg(feature = "u128")] (ZETTA_BIT , 1000000000000000000000),
            #[cfg(feature = "u128")] (YOTTA_BIT , 1000000000000000000000000),
            #[cfg(feature = "u128")] (ZETTA_BYTE, 8000000000000000000000),
            #[cfg(feature = "u128")] (YOTTA_BYTE, 8000000000000000000000000),
            (KIBI_BIT, 1024),                       (KIBI_BYTE, 8192),
            (MEBI_BIT, 1048576),                    (MEBI_BYTE, 8388608),
            (GIBI_BIT, 1073741824),                 (GIBI_BYTE, 8589934592),
            (TEBI_BIT, 1099511627776),              (TEBI_BYTE, 8796093022208),
            (PEBI_BIT, 1125899906842624),           (PEBI_BYTE, 9007199254740992),
            (EXBI_BIT, 1152921504606846976),        (EXBI_BYTE, 9223372036854775808),
            #[cfg(feature = "u128")] (ZEBI_BIT , 1180591620717411303424),
            #[cfg(feature = "u128")] (YOBI_BIT , 1208925819614629174706176),
            #[cfg(feature = "u128")] (ZEBI_BYTE, 9444732965739290427392),
            #[cfg(feature = "u128")] (YOBI_BYTE, 9671406556917033397649408),
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
        #[cfg(feature = "u128")]
        assert_eq!(Unit(Some(Yobi), Byte), Unit::MAX);
        #[cfg(not(feature = "u128"))]
        assert_eq!(Unit(Some(Exbi), Byte), Unit::MAX);
    }

    #[test]
    fn unit_format_and_display_symbol() {
        #[rustfmt::skip]
        let map = [
        //  size,
        //   |         condensed,
        //   |           |  initials,
        //   |           |    |   normal,
        //   |           |    |     |    long,
        //   |           |    |     |      |          long_single_caps,
        //   |           |    |     |      |            |          long_extra,
        //   |           |    |     |      |            |            |          long_extra_single_caps,
        //   |           |    |     |      |            |            |            |
            (BIT      , "b", "b" , "b"  , "Bit"      , "Bit"      , "Bits"     , "Bits"     ),
            (BYTE     , "B", "B" , "B"  , "Byte"     , "Byte"     , "Bytes"    , "Bytes"    ),
            (KILO_BIT , "K", "Kb", "Kb" , "KiloBit"  , "Kilobit"  , "KiloBits" , "Kilobits" ),
            (KILO_BYTE, "K", "KB", "KB" , "KiloByte" , "Kilobyte" , "KiloBytes", "Kilobytes"),
            (MEGA_BIT , "M", "Mb", "Mb" , "MegaBit"  , "Megabit"  , "MegaBits" , "Megabits" ),
            (MEGA_BYTE, "M", "MB", "MB" , "MegaByte" , "Megabyte" , "MegaBytes", "Megabytes"),
            (GIGA_BIT , "G", "Gb", "Gb" , "GigaBit"  , "Gigabit"  , "GigaBits" , "Gigabits" ),
            (GIGA_BYTE, "G", "GB", "GB" , "GigaByte" , "Gigabyte" , "GigaBytes", "Gigabytes"),
            (TERA_BIT , "T", "Tb", "Tb" , "TeraBit"  , "Terabit"  , "TeraBits" , "Terabits" ),
            (TERA_BYTE, "T", "TB", "TB" , "TeraByte" , "Terabyte" , "TeraBytes", "Terabytes"),
            (PETA_BIT , "P", "Pb", "Pb" , "PetaBit"  , "Petabit"  , "PetaBits" , "Petabits" ),
            (PETA_BYTE, "P", "PB", "PB" , "PetaByte" , "Petabyte" , "PetaBytes", "Petabytes"),
            (EXA_BIT  , "E", "Eb", "Eb" , "ExaBit"   , "Exabit"   , "ExaBits"  , "Exabits"  ),
            (EXA_BYTE , "E", "EB", "EB" , "ExaByte"  , "Exabyte"  , "ExaBytes" , "Exabytes" ),
            #[cfg(feature = "u128")] (ZETTA_BIT , "Z", "Zb", "Zb" , "ZettaBit" , "Zettabit" , "ZettaBits" , "Zettabits" ),
            #[cfg(feature = "u128")] (ZETTA_BYTE, "Z", "ZB", "ZB" , "ZettaByte", "Zettabyte", "ZettaBytes", "Zettabytes"),
            #[cfg(feature = "u128")] (YOTTA_BIT , "Y", "Yb", "Yb" , "YottaBit" , "Yottabit" , "YottaBits" , "Yottabits" ),
            #[cfg(feature = "u128")] (YOTTA_BYTE, "Y", "YB", "YB" , "YottaByte", "Yottabyte", "YottaBytes", "Yottabytes"),
            (KIBI_BIT , "K", "Kb", "Kib", "KibiBit"  , "Kibibit"  , "KibiBits" , "Kibibits" ),
            (KIBI_BYTE, "K", "KB", "KiB", "KibiByte" , "Kibibyte" , "KibiBytes", "Kibibytes"),
            (MEBI_BIT , "M", "Mb", "Mib", "MebiBit"  , "Mebibit"  , "MebiBits" , "Mebibits" ),
            (MEBI_BYTE, "M", "MB", "MiB", "MebiByte" , "Mebibyte" , "MebiBytes", "Mebibytes"),
            (GIBI_BIT , "G", "Gb", "Gib", "GibiBit"  , "Gibibit"  , "GibiBits" , "Gibibits" ),
            (GIBI_BYTE, "G", "GB", "GiB", "GibiByte" , "Gibibyte" , "GibiBytes", "Gibibytes"),
            (TEBI_BIT , "T", "Tb", "Tib", "TebiBit"  , "Tebibit"  , "TebiBits" , "Tebibits" ),
            (TEBI_BYTE, "T", "TB", "TiB", "TebiByte" , "Tebibyte" , "TebiBytes", "Tebibytes"),
            (PEBI_BIT , "P", "Pb", "Pib", "PebiBit"  , "Pebibit"  , "PebiBits" , "Pebibits" ),
            (PEBI_BYTE, "P", "PB", "PiB", "PebiByte" , "Pebibyte" , "PebiBytes", "Pebibytes"),
            (EXBI_BIT , "E", "Eb", "Eib", "ExbiBit"  , "Exbibit"  , "ExbiBits" , "Exbibits" ),
            (EXBI_BYTE, "E", "EB", "EiB", "ExbiByte" , "Exbibyte" , "ExbiBytes", "Exbibytes"),
            #[cfg(feature = "u128")] (ZEBI_BIT , "Z", "Zb", "Zib", "ZebiBit"  , "Zebibit"  , "ZebiBits"  , "Zebibits"  ),
            #[cfg(feature = "u128")] (ZEBI_BYTE, "Z", "ZB", "ZiB", "ZebiByte" , "Zebibyte" , "ZebiBytes" , "Zebibytes" ),
            #[cfg(feature = "u128")] (YOBI_BIT , "Y", "Yb", "Yib", "YobiBit"  , "Yobibit"  , "YobiBits"  , "Yobibits"  ),
            #[cfg(feature = "u128")] (YOBI_BYTE, "Y", "YB", "YiB", "YobiByte" , "Yobibyte" , "YobiBytes" , "Yobibytes" ),
        ];

        for (
            unit,
            condensed,
            initials,
            normal,
            long,
            long_single_caps,
            long_extra,
            long_extra_single_caps,
        ) in map.iter()
        {
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
                (unit.symbol_long(false, true), format!("{:+#}", unit)),
                "expected [{:?}] to be represented in long form as [{}]",
                unit,
                long
            );
            assert_eq!(
                long_single_caps.to_string(),
                unit.symbol_long(false, false),
                "expected [{:?}] to be represented in long single-caps form as [{}]",
                unit,
                long
            );
            assert_eq!(
                (long_extra.to_string(), long_extra.to_string()),
                (unit.symbol_long(true, true), format!("{:+}", unit)),
                "expected [{:?}] to be represented in long, plural form as [{}]",
                unit,
                long_extra
            );
            assert_eq!(
                long_extra_single_caps.to_string(),
                unit.symbol_long(true, false),
                "expected [{:?}] to be represented in long, plural, single-caps form as [{}]",
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
        #[rustfmt::skip]
        assert_eq!(
            {
                #[cfg(not(feature = "case-insensitive"))]
                { Err(ParseError { kind: ParseErrorKind::InvalidUnitCaseFormat }) }
                #[cfg(feature = "case-insensitive")]
                { Ok(KIBI_BIT) }
            },
            "kib".parse::<Unit>()
        );
        #[rustfmt::skip]
        assert_eq!(
            {
                #[cfg(not(feature = "case-insensitive"))]
                { Err(ParseError { kind: ParseErrorKind::InvalidUnitCaseFormat }) }
                #[cfg(feature = "case-insensitive")]
                { Ok(MEGA_BIT) }
            },
            // with the default case sensitivity on,
            // the only valid small-caps prefix is `k`
            // even then, it's only valid in the decimal system
            // so while 'kB' is valid, 'kib' is not
            // turn on the `feature="case-insensitive"` flag to relax this
            // and allow all prefixes to be parsed case-insensitively
            // 'kb', 'mB', 'gIB' alike
            "mb".parse::<Unit>()
        );
        assert_eq!(Ok(MEGA_BIT), "Mb".parse::<Unit>());
        assert_eq!(Ok(MEGA_BYTE), "MB".parse::<Unit>());
        assert_eq!(Ok(MEBI_BIT), "Mib".parse::<Unit>());
        assert_eq!(Ok(MEBI_BYTE), "MiB".parse::<Unit>());
        #[rustfmt::skip]
        assert_eq!(
            {
                #[cfg(not(feature = "case-insensitive"))]
                { Err(ParseError { kind: ParseErrorKind::InvalidUnitCaseFormat }) }
                #[cfg(feature = "case-insensitive")]
                { Ok(MEBI_BIT) }
            },
            "mib".parse::<Unit>()
        );
        assert_eq!(Ok(MEGA_BIT), "MegaBit".parse::<Unit>());
        assert_eq!(Ok(MEGA_BYTE), "MegaByte".parse::<Unit>());
        assert_eq!(Ok(GIGA_BIT), "gigabit".parse::<Unit>()); // it is case insensitive in the long form
        assert_eq!(Ok(GIGA_BYTE), "gigabyte".parse::<Unit>());
        assert_eq!(
            Err(ParseError {
                kind: ParseErrorKind::EmptyInput
            }),
            "".parse::<Unit>()
        );
        assert_eq!(
            Err(ParseError {
                kind: ParseErrorKind::InvalidSizeVariant
            }),
            "m".parse::<Unit>()
        );
        assert_eq!(
            Err(ParseError {
                kind: ParseErrorKind::InvalidPrefix
            }),
            "m b".parse::<Unit>()
        );
    }
}

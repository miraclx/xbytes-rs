use super::{Int, ParseError};
use std::{fmt, str::FromStr};

#[rustfmt::skip]
#[derive(Eq, Copy, Clone, Debug, PartialEq)]
pub enum UnitPrefix {
    Kilo, Kibi,
    Mega, Mebi,
    Giga, Gibi,
    Tera, Tebi,
    Peta, Pebi,
    Exa , Exbi,
    #[cfg(feature = "u128")] Zetta,
    #[cfg(feature = "u128")] Zebi ,
    #[cfg(feature = "u128")] Yotta,
    #[cfg(feature = "u128")] Yobi ,
}

use UnitPrefix::*;

impl UnitPrefix {
    #[rustfmt::skip]
    pub const DECIMAL: [UnitPrefix; {
        #[cfg(feature = "u128")] { 8 }
        #[cfg(not(feature = "u128"))] { 6 }
    }] = [
        Kilo, Mega, Giga, Tera, Peta, Exa,
        #[cfg(feature = "u128")] Zetta,
        #[cfg(feature = "u128")] Yotta,
    ];

    #[rustfmt::skip]
    pub const BINARY: [UnitPrefix; {
        #[cfg(feature = "u128")] { 8 }
        #[cfg(not(feature = "u128"))] { 6 }
    }] = [
        Kibi, Mebi, Gibi, Tebi, Pebi, Exbi,
        #[cfg(feature = "u128")] Zebi,
        #[cfg(feature = "u128")] Yobi,
    ];

    #[rustfmt::skip]
    pub const ALL: [UnitPrefix; {
        #[cfg(feature = "u128")] { 16 }
        #[cfg(not(feature = "u128"))] { 12 }
    }] = [
        Kilo, Mega, Giga, Tera, Peta, Exa,
        #[cfg(feature = "u128")] Zetta,
        #[cfg(feature = "u128")] Yotta,
        Kibi, Mebi, Gibi, Tebi, Pebi, Exbi,
        #[cfg(feature = "u128")] Zebi,
        #[cfg(feature = "u128")] Yobi,
    ];

    pub const MIN: UnitPrefix = Kilo;
    #[rustfmt::skip]
    pub const MAX: UnitPrefix = {
        #[cfg(feature = "u128")]      { Yobi }
        #[cfg(not(feature = "u128"))] { Exbi }
    };

    pub const fn is_decimal(&self) -> bool {
        ((*self as u8) & 1) == 0
    }

    pub const fn is_binary(&self) -> bool {
        ((*self as u8) & 1) == 1
    }

    pub const fn index(&self) -> usize {
        (*self as usize) / 2
    }

    pub const fn decimal(&self) -> Self {
        if self.is_binary() {
            return Self::DECIMAL[self.index()];
        }
        *self
    }

    pub const fn binary(&self) -> Self {
        if self.is_decimal() {
            return Self::BINARY[self.index()];
        }
        *self
    }

    #[rustfmt::skip]
    #[inline(always)]
    pub const fn effective_value(&self) -> Int {
        match self {
            Kibi => 1 << 10,   Kilo => 1000,
            Mebi => 1 << 20,   Mega => 1000000,
            Gibi => 1 << 30,   Giga => 1000000000,
            Tebi => 1 << 40,   Tera => 1000000000000,
            Pebi => 1 << 50,   Peta => 1000000000000000,
            Exbi => 1 << 60,   Exa  => 1000000000000000000,
            #[cfg(feature = "u128")] Zebi  => 1 << 70,
            #[cfg(feature = "u128")] Yobi  => 1 << 80,
            #[cfg(feature = "u128")] Zetta => 1000000000000000000000,
            #[cfg(feature = "u128")] Yotta => 1000000000000000000000000,
        }
    }

    #[rustfmt::skip]
    pub const fn symbol(&self) -> &'static str {
        match self {
            Kilo => "K",   Kibi => "Ki",
            Mega => "M",   Mebi => "Mi",
            Giga => "G",   Gibi => "Gi",
            Tera => "T",   Tebi => "Ti",
            Peta => "P",   Pebi => "Pi",
            Exa  => "E",   Exbi => "Ei",
            #[cfg(feature = "u128")] Zetta => "Z" ,
            #[cfg(feature = "u128")] Yotta => "Y" ,
            #[cfg(feature = "u128")] Zebi  => "Zi",
            #[cfg(feature = "u128")] Yobi  => "Yi",
        }
    }

    #[rustfmt::skip]
    pub const fn symbol_long(&self) -> &'static str {
        match self {
            Kilo => "Kilo",   Kibi => "Kibi",
            Mega => "Mega",   Mebi => "Mebi",
            Giga => "Giga",   Gibi => "Gibi",
            Tera => "Tera",   Tebi => "Tebi",
            Peta => "Peta",   Pebi => "Pebi",
            Exa  => "Exa" ,   Exbi => "Exbi",
            #[cfg(feature = "u128")] Zetta => "Zetta",
            #[cfg(feature = "u128")] Yotta => "Yotta",
            #[cfg(feature = "u128")] Zebi  => "Zebi" ,
            #[cfg(feature = "u128")] Yobi  => "Yobi" ,
        }
    }

    #[rustfmt::skip]
    pub const fn symbol_initials(&self) -> &'static str {
        match self {
            Kilo | Kibi => "K",
            Mega | Mebi => "M",
            Giga | Gibi => "G",
            Tera | Tebi => "T",
            Peta | Pebi => "P",
            Exa  | Exbi => "E",
            #[cfg(feature = "u128")] Zetta | Zebi => "Z",
            #[cfg(feature = "u128")] Yotta | Yobi => "Y",
        }
    }
}

impl fmt::Display for UnitPrefix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let unit = if f.sign_minus() {
            self.symbol_initials()
        } else if f.sign_plus() {
            self.symbol_long()
        } else {
            self.symbol()
        };
        f.write_str(unit)
    }
}

impl FromStr for UnitPrefix {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        #[rustfmt::skip]
        let unit = match s {
            "" => return Err(ParseError::EmptyInput),
            // https://web.archive.org/web/20150324153922/https://pacoup.com/2009/05/26/kb-kb-kib-whats-up-with-that/
            "k" | "K"  => Kilo,   "Ki"  => Kibi,
            "M"        => Mega,   "Mi"  => Mebi,
            "G"        => Giga,   "Gi"  => Gibi,
            "T"        => Tera,   "Ti"  => Tebi,
            "P"        => Peta,   "Pi"  => Pebi,
            "E"        => Exa ,   "Ei"  => Exbi,
            #[cfg(feature = "u128")] "Z"   => Zetta,
            #[cfg(feature = "u128")] "Y"   => Yotta,
            #[cfg(feature = "u128")] "Zi"  => Zebi ,
            #[cfg(feature = "u128")] "Yi"  => Yobi ,
            s if (
                matches!(s,
                    "m" | "g" | "t" | "p" | "e" | "ki" | "mi" | "gi" | "ti" | "pi" | "ei"
                ) || (cfg!(feature = "u128") && matches!(s, "z" | "y" | "zi" | "yi"))
            ) => return Err(ParseError::InvalidPrefixCaseFormat),
            s => match s.to_lowercase().as_str() {
                "kilo" => Kilo,   "kibi" => Kibi,
                "mega" => Mega,   "mebi" => Mebi,
                "giga" => Giga,   "gibi" => Gibi,
                "tera" => Tera,   "tebi" => Tebi,
                "peta" => Peta,   "pebi" => Pebi,
                "exa"  => Exa ,   "exbi" => Exbi,
                #[cfg(feature = "u128")] "zetta" => Zetta,
                #[cfg(feature = "u128")] "yotta" => Yotta,
                #[cfg(feature = "u128")] "zebi"  => Zebi ,
                #[cfg(feature = "u128")] "yobi"  => Yobi ,
                _ => return Err(ParseError::InvalidPrefix),
            }
        };
        Ok(unit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decimal() {
        #[rustfmt::skip]
        let lhs = [
            Kilo, Mega, Giga, Tera, Peta, Exa,
            #[cfg(feature = "u128")] Zetta,
            #[cfg(feature = "u128")] Yotta
        ];

        for (index, unit) in lhs.iter().enumerate() {
            assert_eq!(unit, &UnitPrefix::DECIMAL[index]);
            assert_ne!(unit, &UnitPrefix::BINARY[index]);
        }
    }

    #[test]
    fn binary() {
        #[rustfmt::skip]
        let lhs = [
            Kibi, Mebi, Gibi, Tebi, Pebi, Exbi,
            #[cfg(feature = "u128")] Zebi,
            #[cfg(feature = "u128")] Yobi
        ];

        for (index, unit) in lhs.iter().enumerate() {
            assert_eq!(unit, &UnitPrefix::BINARY[index]);
            assert_ne!(unit, &UnitPrefix::DECIMAL[index]);
        }
    }

    #[test]
    fn is_decimal() {
        for unit in UnitPrefix::DECIMAL.iter() {
            assert!(unit.is_decimal())
        }
        for unit in UnitPrefix::BINARY.iter() {
            assert!(!unit.is_decimal())
        }
    }

    #[test]
    fn is_binary() {
        for unit in UnitPrefix::BINARY.iter() {
            assert!(unit.is_binary())
        }
        for unit in UnitPrefix::DECIMAL.iter() {
            assert!(!unit.is_binary())
        }
    }

    #[test]
    fn index() {
        #[rustfmt::skip]
        let map  = [
            (Kilo,   0), (Kibi,   0),
            (Mega,   1), (Mebi,   1),
            (Giga,   2), (Gibi,   2),
            (Tera,   3), (Tebi,   3),
            (Peta,   4), (Pebi,   4),
            (Exa ,   5), (Exbi,   5),
            #[cfg(feature = "u128")] (Zetta,   6),
            #[cfg(feature = "u128")] (Yotta,   7),
            #[cfg(feature = "u128")] (Zebi ,   6),
            #[cfg(feature = "u128")] (Yobi ,   7),
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
    fn to_decimal() {
        #[rustfmt::skip]
        let map = [
            (Kilo,   Kilo), (Kibi,   Kilo),
            (Mega,   Mega), (Mebi,   Mega),
            (Giga,   Giga), (Gibi,   Giga),
            (Tera,   Tera), (Tebi,   Tera),
            (Peta,   Peta), (Pebi,   Peta),
            (Exa ,   Exa ), (Exbi,   Exa ),
            #[cfg(feature = "u128")] (Zetta,   Zetta),
            #[cfg(feature = "u128")] (Yotta,   Yotta),
            #[cfg(feature = "u128")] (Zebi ,   Zetta),
            #[cfg(feature = "u128")] (Yobi ,   Yotta),
        ];

        for (unit, expected) in map.iter() {
            assert_eq!(
                *expected,
                unit.decimal(),
                "expected [{:?}] to be represented as [{:?}] in decimal",
                unit,
                expected
            );
        }
    }

    #[test]
    fn to_binary() {
        #[rustfmt::skip]
        let map = [
            (Kilo,   Kibi), (Kibi,   Kibi),
            (Mega,   Mebi), (Mebi,   Mebi),
            (Giga,   Gibi), (Gibi,   Gibi),
            (Tera,   Tebi), (Tebi,   Tebi),
            (Peta,   Pebi), (Pebi,   Pebi),
            (Exa ,   Exbi), (Exbi,   Exbi),
            #[cfg(feature = "u128")] (Zetta,   Zebi),
            #[cfg(feature = "u128")] (Yotta,   Yobi),
            #[cfg(feature = "u128")] (Zebi ,   Zebi),
            #[cfg(feature = "u128")] (Yobi ,   Yobi),
        ];

        for (unit, expected) in map.iter() {
            assert_eq!(
                *expected,
                unit.binary(),
                "expected [{:?}] to be represented as [{:?}] in binary",
                unit,
                expected
            );
        }
    }

    #[test]
    fn format_and_display_symbol() {
        #[rustfmt::skip]
        let map = [
            (Kilo,   "K"), (Kibi,   "Ki"),
            (Mega,   "M"), (Mebi,   "Mi"),
            (Giga,   "G"), (Gibi,   "Gi"),
            (Tera,   "T"), (Tebi,   "Ti"),
            (Peta,   "P"), (Pebi,   "Pi"),
            (Exa ,   "E"), (Exbi,   "Ei"),
            #[cfg(feature = "u128")] (Zetta,   "Z" ),
            #[cfg(feature = "u128")] (Yotta,   "Y" ),
            #[cfg(feature = "u128")] (Zebi ,   "Zi"),
            #[cfg(feature = "u128")] (Yobi ,   "Yi"),
        ];

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
    fn format_and_display_symbol_long() {
        #[rustfmt::skip]
        let map = [
            (Kilo,   "Kilo"),  (Kibi,   "Kibi"),
            (Mega,   "Mega"),  (Mebi,   "Mebi"),
            (Giga,   "Giga"),  (Gibi,   "Gibi"),
            (Tera,   "Tera"),  (Tebi,   "Tebi"),
            (Peta,   "Peta"),  (Pebi,   "Pebi"),
            (Exa ,   "Exa" ),  (Exbi,   "Exbi"),
            #[cfg(feature = "u128")] (Zetta,   "Zetta"),
            #[cfg(feature = "u128")] (Yotta,   "Yotta"),
            #[cfg(feature = "u128")] (Zebi ,   "Zebi" ),
            #[cfg(feature = "u128")] (Yobi ,   "Yobi" ),
        ];

        for (unit, repr) in map.iter() {
            assert_eq!(
                *repr,
                unit.symbol_long(),
                "expected [{:?}] to be represented in long form as {}",
                unit,
                repr
            );
            assert_eq!(
                *repr,
                format!("{:+}", unit),
                "expected [{:?}] to be represented in long form as {}",
                unit,
                repr
            );
        }
    }

    #[test]
    fn format_and_display_symbol_initials() {
        #[rustfmt::skip]
        let map = [
            (Kilo,   "K"),  (Kibi,   "K"),
            (Mega,   "M"),  (Mebi,   "M"),
            (Giga,   "G"),  (Gibi,   "G"),
            (Tera,   "T"),  (Tebi,   "T"),
            (Peta,   "P"),  (Pebi,   "P"),
            (Exa ,   "E"),  (Exbi,   "E"),
            #[cfg(feature = "u128")] (Zetta,   "Z"),
            #[cfg(feature = "u128")] (Yotta,   "Y"),
            #[cfg(feature = "u128")] (Zebi ,   "Z"),
            #[cfg(feature = "u128")] (Yobi ,   "Y"),
        ];

        for (unit, repr) in map.iter() {
            assert_eq!(
                *repr,
                unit.symbol_initials(),
                "expected [{:?}] to be represented in short form as {}",
                unit,
                repr
            );
            assert_eq!(
                *repr,
                format!("{:-}", unit),
                "expected [{:?}] to be represented in short form as {}",
                unit,
                repr
            );
        }
    }

    #[test]
    fn str_parse() {
        #[rustfmt::skip]
        let map = [
            ("k"  , Ok(Kilo)),
            ("K"  , Ok(Kilo)),  ("Ki"  , Ok(Kibi)),
            ("M"  , Ok(Mega)),  ("Mi"  , Ok(Mebi)),
            ("G"  , Ok(Giga)),  ("Gi"  , Ok(Gibi)),
            ("T"  , Ok(Tera)),  ("Ti"  , Ok(Tebi)),
            ("P"  , Ok(Peta)),  ("Pi"  , Ok(Pebi)),
            ("E"  , Ok(Exa )),  ("Ei"  , Ok(Exbi)),
            #[cfg(feature = "u128")] ("Z" , Ok(Zetta)),
            #[cfg(feature = "u128")] ("Y" , Ok(Yotta)),
            #[cfg(feature = "u128")] ("Zi", Ok(Zebi )),
            #[cfg(feature = "u128")] ("Yi", Ok(Yobi )),
        ];

        assert_eq!(Err(ParseError::EmptyInput), "".parse::<UnitPrefix>());

        for (value, unit) in map.iter() {
            assert_eq!(*unit, value.parse::<UnitPrefix>());
        }

        #[rustfmt::skip]
        let invalid_formats = [
                 "ki", "m", "mi", "g", "gi",
            "t", "ti", "p", "pi", "e", "ei",
            #[cfg(feature = "u128")] "z" ,
            #[cfg(feature = "u128")] "zi",
            #[cfg(feature = "u128")] "y" ,
            #[cfg(feature = "u128")] "yi",
        ];

        for value in invalid_formats.iter() {
            assert_eq!(
                Err(ParseError::InvalidPrefixCaseFormat),
                value.parse::<UnitPrefix>()
            );
        }
    }

    #[test]
    fn effective_value() {
        #[rustfmt::skip]
        let map = [
            (Kilo, 1000),                 (Kibi, 1024),
            (Mega, 1000000),              (Mebi, 1048576),
            (Giga, 1000000000),           (Gibi, 1073741824),
            (Tera, 1000000000000),        (Tebi, 1099511627776),
            (Peta, 1000000000000000),     (Pebi, 1125899906842624),
            (Exa , 1000000000000000000),  (Exbi, 1152921504606846976),
            #[cfg(feature = "u128")] (Zetta, 1000000000000000000000),
            #[cfg(feature = "u128")] (Yotta, 1000000000000000000000000),
            #[cfg(feature = "u128")] (Zebi , 1180591620717411303424),
            #[cfg(feature = "u128")] (Yobi , 1208925819614629174706176)
        ];

        for (prefix, value) in map.iter() {
            assert_eq!(
                *value,
                prefix.effective_value(),
                "expected [{:?}] to have the value [{}]",
                prefix,
                value
            );
        }
    }

    #[test]
    fn min_max() {
        assert_eq!(Kilo, UnitPrefix::MIN);
        #[cfg(feature = "u128")]
        assert_eq!(Yobi, UnitPrefix::MAX);
        #[cfg(not(feature = "u128"))]
        assert_eq!(Exbi, UnitPrefix::MAX);
    }
}

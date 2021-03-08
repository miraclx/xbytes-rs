use super::{Int, ParseError};
use std::fmt;

#[rustfmt::skip]
#[derive(Eq, Copy, Clone, Debug, PartialEq)]
pub enum UnitPrefix {
    Kilo , Kibi,
    Mega , Mebi,
    Giga , Gibi,
    Tera , Tebi,
    Peta , Pebi,
    Exa  , Exbi,
    Zetta, Zebi,
    Yotta, Yobi,
}

use UnitPrefix::*;

impl UnitPrefix {
    pub const DECIMAL: [UnitPrefix; 8] = [Kilo, Mega, Giga, Tera, Peta, Exa, Zetta, Yotta];
    pub const BINARY: [UnitPrefix; 8] = [Kibi, Mebi, Gibi, Tebi, Pebi, Exbi, Zebi, Yobi];

    #[rustfmt::skip]
    pub const ALL: [UnitPrefix; 16] = [
        Kilo, Mega, Giga, Tera, Peta, Exa, Zetta, Yotta,
        Kibi, Mebi, Gibi, Tebi, Pebi, Exbi, Zebi, Yobi,
    ];

    pub const MIN: UnitPrefix = Kilo;
    pub const MAX: UnitPrefix = Yobi;

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
            Kibi => 1 << 10,   Kilo  => 1000,
            Mebi => 1 << 20,   Mega  => 1000000,
            Gibi => 1 << 30,   Giga  => 1000000000,
            Tebi => 1 << 40,   Tera  => 1000000000000,
            Pebi => 1 << 50,   Peta  => 1000000000000000,
            Exbi => 1 << 60,   Exa   => 1000000000000000000,
            Zebi => 1 << 70,   Zetta => 1000000000000000000000,
            Yobi => 1 << 80,   Yotta => 1000000000000000000000000,
        }
    }

    #[rustfmt::skip]
    pub const fn symbol(&self) -> &'static str {
        match self {
            Kilo  => "K",   Kibi => "Ki",
            Mega  => "M",   Mebi => "Mi",
            Giga  => "G",   Gibi => "Gi",
            Tera  => "T",   Tebi => "Ti",
            Peta  => "P",   Pebi => "Pi",
            Exa   => "E",   Exbi => "Ei",
            Zetta => "Z",   Zebi => "Zi",
            Yotta => "Y",   Yobi => "Yi",
        }
    }

    #[rustfmt::skip]
    pub const fn symbol_long(&self) -> &'static str {
        match self {
            Kilo  => "Kilo" ,   Kibi => "Kibi",
            Mega  => "Mega" ,   Mebi => "Mebi",
            Giga  => "Giga" ,   Gibi => "Gibi",
            Tera  => "Tera" ,   Tebi => "Tebi",
            Peta  => "Peta" ,   Pebi => "Pebi",
            Exa   => "Exa"  ,   Exbi => "Exbi",
            Zetta => "Zetta",   Zebi => "Zebi",
            Yotta => "Yotta",   Yobi => "Yobi",
        }
    }

    #[rustfmt::skip]
    pub const fn symbol_initials(&self) -> &'static str {
        match self {
            Kilo  | Kibi => "K",
            Mega  | Mebi => "M",
            Giga  | Gibi => "G",
            Tera  | Tebi => "T",
            Peta  | Pebi => "P",
            Exa   | Exbi => "E",
            Zetta | Zebi => "Z",
            Yotta | Yobi => "Y",
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

impl std::str::FromStr for UnitPrefix {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        #[rustfmt::skip]
        let unit = match s.to_lowercase().as_str() {
            "" => return Err(ParseError::EmptyString),
            "k"     => Kilo ,   "ki"   => Kibi,
            "m"     => Mega ,   "mi"   => Mebi,
            "g"     => Giga ,   "gi"   => Gibi,
            "t"     => Tera ,   "ti"   => Tebi,
            "p"     => Peta ,   "pi"   => Pebi,
            "e"     => Exa  ,   "ei"   => Exbi,
            "z"     => Zetta,   "zi"   => Zebi,
            "y"     => Yotta,   "yi"   => Yobi,
            _ => return Err(ParseError::PrefixParseError),
        };
        Ok(unit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decimal() {
        let lhs = [Kilo, Mega, Giga, Tera, Peta, Exa, Zetta, Yotta];

        for (index, unit) in lhs.iter().enumerate() {
            assert_eq!(unit, &UnitPrefix::DECIMAL[index]);
            assert_ne!(unit, &UnitPrefix::BINARY[index]);
        }
    }

    #[test]
    fn binary() {
        let lhs = [Kibi, Mebi, Gibi, Tebi, Pebi, Exbi, Zebi, Yobi];

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
            (Kilo ,   0), (Kibi,   0),
            (Mega ,   1), (Mebi,   1),
            (Giga ,   2), (Gibi,   2),
            (Tera ,   3), (Tebi,   3),
            (Peta ,   4), (Pebi,   4),
            (Exa  ,   5), (Exbi,   5),
            (Zetta,   6), (Zebi,   6),
            (Yotta,   7), (Yobi,   7),
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
            (Kilo ,   Kilo ), (Kibi,   Kilo ),
            (Mega ,   Mega ), (Mebi,   Mega ),
            (Giga ,   Giga ), (Gibi,   Giga ),
            (Tera ,   Tera ), (Tebi,   Tera ),
            (Peta ,   Peta ), (Pebi,   Peta ),
            (Exa  ,   Exa  ), (Exbi,   Exa  ),
            (Zetta,   Zetta), (Zebi,   Zetta),
            (Yotta,   Yotta), (Yobi,   Yotta),
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
            (Kilo ,   Kibi), (Kibi,   Kibi),
            (Mega ,   Mebi), (Mebi,   Mebi),
            (Giga ,   Gibi), (Gibi,   Gibi),
            (Tera ,   Tebi), (Tebi,   Tebi),
            (Peta ,   Pebi), (Pebi,   Pebi),
            (Exa  ,   Exbi), (Exbi,   Exbi),
            (Zetta,   Zebi), (Zebi,   Zebi),
            (Yotta,   Yobi), (Yobi,   Yobi),
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
            (Kilo ,   "K"), (Kibi,   "Ki"),
            (Mega ,   "M"), (Mebi,   "Mi"),
            (Giga ,   "G"), (Gibi,   "Gi"),
            (Tera ,   "T"), (Tebi,   "Ti"),
            (Peta ,   "P"), (Pebi,   "Pi"),
            (Exa  ,   "E"), (Exbi,   "Ei"),
            (Zetta,   "Z"), (Zebi,   "Zi"),
            (Yotta,   "Y"), (Yobi,   "Yi"),
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
            (Kilo ,   "Kilo" ),  (Kibi,   "Kibi"),
            (Mega ,   "Mega" ),  (Mebi,   "Mebi"),
            (Giga ,   "Giga" ),  (Gibi,   "Gibi"),
            (Tera ,   "Tera" ),  (Tebi,   "Tebi"),
            (Peta ,   "Peta" ),  (Pebi,   "Pebi"),
            (Exa  ,   "Exa"  ),  (Exbi,   "Exbi"),
            (Zetta,   "Zetta"),  (Zebi,   "Zebi"),
            (Yotta,   "Yotta"),  (Yobi,   "Yobi"),
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
            (Kilo ,   "K"),  (Kibi,   "K"),
            (Mega ,   "M"),  (Mebi,   "M"),
            (Giga ,   "G"),  (Gibi,   "G"),
            (Tera ,   "T"),  (Tebi,   "T"),
            (Peta ,   "P"),  (Pebi,   "P"),
            (Exa  ,   "E"),  (Exbi,   "E"),
            (Zetta,   "Z"),  (Zebi,   "Z"),
            (Yotta,   "Y"),  (Yobi,   "Y"),
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
            ("K"    , Kilo ),  ("Ki"  , Kibi),
            ("M"    , Mega ),  ("Mi"  , Mebi),
            ("G"    , Giga ),  ("Gi"  , Gibi),
            ("T"    , Tera ),  ("Ti"  , Tebi),
            ("P"    , Peta ),  ("Pi"  , Pebi),
            ("E"    , Exa  ),  ("Ei"  , Exbi),
            ("Z"    , Zetta),  ("Zi"  , Zebi),
            ("Y"    , Yotta),  ("Yi"  , Yobi),
        ];

        assert_eq!(Err(ParseError::EmptyString), "".parse::<UnitPrefix>());
        for (value, unit) in map.iter() {
            assert_eq!(Ok(*unit), value.parse::<UnitPrefix>());
        }
    }

    #[test]
    fn effective_value() {
        #[rustfmt::skip]
        let map = [
            (Kilo , 1000),                       (Kibi, 1024),
            (Mega , 1000000),                    (Mebi, 1048576),
            (Giga , 1000000000),                 (Gibi, 1073741824),
            (Tera , 1000000000000),              (Tebi, 1099511627776),
            (Peta , 1000000000000000),           (Pebi, 1125899906842624),
            (Exa  , 1000000000000000000),        (Exbi, 1152921504606846976),
            (Zetta, 1000000000000000000000),     (Zebi, 1180591620717411303424),
            (Yotta, 1000000000000000000000000),  (Yobi, 1208925819614629174706176)
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
        assert_eq!(Yobi, UnitPrefix::MAX);
    }
}

use core::fmt;
use core::str::FromStr;

use super::{Int, ParseError};

/// A metric (SI) or binary (IEC) unit prefix used to scale a byte size.
#[rustfmt::skip]
#[derive(Eq, Ord, Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum UnitPrefix {
    /// The decimal kilo prefix (10^3, symbol "k").
    Kilo,
    /// The binary kibi prefix (2^10, symbol "Ki").
    Kibi,
    /// The decimal mega prefix (10^6, symbol "M").
    Mega,
    /// The binary mebi prefix (2^20, symbol "Mi").
    Mebi,
    /// The decimal giga prefix (10^9, symbol "G").
    Giga,
    /// The binary gibi prefix (2^30, symbol "Gi").
    Gibi,
    /// The decimal tera prefix (10^12, symbol "T").
    Tera,
    /// The binary tebi prefix (2^40, symbol "Ti").
    Tebi,
    /// The decimal peta prefix (10^15, symbol "P").
    Peta,
    /// The binary pebi prefix (2^50, symbol "Pi").
    Pebi,
    /// The decimal exa prefix (10^18, symbol "E").
    Exa ,
    /// The binary exbi prefix (2^60, symbol "Ei").
    Exbi,
    /// The decimal zetta prefix (10^21, symbol "Z").
    #[cfg(feature = "u128")] Zetta,
    /// The binary zebi prefix (2^70, symbol "Zi").
    #[cfg(feature = "u128")] Zebi ,
    /// The decimal yotta prefix (10^24, symbol "Y").
    #[cfg(feature = "u128")] Yotta,
    /// The binary yobi prefix (2^80, symbol "Yi").
    #[cfg(feature = "u128")] Yobi ,
}

use UnitPrefix::*;

// `is_decimal`/`is_binary` (even discriminant) and `index` (`/ 2`) hold only while the variants stay
// declared decimal/binary interleaved from zero. Assert that at build time so a future reorder or
// insertion fails to compile instead of silently corrupting every conversion and the `sizes` tables.
const _: () = {
    let mut i = 0;
    while i < UnitPrefix::DECIMAL.len() {
        assert!(
            UnitPrefix::DECIMAL[i] as u8 == (i as u8) * 2,
            "decimal prefixes must sit at even discriminants, in ascending order"
        );
        assert!(
            UnitPrefix::BINARY[i] as u8 == (i as u8) * 2 + 1,
            "each binary prefix must follow its decimal twin at the next discriminant"
        );
        i += 1;
    }
};

impl UnitPrefix {
    /// All decimal (power-of-1000) prefixes in ascending order.
    #[rustfmt::skip]
    pub const DECIMAL: [UnitPrefix; {
        #[cfg(feature = "u128")] { 8 }
        #[cfg(not(feature = "u128"))] { 6 }
    }] = [
        Kilo, Mega, Giga, Tera, Peta, Exa,
        #[cfg(feature = "u128")] Zetta,
        #[cfg(feature = "u128")] Yotta,
    ];

    /// All binary (power-of-1024) prefixes in ascending order.
    #[rustfmt::skip]
    pub const BINARY: [UnitPrefix; {
        #[cfg(feature = "u128")] { 8 }
        #[cfg(not(feature = "u128"))] { 6 }
    }] = [
        Kibi, Mebi, Gibi, Tebi, Pebi, Exbi,
        #[cfg(feature = "u128")] Zebi,
        #[cfg(feature = "u128")] Yobi,
    ];

    /// All prefixes, decimal and binary interleaved, in ascending order.
    #[rustfmt::skip]
    pub const ALL: [UnitPrefix; {
        #[cfg(feature = "u128")] { 16 }
        #[cfg(not(feature = "u128"))] { 12 }
    }] = [
        Kilo, Kibi, Mega, Mebi, Giga, Gibi,
        Tera, Tebi, Peta, Pebi, Exa, Exbi,
        #[cfg(feature = "u128")] Zetta,
        #[cfg(feature = "u128")] Zebi,
        #[cfg(feature = "u128")] Yotta,
        #[cfg(feature = "u128")] Yobi,
    ];

    /// The smallest prefix, useful as a lower bound when scaling.
    pub const MIN: UnitPrefix = Kilo;

    /// The largest available prefix, useful as an upper bound when scaling.
    #[rustfmt::skip]
    pub const MAX: UnitPrefix = {
        #[cfg(feature = "u128")]      { Yobi }
        #[cfg(not(feature = "u128"))] { Exbi }
    };

    /// Whether this is a decimal (power-of-1000) prefix.
    #[must_use]
    pub const fn is_decimal(&self) -> bool {
        ((*self as u8) & 1) == 0
    }

    /// Whether this is a binary (power-of-1024) prefix.
    #[must_use]
    pub const fn is_binary(&self) -> bool {
        ((*self as u8) & 1) == 1
    }

    /// The magnitude index of this prefix, so decimal and binary counterparts share one.
    #[must_use]
    pub const fn index(&self) -> usize {
        (*self as usize) / 2
    }

    /// The decimal prefix of the same magnitude (returns itself if already decimal).
    #[must_use]
    pub const fn decimal(&self) -> Self {
        if self.is_binary() {
            return Self::DECIMAL[self.index()];
        }
        *self
    }

    /// The binary prefix of the same magnitude (returns itself if already binary).
    #[must_use]
    pub const fn binary(&self) -> Self {
        if self.is_decimal() {
            return Self::BINARY[self.index()];
        }
        *self
    }

    /// The numeric multiplier this prefix represents, for converting to raw bytes.
    #[rustfmt::skip]
    #[inline]
    #[must_use]
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

    /// The short symbol for this prefix (e.g. "k" or "Ki").
    #[rustfmt::skip]
    #[must_use]
    pub const fn symbol(&self) -> &'static str {
        match self {
            Kilo => "k",   Kibi => "Ki",
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

    /// The full name for this prefix (e.g. "Kilo" or "Kibi").
    #[rustfmt::skip]
    #[must_use]
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

    /// The single-letter initial shared by a prefix and its counterpart (e.g. "K").
    #[rustfmt::skip]
    #[must_use]
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
    /// Writes the canonical symbol (`Ki`, `M`, ...). For the long name or the
    /// shared initial, call [`symbol_long`](UnitPrefix::symbol_long) or
    /// [`symbol_initials`](UnitPrefix::symbol_initials) directly.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.symbol())
    }
}

impl FromStr for UnitPrefix {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        #[rustfmt::skip]
        let normalized = {
            #[cfg(not(feature = "case-insensitive"))] { s.to_string() }
            #[cfg(feature = "case-insensitive")] {
                if s.is_empty() {
                    s.to_string()
                } else {
                    let (first, rest) = s.split_at(1);
                    format!("{}{}", first.to_uppercase(), rest.to_lowercase())
                }
            }
        };
        #[rustfmt::skip]
        let unit = match normalized.as_str() {
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
            #[cfg(not(feature = "case-insensitive"))]
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
    #[rustfmt::skip]
    fn cmp() {
        assert!(Kilo < Kibi && Kibi > Kilo);
        assert!(Kibi < Mega && Mega > Kibi);
        assert!(Mega < Mebi && Mebi > Mega);
        assert!(Mebi < Giga && Giga > Mebi);
        assert!(Giga < Gibi && Gibi > Giga);
        assert!(Gibi < Tera && Tera > Gibi);
        assert!(Tera < Tebi && Tebi > Tera);
        assert!(Tebi < Peta && Peta > Tebi);
        assert!(Peta < Pebi && Pebi > Peta);
        assert!(Pebi < Exa  && Exa  > Pebi);
        assert!(Exa  < Exbi && Exbi > Exa );

        #[cfg(feature = "u128")] assert!(Exbi  < Zetta && Zetta > Exbi );
        #[cfg(feature = "u128")] assert!(Zetta < Zebi  && Zebi  > Zetta);
        #[cfg(feature = "u128")] assert!(Zebi  < Yotta && Yotta > Zebi );
        #[cfg(feature = "u128")] assert!(Yotta < Yobi  && Yobi  > Yotta);
    }

    #[test]
    fn const_prefix_sorted() {
        fn is_sorted(prefix: &mut [UnitPrefix]) -> bool {
            let a = prefix.windows(2).all(|lr| lr[0] < lr[1]);
            prefix.reverse();
            let b = prefix.windows(2).all(|lr| lr[0] > lr[1]);
            a && b
        }

        assert!(is_sorted(&mut { UnitPrefix::DECIMAL }));
        assert!(is_sorted(&mut { UnitPrefix::BINARY }));
        assert!(is_sorted(&mut { UnitPrefix::ALL }));
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
            (Kilo,   "k"), (Kibi,   "Ki"),
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
            #[cfg(feature = "case-insensitive")] ("k" , Ok(Kilo)),
            #[cfg(feature = "case-insensitive")] ("ki", Ok(Kibi)),
            #[cfg(feature = "case-insensitive")] ("m" , Ok(Mega)),
            #[cfg(feature = "case-insensitive")] ("mi", Ok(Mebi)),
            #[cfg(feature = "case-insensitive")] ("g" , Ok(Giga)),
            #[cfg(feature = "case-insensitive")] ("gi", Ok(Gibi)),
            #[cfg(feature = "case-insensitive")] ("t" , Ok(Tera)),
            #[cfg(feature = "case-insensitive")] ("ti", Ok(Tebi)),
            #[cfg(feature = "case-insensitive")] ("p" , Ok(Peta)),
            #[cfg(feature = "case-insensitive")] ("pi", Ok(Pebi)),
            #[cfg(feature = "case-insensitive")] ("e" , Ok(Exa )),
            #[cfg(feature = "case-insensitive")] ("ei", Ok(Exbi)),
            #[cfg(feature = "case-insensitive")] #[cfg(feature = "u128")] ("z" , Ok(Zetta)),
            #[cfg(feature = "case-insensitive")] #[cfg(feature = "u128")] ("y" , Ok(Yotta)),
            #[cfg(feature = "case-insensitive")] #[cfg(feature = "u128")] ("zi", Ok(Zebi )),
            #[cfg(feature = "case-insensitive")] #[cfg(feature = "u128")] ("yi", Ok(Yobi )),
            #[cfg(feature = "case-insensitive")] ("kI", Ok(Kibi)),
            #[cfg(feature = "case-insensitive")] ("KI", Ok(Kibi)),
            #[cfg(feature = "case-insensitive")] ("mI", Ok(Mebi)),
            #[cfg(feature = "case-insensitive")] ("MI", Ok(Mebi)),
            #[cfg(feature = "case-insensitive")] ("gI", Ok(Gibi)),
            #[cfg(feature = "case-insensitive")] ("GI", Ok(Gibi)),
            #[cfg(feature = "case-insensitive")] ("tI", Ok(Tebi)),
            #[cfg(feature = "case-insensitive")] ("TI", Ok(Tebi)),
            #[cfg(feature = "case-insensitive")] ("pI", Ok(Pebi)),
            #[cfg(feature = "case-insensitive")] ("PI", Ok(Pebi)),
            #[cfg(feature = "case-insensitive")] ("eI", Ok(Exbi)),
            #[cfg(feature = "case-insensitive")] ("EI", Ok(Exbi)),
            #[cfg(feature = "case-insensitive")] #[cfg(feature = "u128")] ("zI", Ok(Zebi)),
            #[cfg(feature = "case-insensitive")] #[cfg(feature = "u128")] ("ZI", Ok(Zebi)),
            #[cfg(feature = "case-insensitive")] #[cfg(feature = "u128")] ("yI", Ok(Yobi)),
            #[cfg(feature = "case-insensitive")] #[cfg(feature = "u128")] ("YI", Ok(Yobi)),
        ];

        assert_eq!(Err(ParseError::EmptyInput), "".parse::<UnitPrefix>());

        for (value, unit) in map.iter() {
            assert_eq!(*unit, value.parse::<UnitPrefix>());
        }

        #[cfg(not(feature = "case-insensitive"))]
        {
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

            #[rustfmt::skip]
            let invalid_prefixes = [
                "kI", "KI", "mI", "MI", "gI", "GI",
                "tI", "TI", "pI", "PI", "eI", "EI",
                #[cfg(feature = "u128")] "zI" ,
                #[cfg(feature = "u128")] "ZI",
                #[cfg(feature = "u128")] "yI" ,
                #[cfg(feature = "u128")] "YI",
            ];

            for value in invalid_prefixes.iter() {
                assert_eq!(Err(ParseError::InvalidPrefix), value.parse::<UnitPrefix>());
            }
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

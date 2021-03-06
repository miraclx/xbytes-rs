use std::fmt;

#[rustfmt::skip]
#[derive(Eq, Copy, Clone, Debug, PartialEq)]
pub enum Unit {
    Kilo , Kibi,
    Mega , Mebi,
    Giga , Gibi,
    Tera , Tebi,
    Peta , Pebi,
    Exa  , Exbi,
    Zetta, Zebi,
    Yotta, Yobi,
}

use Unit::*;

impl Unit {
    const DECIMAL: [Unit; 8] = [Kilo, Mega, Giga, Tera, Peta, Exa, Zetta, Yotta];
    const BINARY: [Unit; 8] = [Kibi, Mebi, Gibi, Tebi, Pebi, Exbi, Zebi, Yobi];

    pub fn is_decimal(&self) -> bool {
        Self::DECIMAL.contains(self)
    }

    pub fn is_binary(&self) -> bool {
        Self::BINARY.contains(self)
    }

    pub fn index(&self) -> usize {
        let stack = if self.is_decimal() {
            Self::DECIMAL
        } else {
            Self::BINARY
        };
        stack.iter().position(|val| val == self).unwrap()
    }

    pub fn decimal(&self) -> Self {
        if self.is_binary() {
            return Self::DECIMAL[self.index()];
        }
        *self
    }

    pub fn binary(&self) -> Self {
        if self.is_decimal() {
            return Self::BINARY[self.index()];
        }
        *self
    }

    #[rustfmt::skip]
    pub fn symbol(&self) -> &str {
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
    pub fn symbol_long(&self) -> &str {
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
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(if !f.alternate() {
            self.symbol()
        } else {
            self.symbol_long()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decimal() {
        let lhs = [Kilo, Mega, Giga, Tera, Peta, Exa, Zetta, Yotta];

        let right_rhs = [Kilo, Mega, Giga, Tera, Peta, Exa, Zetta, Yotta];
        let wrong_rhs = [Kibi, Mebi, Gibi, Tebi, Pebi, Exbi, Zebi, Yobi];

        for (index, unit) in lhs.iter().enumerate() {
            assert_eq!(unit, &right_rhs[index]);
            assert_ne!(unit, &wrong_rhs[index]);
        }
    }

    #[test]
    fn binary() {
        let lhs = [Kibi, Mebi, Gibi, Tebi, Pebi, Exbi, Zebi, Yobi];

        let right_rhs = [Kibi, Mebi, Gibi, Tebi, Pebi, Exbi, Zebi, Yobi];
        let wrong_rhs = [Kilo, Mega, Giga, Tera, Peta, Exa, Zetta, Yotta];

        for (index, unit) in lhs.iter().enumerate() {
            assert_eq!(unit, &right_rhs[index]);
            assert_ne!(unit, &wrong_rhs[index]);
        }
    }

    #[test]
    fn is_decimal() {
        for unit in [Kilo, Mega, Giga, Tera, Peta, Exa, Zetta, Yotta].iter() {
            assert!(unit.is_decimal())
        }
    }

    #[test]
    fn is_binary() {
        for unit in [Kibi, Mebi, Gibi, Tebi, Pebi, Exbi, Zebi, Yobi].iter() {
            assert!(unit.is_binary())
        }
    }

    #[test]
    fn index() {
        #[rustfmt::skip]
        let map: [(Unit, usize); 16] = [
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
    fn format_and_display_symbol() {
        #[rustfmt::skip]
        let map: [(Unit, &str); 16] = [
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
        let map: [(Unit, &str); 16] = [
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
                format!("{:#}", unit),
                "expected [{:?}] to be represented in long form as {}",
                unit,
                repr
            );
        }
    }
}

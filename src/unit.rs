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
                unit.index(),
                *index,
                "expected [{:?}] to have the index {}",
                unit,
                index
            );
        }
    }
}

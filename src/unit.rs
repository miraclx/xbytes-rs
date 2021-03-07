use super::UnitPrefix::{self, *};

#[derive(Eq, Copy, Clone, Debug, PartialEq)]
pub enum SizeVariant {
    Bit,
    Byte,
}

use SizeVariant::*;

#[derive(Eq, Copy, Clone, Debug, PartialEq)]
pub struct Unit(UnitPrefix, SizeVariant);

pub mod Decimal {
    use super::Unit;

    #[rustfmt::skip]
    pub(super) mod _exported {
        use super::super::*;
        pub const KILO_BIT  : Unit = Unit(Kilo , Bit);
        pub const MEGA_BIT  : Unit = Unit(Mega , Bit);
        pub const GIGA_BIT  : Unit = Unit(Giga , Bit);
        pub const TERA_BIT  : Unit = Unit(Tera , Bit);
        pub const PETA_BIT  : Unit = Unit(Peta , Bit);
        pub const EXA_BIT   : Unit = Unit(Exa  , Bit);
        pub const ZETTA_BIT : Unit = Unit(Zetta, Bit);
        pub const YOTTA_BIT : Unit = Unit(Yotta, Bit);
        pub const KILO_BYTE : Unit = Unit(Kilo , Byte);
        pub const MEGA_BYTE : Unit = Unit(Mega , Byte);
        pub const GIGA_BYTE : Unit = Unit(Giga , Byte);
        pub const TERA_BYTE : Unit = Unit(Tera , Byte);
        pub const PETA_BYTE : Unit = Unit(Peta , Byte);
        pub const EXA_BYTE  : Unit = Unit(Exa  , Byte);
        pub const ZETTA_BYTE: Unit = Unit(Zetta, Byte);
        pub const YOTTA_BYTE: Unit = Unit(Yotta, Byte);
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
        pub const KIBI_BIT : Unit = Unit(Kibi, Bit);
        pub const MEBI_BIT : Unit = Unit(Mebi, Bit);
        pub const GIBI_BIT : Unit = Unit(Gibi, Bit);
        pub const TEBI_BIT : Unit = Unit(Tebi, Bit);
        pub const PEBI_BIT : Unit = Unit(Pebi, Bit);
        pub const EXBI_BIT : Unit = Unit(Exbi, Bit);
        pub const ZEBI_BIT : Unit = Unit(Zebi, Bit);
        pub const YOBI_BIT : Unit = Unit(Yobi, Bit);
        pub const KIBI_BYTE: Unit = Unit(Kibi, Byte);
        pub const MEBI_BYTE: Unit = Unit(Mebi, Byte);
        pub const GIBI_BYTE: Unit = Unit(Gibi, Byte);
        pub const TEBI_BYTE: Unit = Unit(Tebi, Byte);
        pub const PEBI_BYTE: Unit = Unit(Pebi, Byte);
        pub const EXBI_BYTE: Unit = Unit(Exbi, Byte);
        pub const ZEBI_BYTE: Unit = Unit(Zebi, Byte);
        pub const YOBI_BYTE: Unit = Unit(Yobi, Byte);
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

    pub const fn symbol_long(&self) -> &'static str {
        match self {
            Bit => "Bit",
            Byte => "Byte",
        }
    }
}

impl Unit {
    pub const fn is_decimal(&self) -> bool {
        self.0.is_decimal()
    }

    pub const fn is_binary(&self) -> bool {
        self.0.is_binary()
    }

    pub const fn is_bit(&self) -> bool {
        self.1.is_bit()
    }

    pub const fn is_byte(&self) -> bool {
        self.1.is_byte()
    }

    pub const fn index(&self) -> usize {
        self.0.index()
    }

    pub const fn decimal(&self) -> Self {
        Self(self.0.decimal(), self.1)
    }

    pub const fn binary(&self) -> Self {
        Self(self.0.binary(), self.1)
    }

    pub const fn bit(&self) -> Self {
        Self(self.0, Bit)
    }

    pub const fn byte(&self) -> Self {
        Self(self.0, Byte)
    }

    pub const fn symbols(&self) -> (&'static str, &'static str) {
        (self.0.symbol(), self.1.symbol())
    }

    pub fn symbol(&self) -> String {
        let (unit, size_variant) = self.symbols();
        format!("{}{}", unit, size_variant)
    }

    pub fn symbols_long(&self) -> (&'static str, &'static str) {
        (self.0.symbol_long(), self.1.symbol_long())
    }

    pub fn symbol_long(&self) -> String {
        let (unit, size_variant) = self.symbols_long();
        format!("{}{}", unit, size_variant)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let a = Sizes::KIBI_BIT;
    }
}

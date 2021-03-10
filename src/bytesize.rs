use super::{Int, Unit};

mod flags {
    #![allow(non_upper_case_globals)]
    use bitflags::bitflags;

    bitflags! {
        pub struct Mode: u8 {
            const Bits    = 0b01;
            const Decimal = 0b10;
        }
    }

    bitflags! {
        pub struct Format: u8 {
            const Initials           = 0b00000001;
            const Condensed          = 0b00000010;
            const Long               = 0b00000100;
            const Plural             = 0b00001000;
            const LowerCaps          = 0b00010000;
            const UpperCaps          = 0b00100000;
            const ForceFraction      = 0b01000000;
            const ThousandsSeparator = 0b10000000;
        }
    }
}

pub use flags::*;

pub struct ByteSize(Int);

impl ByteSize {
    // let size = ByteSize(1039384);

    // size.to_string(Mode::Binary) -> '1015.02 KiB'
    // size.to_string(Mode::Binary | Mode::Bits) -> '7.93 Mib'
    // size.to_string_as(Mode::Binary, Format::SmallCaps) -> '1015.02 kib'

    // size.to_string(Mode::Decimal) -> '1.04 MB'
    // size.to_string_as(Mode::Decimal, Format::Long) -> '1.04 MegaBytes'
    // size.to_string_as(Mode::Decimal, Format::Long | Format::SmallCaps) -> '1.04 megabytes'
    // size.to_string_as(Mode::Decimal, Format::Long | Format::NoPlural) -> '1.04 MegaByte'
    // size.to_string_as(Mode::Decimal, Format::Initials | Format::NoSpace) -> '1.04M'

    // size.repr(KIBI_BYTE) -> '1015.02 KiB'
    // size.repr_as(KIBI_BYTE, Format::Initials) -> '1015.02 KB'
    // size.repr_as(KIBI_BYTE, Format::Condensed) -> '1015.02 K'
    // size.repr_as(KIBI_BYTE, Format::ThousandsSeparator) -> '1,039.38 KiB'
    // size.repr_as(KIBI_BYTE, Format::ThousandsSeparator) -> '1,039.38 KiB'
    // size.repr_as(KIBI_BYTE, Format::Long | Format::NoPlural | Format::SmallCaps) -> '1015.02 kilobyte'

    // let size = "10 MiB".parse::<ByteSize>()?;
    // size.value()     -> 83886080 (in bits)
    // size.value() / 8 -> 10485760 (in bytes)
    // size.repr(MEBI_BYTE) -> '10 MiB'
    // size.repr_as(MEBI_BYTE, Format::ForceFraction) -> '10.00 MiB'

    // destructure and create size
    // let (value, unit) = "10 MiB".parse::<(Int, Unit)>().unwrap();
    // (value, unit) -> (80, MEBI_BYTE)
    // let size = ByteSize(value);

    // let (value, unit) = "10 MiB".parse::<(ByteSize, Unit)>().unwrap();
    // (value, unit) -> (ByteSize(80), MEBI_BYTE)

    #[inline]
    pub const fn from_bits(value: Int) -> Self {
        Self(value)
    }

    #[inline]
    pub const fn from_bytes(value: Int) -> Option<Self> {
        if let Some(bits) = value.checked_mul(8) {
            return Some(Self(bits));
        }
        None
    }

    #[inline]
    pub fn value<T: From<Int>>(&self) -> T {
        self.0.into()
    }

    #[inline]
    pub const fn bits(&self) -> Int {
        self.0
    }

    #[inline]
    pub const fn bytes(&self) -> Option<Int> {
        self.0.checked_div(8)
    }

    pub fn to_string(&self, mode: Mode) -> String {
        todo!()
    }

    pub fn to_string_as(&self, mode: Mode, format: Format) -> String {
        todo!()
    }

    pub fn repr(&self, unit: Unit) -> String {
        todo!()
    }

    pub fn repr_as(&self, unit: Unit, format: Format) -> String {
        todo!()
    }
}

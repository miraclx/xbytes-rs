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
        pub struct Format: u16 {
            const Initials           = 0b_0000_0000_0001;
            const Condensed          = 0b_0000_0000_0010;
            const Long               = 0b_0000_0000_0100;
            const NoPlural           = 0b_0000_0000_1000;
            const LowerCaps          = 0b_0000_0001_0000;
            const UpperCaps          = 0b_0000_0010_0000;
            const ForceFraction      = 0b_0000_0100_0000;
            const ThousandsSeparator = 0b_0000_1000_0000;
            const NoSpace            = 0b_0001_0000_0000;
        }
    }
}

pub use flags::*;

#[derive(Eq, Copy, Clone, Debug, PartialEq)]
pub struct ByteSizer(Mode, Format);

impl ByteSizer {
    const MODE: Mode = Mode::empty();
    const FORMAT: Format = Format::empty();

    pub const BINARY: Self = Self::new(); // b, B, KiB, MiB
    pub const DECIMAL: Self = Self::new().with_mode(Mode::Decimal); // b, B, KB, MB

    pub const INITIALS: Self = Self::new().with_format(Format::Initials); // b, B, KB, MB (no binary symbols)
    pub const CONDENSED: Self = Self::new().with_format(Format::Condensed); // b, B, K, M (single chars)
    pub const LONG: Self = Self::new().with_format(Format::Long); // Bits, Bytes, KiloBytes

    #[inline]
    pub const fn new() -> Self {
        Self(Self::MODE, Self::FORMAT)
    }

    #[inline]
    pub const fn with_mode(&self, mode: Mode) -> Self {
        Self(
            Mode::from_bits_truncate(self.0.bits() | mode.bits()),
            self.1,
        )
    }

    #[inline]
    pub const fn with_format(&self, format: Format) -> Self {
        Self(
            self.0,
            Format::from_bits_truncate(self.1.bits() | format.bits()),
        )
    }

    #[inline]
    pub const fn mode(&self) -> Mode {
        self.0
    }

    #[inline]
    pub const fn format(&self) -> Format {
        self.1
    }

    #[inline]
    pub const fn reset_mode(&self) -> Self {
        Self(Self::MODE, self.1)
    }

    #[inline]
    pub const fn reset_format(&self) -> Self {
        Self(self.0, Self::FORMAT)
    }

    #[inline]
    pub fn repr(&self, size: &ByteSize) -> String {
        size.to_string_as(self.0, self.1)
    }
}

impl std::ops::BitOr for ByteSizer {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0, self.1 | rhs.1)
    }
}

#[derive(Eq, Copy, Clone, Debug, PartialEq, PartialOrd)]
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
    #[cfg(feature = "bits")]
    pub const fn from_bits(value: Int) -> Self {
        Self(value)
    }

    #[inline]
    #[cfg(feature = "bits")]
    pub const fn from_bytes(value: Int) -> Option<Self> {
        if let Some(bits) = value.checked_mul(8) {
            return Some(Self(bits));
        }
        None
    }

    #[inline]
    #[cfg(not(feature = "bits"))]
    pub const fn from_bytes(value: Int) -> Self {
        Self(value)
    }

    #[inline]
    #[cfg(not(feature = "bits"))]
    pub const fn from_bits(value: Int) -> Option<Self> {
        if let Some(bytes) = value.checked_div(8) {
            return Some(Self(bytes));
        }
        None
    }

    #[inline]
    #[cfg(feature = "bits")]
    pub const fn bits(&self) -> Int {
        self.0
    }

    #[inline]
    #[cfg(feature = "bits")]
    pub const fn bytes(&self) -> Option<Int> {
        self.0.checked_div(8)
    }

    #[inline]
    #[cfg(not(feature = "bits"))]
    pub const fn bytes(&self) -> Int {
        self.0
    }

    #[inline]
    #[cfg(not(feature = "bits"))]
    pub const fn bits(&self) -> Option<Int> {
        self.0.checked_mul(8)
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

    #[inline]
    pub fn repr_with(&self, sizer: ByteSizer) -> String {
        self.to_string_as(sizer.0, sizer.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bytesizer() {
        let size = ByteSize::from_bytes(1073741824);

        #[cfg(feature = "bits")]
        let size = size.unwrap();

        let sizer = ByteSizer::new();
        assert_eq!("1 GiB", size.repr_with(sizer).as_str());

        let fractional_sizer = sizer.with_format(Format::ForceFraction);
        assert_eq!("1.00 GiB", size.repr_with(fractional_sizer).as_str());

        let decimal_bit_sizer = sizer.with_mode(Mode::Decimal | Mode::Bits);
        assert_eq!("8 Gib", size.repr_with(decimal_bit_sizer).as_str());

        let fractional_decimal_bit_sizer = fractional_sizer | decimal_bit_sizer;
        assert_eq!(
            "8.00 Gib",
            size.repr_with(fractional_decimal_bit_sizer).as_str()
        );
    }
}

use super::{sizes, Int, ParseError, Unit, UnitPrefix};
use std::fmt;

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
            const Initials           = 1 << 0;
            const Condensed          = 1 << 1;
            const Long               = 1 << 2;
            const NoPlural           = 1 << 3;
            const ForcePlural        = 1 << 4;
            const LowerCaps          = 1 << 5;
            const UpperCaps          = 1 << 6;
            const ForceFraction      = 1 << 7;
            const ThousandsSeparator = 1 << 8;
            const NoSpace            = 1 << 9;
        }
    }
}

pub use flags::*;

// thousands separator
// thsep("1234567") -> ['1', '234', '567']
fn thsep(digits: &str) -> impl Iterator<Item = &str> {
    let chars = digits.as_bytes();
    let len = chars.len();
    let tip = len - ((len / 3) * 3);
    std::iter::once(&chars[..tip])
        .chain(chars[tip..].chunks(3))
        .map(|digits| {
            std::str::from_utf8(digits).expect("where did the non-utf8 character come from?")
        })
}

#[derive(Eq, Copy, Clone, Debug, PartialEq)]
pub struct ByteSizeOptions {
    pub mode: Mode,
    pub format: Format,
    pub fixed_prefix: Option<UnitPrefix>,
    pub decimal_places: usize,
}

impl Default for ByteSizeOptions {
    fn default() -> Self {
        Self::BINARY
    }
}

impl ByteSizeOptions {
    const MODE: Mode = Mode::empty();
    const FORMAT: Format = Format::empty();

    pub const BINARY: Self = Self::default(); // b, B, KiB, MiB
    pub const DECIMAL: Self = Self::default().with_mode(Mode::Decimal); // b, B, KB, MB

    pub const INITIALS: Self = Self::default().with_format(Format::Initials); // b, B, KB, MB (no binary symbols)
    pub const CONDENSED: Self = Self::default().with_format(Format::Condensed); // b, B, K, M (single chars)
    pub const LONG: Self = Self::default().with_format(Format::Long); // Bits, Bytes, KiloBytes
    pub const NOSPACE: Self = Self::default().with_format(Format::NoSpace); // 10b, 10B, 10MB

    #[inline]
    const fn default() -> Self {
        Self {
            mode: Self::MODE,
            format: Self::FORMAT,
            fixed_prefix: None,
            decimal_places: 2,
        }
    }

    #[inline]
    pub const fn with_mode(&self, mode: Mode) -> Self {
        let mut new = *self;
        new.mode = bitflags_const_or!(Mode::{new.mode, mode});
        new
    }

    #[inline]
    pub const fn with_format(&self, format: Format) -> Self {
        let mut new = *self;
        new.format = bitflags_const_or!(Format::{new.format, format});
        new
    }

    #[inline]
    pub const fn reset_mode(&self) -> Self {
        let mut new = *self;
        new.mode = Self::MODE;
        new
    }

    #[inline]
    pub const fn reset_format(&self) -> Self {
        let mut new = *self;
        new.format = Self::FORMAT;
        new
    }

    pub fn repr(&self, value: f64, unit: Unit) -> (String, String) {
        let mut value_part = if self.format.contains(Format::ForceFraction) || value.fract() != 0.0
        {
            format!("{:.fixed$}", value, fixed = self.decimal_places)
        } else {
            format!("{}", value)
        };

        if self.format.contains(Format::ThousandsSeparator) {
            let (whole, fract) = value_part
                .find('.')
                .map_or((&value_part[..], ""), |index| value_part.split_at(index));
            value_part = format!("{}{}", thsep(whole).join(","), fract);
        }

        #[rustfmt::skip]
        let unit_part = format!(
            "{}{}",
            if !self.format.contains(Format::NoSpace) { " " } else { "" },
            unit
        );

        (value_part, unit_part)
    }
}

impl std::ops::BitOr for ByteSizeOptions {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            mode: self.mode | rhs.mode,
            format: self.format | rhs.format,
            ..self
        }
    }
}

macro_rules! ok_or {
    ($value:expr, $err:expr) => {
        match ($value, $err) {
            (Some(value), _) => Ok(value),
            (_, err) => Err(err),
        }
    };
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
    pub const fn from_bytes(value: Int) -> Result<Self, ParseError> {
        match ok_or!(value.checked_mul(8), ParseError::ValueOverflow) {
            Ok(value) => Ok(Self(value)),
            Err(err) => Err(err),
        }
    }

    #[inline]
    #[cfg(not(feature = "bits"))]
    pub const fn from_bytes(value: Int) -> Self {
        Self(value)
    }

    #[inline]
    #[cfg(not(feature = "bits"))]
    pub const fn from_bits(value: Int) -> Result<Self, ParseError> {
        match ok_or!(value.checked_div(8), ParseError::ValueOverflow) {
            Ok(value) => Ok(Self(value)),
            Err(err) => Err(err),
        }
    }

    #[inline]
    #[cfg(feature = "bits")]
    pub const fn bits(&self) -> Int {
        self.0
    }

    #[inline]
    #[cfg(feature = "bits")]
    pub const fn bytes(&self) -> Result<Int, ParseError> {
        ok_or!(self.0.checked_div(8), ParseError::ValueOverflow)
    }

    #[inline]
    #[cfg(not(feature = "bits"))]
    pub const fn bytes(&self) -> Int {
        self.0
    }

    #[inline]
    #[cfg(not(feature = "bits"))]
    pub const fn bits(&self) -> Result<Int, ParseError> {
        ok_or!(self.0.checked_mul(8), ParseError::ValueOverflow)
    }

    pub fn repr(&self, unit: Unit) -> String {
        self.repr_as(unit, ByteSizeOptions::FORMAT)
    }

    #[rustfmt::skip]
    fn prep_value(&self, mode: Mode) -> f64 {
        let value = self.0 as f64;
        let wants_bits = mode.contains(Mode::Bits);
        if {
            #[cfg(feature = "bits")] {!wants_bits}
            #[cfg(not(feature = "bits"))] {wants_bits}
        } {
            #[cfg(feature = "bits")] {value / 8.0}
            #[cfg(not(feature = "bits"))] {value * 8.0}
        } else { value }
    }

    pub fn repr_as(&self, unit: Unit, format: Format) -> String {
        let value = self.prep_value(unit.mode());
        let sizer = ByteSizeOptions::BINARY.with_format(format);
        let (value, postfix) = sizer.repr(value / unit.effective_value() as f64, unit);
        format!("{} {}", value, postfix)
    }

    #[rustfmt::skip]
    pub fn repr_with_components(&self, mode: Mode) -> (f64, Unit) {
        let mut value = self.prep_value(mode);
        let as_bits = mode.contains(Mode::Bits);
        let as_decimal = mode.contains(Mode::Decimal);
        let divisor = if as_decimal { 1000f64 } else { 1024f64 };
        let mut prefix_index = 0;
        while value >= divisor {
            value /= divisor;
            prefix_index += 2;
        }
        if prefix_index > 0 && as_decimal { prefix_index -= 1 }
        let unit = (if as_bits { sizes::BITS } else { sizes::BYTES })[prefix_index];
        (value, unit)
    }

    pub fn repr_with(&self, sizer: ByteSizeOptions) -> String {
        let (value, unit) = self.repr_with_components(sizer.mode);
        let (value, postfix) = sizer.repr(value, unit);
        format!("{} {}", value, postfix)
    }
}

impl fmt::Display for ByteSize {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut sizer = ByteSizeOptions::BINARY;
        if f.sign_plus() {
            let mut format = Format::Long;
            if f.alternate() {
                format |= Format::NoPlural;
            }
            sizer = sizer.with_format(format)
        } else if f.sign_minus() {
            let format = if f.alternate() {
                Format::Condensed
            } else {
                Format::Initials
            };
            sizer = sizer.with_format(format);
        }
        if let Some(precision) = f.precision() {
            sizer.decimal_places = precision;
        }
        write!(f, "{}", self.repr_with(sizer))
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

        let sizer = ByteSizeOptions::BINARY;
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

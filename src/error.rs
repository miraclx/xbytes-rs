use std::fmt;

#[derive(Eq, Clone, Debug, PartialEq)]
pub struct ParseError {
    pub(crate) kind: ParseErrorKind,
}

impl ParseError {
    pub fn kind(&self) -> &ParseErrorKind {
        &self.kind
    }
}

impl std::error::Error for ParseError {}
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.kind.fmt(f)
    }
}

#[derive(Eq, Clone, Debug, PartialEq)]
pub enum ParseErrorKind {
    EmptyInput,
    MissingUnit,
    InvalidValue,
    MissingValue,
    InvalidPrefix,
    ValueOverflow,
    InvalidSizeVariant,
    InvalidThousandsFormat,
    #[cfg(not(feature = "case-insensitive"))]
    InvalidUnitCaseFormat,
    #[cfg(not(feature = "case-insensitive"))]
    InvalidPrefixCaseFormat,
}

impl fmt::Display for ParseErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad(match self {
            ParseErrorKind::EmptyInput => "empty input",
            ParseErrorKind::MissingUnit => "missing unit",
            ParseErrorKind::InvalidValue => "invalid value",
            ParseErrorKind::MissingValue => "missing value",
            ParseErrorKind::InvalidPrefix => "invalid prefix",
            ParseErrorKind::InvalidSizeVariant => "invalid size variant",
            ParseErrorKind::InvalidThousandsFormat => "invalid thousands format",
            ParseErrorKind::ValueOverflow => "value overflow",
            #[cfg(not(feature = "case-insensitive"))]
            ParseErrorKind::InvalidUnitCaseFormat => {
                "invalid case: expected format like 'kB', 'Kb', 'KiB', 'Mb', 'MiB'"
            }
            #[cfg(not(feature = "case-insensitive"))]
            ParseErrorKind::InvalidPrefixCaseFormat => {
                "invalid case: expected format like 'k', 'K', 'Ki', 'M', 'Mi'"
            }
        })
    }
}

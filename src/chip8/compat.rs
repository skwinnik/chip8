use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub enum Compatibility {
    /// COSMAC
    Cosmac,
    /// Chip48 or SuperChip
    Chip48,
}

impl Display for Compatibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

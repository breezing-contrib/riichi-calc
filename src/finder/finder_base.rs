use crate::constants::field::Field;
use crate::constants::hand::WinningHand;
use crate::constants::status::Status;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct YakuEntry {
    pub name: String,
    pub value: u8,
}

impl YakuEntry {
    pub fn new(name: impl Into<String>, value: u8) -> Self {
        Self {
            name: name.into(),
            value,
        }
    }
}

pub type YakuValidator = fn(&Field, &WinningHand, &Status) -> Option<YakuEntry>;

pub trait YakuBase {
    /// check if certain yaku in valid
    ///
    /// returns yaku name han count if valid
    fn validate(field: &Field, hand: &WinningHand, status: &Status) -> Option<YakuEntry>;
}

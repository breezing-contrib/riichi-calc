use crate::constants::field::Field;
use crate::constants::hand::WinningHand;
use crate::constants::status::Status;
use crate::finder::finder_base::{YakuBase, YakuEntry, YakuValidator};
use crate::finder::roku_han::chinitu::Chinitu;

mod chinitu;

pub fn roku_han_yaku(field: &Field, winning_hand: &WinningHand, status: &Status) -> Vec<YakuEntry> {
    let validators: Vec<YakuValidator> = vec![Chinitu::validate];

    validators
        .iter()
        .filter_map(|validator| validator(field, winning_hand, status))
        .collect()
}

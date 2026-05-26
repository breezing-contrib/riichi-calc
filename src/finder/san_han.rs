use crate::constants::field::Field;
use crate::constants::hand::WinningHand;
use crate::constants::status::Status;
use crate::finder::finder_base::{YakuBase, YakuValidator};
use crate::finder::yaku::YakuEntry;

mod honitu;
mod junchan;
mod riyan_peco;

pub fn san_han_yaku(field: &Field, winning_hand: &WinningHand, status: &Status) -> Vec<YakuEntry> {
    let validators: Vec<YakuValidator> = vec![
        riyan_peco::RiyanPeco::validate,
        junchan::Junchan::validate,
        honitu::Honitu::validate,
    ];

    validators
        .iter()
        .filter_map(|validator| validator(field, winning_hand, status))
        .collect()
}

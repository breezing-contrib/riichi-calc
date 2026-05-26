use crate::constants::field::Field;
use crate::constants::hand::WinningHand;
use crate::constants::status::RiichiStatus::DoubleRiichi as DoubleRiichiStatus;
use crate::constants::status::Status;
use crate::finder::finder_base::{YakuBase, YakuEntry};

pub struct DoubleRiichi;

impl YakuBase for DoubleRiichi {
    fn validate(_: &Field, _: &WinningHand, status: &Status) -> Option<YakuEntry> {
        match status.riichi {
            DoubleRiichiStatus(_) => Some(YakuEntry::new("ダブルリーチ", 2)),
            _ => None,
        }
    }
}

#[cfg(test)]
mod valid {
    use crate::constants::status::RiichiStatus;
    use crate::finder::finder_base::{YakuBase, YakuEntry};
    use crate::finder::ryan_han::double_riichi::DoubleRiichi;
    use crate::finder::test_utils::{
        from_hand, random_field, random_janto, random_mentsu, random_status, random_tile,
    };

    #[test]
    fn valid_double_riichi() {
        let field = random_field();
        let hand = [
            random_mentsu(true, false),
            random_mentsu(true, false),
            random_mentsu(true, false),
            random_mentsu(true, false),
            random_janto(false),
        ];
        let winning_hand = from_hand(hand);
        let mut status = random_status();
        status.riichi = RiichiStatus::DoubleRiichi(vec![random_tile()]);
        assert_eq!(
            DoubleRiichi::validate(&field, &winning_hand, &status),
            Some(YakuEntry::new("ダブルリーチ", 2)),
            "{:?}",
            hand
        );
    }
}

#[cfg(test)]
mod invalid {
    use crate::constants::status::RiichiStatus;
    use crate::finder::finder_base::YakuBase;
    use crate::finder::ryan_han::double_riichi::DoubleRiichi;
    use crate::finder::test_utils::{
        from_hand, random_field, random_janto, random_mentsu, random_status, random_tile,
    };
    use rand::random;

    #[test]
    fn invalid_double_riichi() {
        let field = random_field();
        let hand = [
            random_mentsu(true, false),
            random_mentsu(true, false),
            random_mentsu(true, false),
            random_mentsu(true, false),
            random_janto(false),
        ];
        let winning_hand = from_hand(hand);
        let mut status = random_status();
        status.riichi = match random::<u8>() % 2 {
            0 => RiichiStatus::NoRiichi,
            1 => RiichiStatus::Riichi(vec![random_tile(), random_tile()]),
            _ => unreachable!(),
        };
        assert_eq!(
            DoubleRiichi::validate(&field, &winning_hand, &status),
            None,
            "{:?}",
            hand
        );
    }
}

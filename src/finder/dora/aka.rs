use crate::constants::field::Field;
use crate::constants::hand::WinningHand;
use crate::constants::status::Status;
use crate::finder::finder_base::YakuBase;
use crate::finder::yaku::{YakuEntry, YakuKind};

pub struct Aka;

impl YakuBase for Aka {
    fn validate(_: &Field, hand: &WinningHand, _: &Status) -> Option<YakuEntry> {
        if hand.red_tile > 0 {
            Some(YakuEntry::new(YakuKind::AkaDora, hand.red_tile))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod count {
    use crate::constants::hand::WinningHand;
    use crate::finder::dora::aka::Aka;
    use crate::finder::finder_base::YakuBase;
    use crate::finder::test_utils::{
        random_field, random_janto, random_mentsu, random_status, random_tile,
    };
    use crate::finder::yaku::{YakuEntry, YakuKind};

    #[test]
    fn one() {
        let hand = [
            random_mentsu(true, false),
            random_mentsu(true, false),
            random_mentsu(true, false),
            random_mentsu(true, false),
            random_janto(false),
        ];
        let winning_hand = WinningHand {
            hand,
            winning_tile: random_tile(),
            red_tile: 1,
        };

        assert_eq!(
            Aka::validate(&random_field(), &winning_hand, &random_status()),
            Some(YakuEntry::new(YakuKind::AkaDora, 1))
        );
    }

    #[test]
    fn should_not_print_in_zero() {
        let hand = [
            random_mentsu(true, false),
            random_mentsu(true, false),
            random_mentsu(true, false),
            random_mentsu(true, false),
            random_janto(false),
        ];
        let winning_hand = WinningHand {
            hand,
            winning_tile: random_tile(),
            red_tile: 0,
        };

        assert_eq!(
            Aka::validate(&random_field(), &winning_hand, &random_status()),
            None
        );
    }
}

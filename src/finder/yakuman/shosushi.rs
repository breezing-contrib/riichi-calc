use crate::constants::field::Field;
use crate::constants::hand::{Mentsu, WinningHand};
use crate::constants::status::Status;
use crate::constants::tiles::{Tile, TileType};
use crate::finder::finder_base::YakuBase;
use crate::finder::yaku::{YakuEntry, YakuKind};

pub struct Shosushi;

impl YakuBase for Shosushi {
    fn validate(_: &Field, hand: &WinningHand, _: &Status) -> Option<YakuEntry> {
        let mut wind_count = 0;
        let mut janto: Option<Tile> = None;

        for mentsu in hand.hand {
            match mentsu {
                Mentsu::Shuntsu(_, _) => continue,
                Mentsu::Janto(tile) => {
                    if tile.tile_type == TileType::Wind {
                        janto = Some(tile);
                        wind_count += 1;
                    } else {
                        continue;
                    }
                }
                Mentsu::Koutsu(tile, _) | Mentsu::Kantsu(tile, _) => {
                    if tile.tile_type == TileType::Wind {
                        wind_count += 1;
                    }
                }
            }
        }

        if wind_count != 4 {
            return None;
        }

        if janto.is_some() {
            Some(YakuEntry::new(YakuKind::Shosushi, 1))
        } else {
            Some(YakuEntry::new(YakuKind::Daisushi, 2))
        }
    }
}

#[cfg(test)]
mod valid {
    use crate::constants::hand::Mentsu;
    use crate::constants::tiles::{Tile, TileType};
    use crate::finder::finder_base::YakuBase;
    use crate::finder::test_utils::{
        from_hand, random_field, random_janto, random_mentsu, random_status,
    };
    use crate::finder::yaku::{YakuEntry, YakuKind};
    use crate::finder::yakuman::shosushi::Shosushi;
    use rand::random;

    #[test]
    fn valid_shosushi() {
        let hand = [
            Mentsu::Koutsu(
                Tile {
                    tile_type: TileType::Wind,
                    number: 1,
                },
                random(),
            ),
            Mentsu::Koutsu(
                Tile {
                    tile_type: TileType::Wind,
                    number: 2,
                },
                random(),
            ),
            Mentsu::Koutsu(
                Tile {
                    tile_type: TileType::Wind,
                    number: 3,
                },
                random(),
            ),
            random_mentsu(true, true),
            Mentsu::Janto(Tile {
                tile_type: TileType::Wind,
                number: 4,
            }),
        ];

        assert_eq!(
            Shosushi::validate(&random_field(), &from_hand(hand), &random_status()),
            Some(YakuEntry::new(YakuKind::Shosushi, 1)),
            "{:?}",
            hand
        );
    }

    #[test]
    fn valid_daisushi() {
        let hand = [
            Mentsu::Koutsu(
                Tile {
                    tile_type: TileType::Wind,
                    number: 1,
                },
                random(),
            ),
            Mentsu::Koutsu(
                Tile {
                    tile_type: TileType::Wind,
                    number: 2,
                },
                random(),
            ),
            Mentsu::Koutsu(
                Tile {
                    tile_type: TileType::Wind,
                    number: 3,
                },
                random(),
            ),
            Mentsu::Koutsu(
                Tile {
                    tile_type: TileType::Wind,
                    number: 4,
                },
                random(),
            ),
            random_janto(true),
        ];

        assert_eq!(
            Shosushi::validate(&random_field(), &from_hand(hand), &random_status()),
            Some(YakuEntry::new(YakuKind::Daisushi, 2)),
            "{:?}",
            hand
        );
    }
}

#[cfg(test)]
mod invalid {
    use crate::constants::hand::Mentsu;
    use crate::constants::tiles::{Tile, TileType};
    use crate::finder::finder_base::YakuBase;
    use crate::finder::test_utils::{from_hand, random_field, random_status};
    use crate::finder::yakuman::shosushi::Shosushi;
    use rand::random;

    #[test]
    fn tuiso_not_shosushi() {
        let hand = [
            Mentsu::Koutsu(
                Tile {
                    tile_type: TileType::Wind,
                    number: 1,
                },
                random(),
            ),
            Mentsu::Koutsu(
                Tile {
                    tile_type: TileType::Wind,
                    number: 2,
                },
                random(),
            ),
            Mentsu::Koutsu(
                Tile {
                    tile_type: TileType::Wind,
                    number: 3,
                },
                random(),
            ),
            Mentsu::Koutsu(
                Tile {
                    tile_type: TileType::Dragon,
                    number: 2,
                },
                random(),
            ),
            Mentsu::Janto(Tile {
                tile_type: TileType::Dragon,
                number: 1,
            }),
        ];

        assert_eq!(
            Shosushi::validate(&random_field(), &from_hand(hand), &random_status()),
            None,
            "{:?}",
            hand
        );
    }
}

use std::fmt;
use std::str::FromStr;

///
/// # Types
///
/// - [Manzu](TileType::Manzu)
/// - [Pinzu](TileType::Pinzu)
/// - [Souzu](TileType::Souzu)
/// - [Wind](TileType::Wind)
/// - [Dragon](TileType::Dragon)
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TileType {
    /// 萬子
    Manzu,
    /// 筒子
    Pinzu,
    /// 索子
    Souzu,
    /// 風牌
    Wind,
    /// 三元牌
    Dragon,
}

impl fmt::Display for TileType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TileType::Manzu => write!(f, "Manzu"),
            TileType::Pinzu => write!(f, "Pinzu"),
            TileType::Souzu => write!(f, "Souzu"),
            TileType::Wind => write!(f, "Wind"),
            TileType::Dragon => write!(f, "Dragon"),
        }
    }
}

/// represents a single tile
///
/// # Fields
/// - `number`: the number of tiles [number]
/// - `tile_type`: types in the enum [tile_type]
///
/// [number]: #structfield.number
/// [tile_type]: #structfield.tile_type
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Tile {
    /// - 1~10 for number tile types 10 is used when it is a red tile
    /// - 1~4 for wind tile type `order`: (東, 南, 西, 北)
    /// - 1~3 for dragon tile type `order`: (白, 發, 中)
    pub number: u8,
    /// types in the enum [TileType]
    pub tile_type: TileType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseTileError {
    InvalidFormat,
    InvalidNumber,
    InvalidTileType,
}

impl Tile {
    pub fn normalize_red(self) -> Tile {
        match self.tile_type {
            TileType::Manzu | TileType::Pinzu | TileType::Souzu if self.number == 10 => Tile {
                number: 5,
                tile_type: self.tile_type,
            },
            _ => self,
        }
    }
}

impl FromStr for Tile {
    type Err = ParseTileError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 2 {
            return Err(ParseTileError::InvalidFormat);
        }

        let mut chars = s.chars();
        let number = chars
            .next()
            .and_then(|c| c.to_digit(10))
            .ok_or(ParseTileError::InvalidNumber)? as u8;
        let suffix = chars.next().ok_or(ParseTileError::InvalidFormat)?;

        match suffix {
            'm' if number <= 9 => Ok(Tile {
                number: parse_suhai_number(number),
                tile_type: TileType::Manzu,
            }),
            'p' if number <= 9 => Ok(Tile {
                number: parse_suhai_number(number),
                tile_type: TileType::Pinzu,
            }),
            's' if number <= 9 => Ok(Tile {
                number: parse_suhai_number(number),
                tile_type: TileType::Souzu,
            }),
            'z' if number == 0 => Err(ParseTileError::InvalidNumber),
            'z' if number <= 4 => Ok(Tile {
                number,
                tile_type: TileType::Wind,
            }),
            'z' if number <= 7 => Ok(Tile {
                number: number - 4,
                tile_type: TileType::Dragon,
            }),
            'm' | 'p' | 's' | 'z' => Err(ParseTileError::InvalidNumber),
            _ => Err(ParseTileError::InvalidTileType),
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.tile_type {
            TileType::Manzu => format_suhai_code(f, self.number, 'm'),
            TileType::Pinzu => format_suhai_code(f, self.number, 'p'),
            TileType::Souzu => format_suhai_code(f, self.number, 's'),
            TileType::Wind => match self.number {
                1..=4 => write!(f, "{}z", self.number),
                _ => panic!("invalid wind tile number: {}", self.number),
            },
            TileType::Dragon => match self.number {
                1..=3 => write!(f, "{}z", self.number + 4),
                _ => panic!("invalid dragon tile number: {}", self.number),
            },
        }
    }
}

fn format_suhai_code(f: &mut fmt::Formatter, number: u8, suffix: char) -> fmt::Result {
    match number {
        10 => write!(f, "0{}", suffix),
        1..=9 => write!(f, "{}{}", number, suffix),
        _ => panic!("invalid number tile number: {}", number),
    }
}

fn parse_suhai_number(number: u8) -> u8 {
    if number == 0 {
        10
    } else {
        number
    }
}

#[cfg(test)]
mod tests {
    use super::{ParseTileError, Tile, TileType};

    #[test]
    fn parses_suhai_codes() {
        assert_eq!(
            "1m".parse::<Tile>(),
            Ok(Tile {
                number: 1,
                tile_type: TileType::Manzu,
            })
        );
        assert_eq!(
            "9p".parse::<Tile>(),
            Ok(Tile {
                number: 9,
                tile_type: TileType::Pinzu,
            })
        );
        assert_eq!(
            "1s".parse::<Tile>(),
            Ok(Tile {
                number: 1,
                tile_type: TileType::Souzu,
            })
        );
        assert_eq!(
            "0m".parse::<Tile>(),
            Ok(Tile {
                number: 10,
                tile_type: TileType::Manzu,
            })
        );
        assert_eq!(
            "0p".parse::<Tile>(),
            Ok(Tile {
                number: 10,
                tile_type: TileType::Pinzu,
            })
        );
        assert_eq!(
            "0s".parse::<Tile>(),
            Ok(Tile {
                number: 10,
                tile_type: TileType::Souzu,
            })
        );
    }

    #[test]
    fn converts_red_suhai_to_normal_five() {
        assert_eq!(
            Tile {
                number: 10,
                tile_type: TileType::Manzu,
            }
            .normalize_red(),
            Tile {
                number: 5,
                tile_type: TileType::Manzu,
            }
        );
        assert_eq!(
            Tile {
                number: 10,
                tile_type: TileType::Pinzu,
            }
            .normalize_red(),
            Tile {
                number: 5,
                tile_type: TileType::Pinzu,
            }
        );
        assert_eq!(
            Tile {
                number: 10,
                tile_type: TileType::Souzu,
            }
            .normalize_red(),
            Tile {
                number: 5,
                tile_type: TileType::Souzu,
            }
        );
    }

    #[test]
    fn leaves_normal_tiles_unchanged_when_normalizing_red() {
        let tile = Tile {
            number: 5,
            tile_type: TileType::Manzu,
        };

        assert_eq!(tile.normalize_red(), tile);
    }

    #[test]
    fn parses_honor_codes() {
        for number in 1..=4 {
            assert_eq!(
                format!("{}z", number).parse::<Tile>(),
                Ok(Tile {
                    number,
                    tile_type: TileType::Wind,
                })
            );
        }

        for number in 5..=7 {
            assert_eq!(
                format!("{}z", number).parse::<Tile>(),
                Ok(Tile {
                    number: number - 4,
                    tile_type: TileType::Dragon,
                })
            );
        }
    }

    #[test]
    fn formats_suhai_codes() {
        assert_eq!(
            Tile {
                number: 1,
                tile_type: TileType::Manzu,
            }
            .to_string(),
            "1m"
        );
        assert_eq!(
            Tile {
                number: 9,
                tile_type: TileType::Pinzu,
            }
            .to_string(),
            "9p"
        );
        assert_eq!(
            Tile {
                number: 1,
                tile_type: TileType::Souzu,
            }
            .to_string(),
            "1s"
        );
        assert_eq!(
            Tile {
                number: 10,
                tile_type: TileType::Manzu,
            }
            .to_string(),
            "0m"
        );
        assert_eq!(
            Tile {
                number: 10,
                tile_type: TileType::Pinzu,
            }
            .to_string(),
            "0p"
        );
        assert_eq!(
            Tile {
                number: 10,
                tile_type: TileType::Souzu,
            }
            .to_string(),
            "0s"
        );
    }

    #[test]
    fn formats_honor_codes() {
        for number in 1..=4 {
            assert_eq!(
                Tile {
                    number,
                    tile_type: TileType::Wind,
                }
                .to_string(),
                format!("{}z", number)
            );
        }

        for number in 1..=3 {
            assert_eq!(
                Tile {
                    number,
                    tile_type: TileType::Dragon,
                }
                .to_string(),
                format!("{}z", number + 4)
            );
        }
    }

    #[test]
    fn rejects_invalid_codes() {
        assert_eq!("".parse::<Tile>(), Err(ParseTileError::InvalidFormat));
        assert_eq!("11m".parse::<Tile>(), Err(ParseTileError::InvalidFormat));
        assert_eq!("xm".parse::<Tile>(), Err(ParseTileError::InvalidNumber));
        assert_eq!("0z".parse::<Tile>(), Err(ParseTileError::InvalidNumber));
        assert_eq!("8z".parse::<Tile>(), Err(ParseTileError::InvalidNumber));
        assert_eq!("1x".parse::<Tile>(), Err(ParseTileError::InvalidTileType));
    }

    #[test]
    fn display_uses_short_code() {
        assert_eq!(
            Tile {
                number: 1,
                tile_type: TileType::Souzu,
            }
            .to_string(),
            "1s"
        );
    }

    #[test]
    #[should_panic(expected = "invalid number tile number")]
    fn display_rejects_invalid_suhai() {
        Tile {
            number: 11,
            tile_type: TileType::Manzu,
        }
        .to_string();
    }
}

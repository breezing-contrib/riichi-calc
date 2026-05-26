use crate::finder::yaku::YakuEntry;

/// represents which yaku was found
#[derive(Debug)]
pub enum FoundResult {
    FoundYaku(FoundYaku),
    FoundYakuman(FoundYakuman),
}

#[derive(Debug)]
pub struct FoundYaku {
    pub dora: Vec<YakuEntry>,
    pub ii_han: Vec<YakuEntry>,
    pub ryan_han: Vec<YakuEntry>,
    pub san_han: Vec<YakuEntry>,
    pub roku_han: Vec<YakuEntry>,
}

#[derive(Debug)]
pub struct FoundYakuman {
    pub yakuman: Vec<YakuEntry>,
}

impl FoundResult {
    pub fn count_yaku(&self) -> u8 {
        match self {
            FoundResult::FoundYaku(yaku) => {
                yaku.dora.iter().map(|entry| entry.value).sum::<u8>()
                    + yaku.ii_han.iter().map(|entry| entry.value).sum::<u8>()
                    + yaku.ryan_han.iter().map(|entry| entry.value).sum::<u8>()
                    + yaku.san_han.iter().map(|entry| entry.value).sum::<u8>()
                    + yaku.roku_han.iter().map(|entry| entry.value).sum::<u8>()
            }
            FoundResult::FoundYakuman(yakuaman) => {
                yakuaman.yakuman.iter().map(|entry| entry.value).sum()
            }
        }
    }

    pub fn is_valid_hora(&self) -> bool {
        match self {
            FoundResult::FoundYaku(yaku) => {
                yaku.ii_han.len() > 0
                    || yaku.ryan_han.len() > 0
                    || yaku.san_han.len() > 0
                    || yaku.roku_han.len() > 0
            }
            FoundResult::FoundYakuman(yakuaman) => yakuaman.yakuman.len() > 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::finder::result::{FoundResult, FoundYakuman};
    use crate::finder::yaku::{YakuEntry, YakuKind};

    #[test]
    fn count_yaku_uses_yakuman_value() {
        let result = FoundResult::FoundYakuman(FoundYakuman {
            yakuman: vec![YakuEntry::new(YakuKind::SuankoTanki, 2)],
        });

        assert_eq!(result.count_yaku(), 2);
    }
}

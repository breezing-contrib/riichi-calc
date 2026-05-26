#[cfg(feature = "zh-cn")]
macro_rules! yaku_name {
    ($ja:literal, $zh_cn:literal) => {
        $zh_cn
    };
}

#[cfg(not(feature = "zh-cn"))]
macro_rules! yaku_name {
    ($ja:literal, $zh_cn:literal) => {
        $ja
    };
}

macro_rules! define_yaku_kinds {
    ($($kind:ident => { ja: $ja:literal, zh_cn: $zh_cn:literal }),+ $(,)?) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum YakuKind {
            $($kind),+
        }

        impl YakuKind {
            pub fn name(&self) -> &'static str {
                match self {
                    $(YakuKind::$kind => yaku_name!($ja, $zh_cn)),+
                }
            }
        }
    };
}

define_yaku_kinds! {
    Dora => { ja: "ドラ", zh_cn: "宝牌" },
    AkaDora => { ja: "赤ドラ", zh_cn: "赤宝牌" },
    UraDora => { ja: "裏ドラ", zh_cn: "里宝牌" },
    Riichi => { ja: "立直", zh_cn: "立直" },
    Tanyao => { ja: "断么九", zh_cn: "断幺九" },
    MenzenTsumo => { ja: "門前清自摸和", zh_cn: "门前清自摸和" },
    Zikaze => { ja: "役牌:自風牌", zh_cn: "役牌:自风牌" },
    Bakaze => { ja: "役牌:場風牌", zh_cn: "役牌:场风牌" },
    Haku => { ja: "役牌:白", zh_cn: "役牌:白" },
    Hatu => { ja: "役牌:發", zh_cn: "役牌:发" },
    Chun => { ja: "役牌:中", zh_cn: "役牌:中" },
    Pinfu => { ja: "平和", zh_cn: "平和" },
    Iipeco => { ja: "一盃口", zh_cn: "一杯口" },
    Chankan => { ja: "搶槓", zh_cn: "抢杠" },
    Rinshan => { ja: "嶺上開花", zh_cn: "岭上开花" },
    Haitei => { ja: "海底自摸", zh_cn: "海底摸月" },
    Hotei => { ja: "河底撈魚", zh_cn: "河底捞鱼" },
    Ipatu => { ja: "一発", zh_cn: "一发" },
    DoubleRiichi => { ja: "ダブルリーチ", zh_cn: "两立直" },
    SanshokuDoko => { ja: "三色同刻", zh_cn: "三色同刻" },
    SanshokuDojun => { ja: "三色同順", zh_cn: "三色同顺" },
    Ixtukitukan => { ja: "一気通貫", zh_cn: "一气通贯" },
    ToiToi => { ja: "対対和", zh_cn: "对对和" },
    Sananko => { ja: "三暗刻", zh_cn: "三暗刻" },
    Sankantu => { ja: "三槓子", zh_cn: "三杠子" },
    Shosangen => { ja: "小三元", zh_cn: "小三元" },
    Honroto => { ja: "混老頭", zh_cn: "混老头" },
    Chanta => { ja: "混全帯么九", zh_cn: "混全带幺九" },
    RiyanPeco => { ja: "二盃口", zh_cn: "二杯口" },
    Junchan => { ja: "純全帯么九", zh_cn: "纯全带幺九" },
    Honitu => { ja: "混一色", zh_cn: "混一色" },
    Chinitu => { ja: "清一色", zh_cn: "清一色" },
    Tenho => { ja: "天和", zh_cn: "天和" },
    Chiho => { ja: "地和", zh_cn: "地和" },
    Daisangen => { ja: "大三元", zh_cn: "大三元" },
    Suanko => { ja: "四暗刻", zh_cn: "四暗刻" },
    SuankoTanki => { ja: "四暗刻単騎", zh_cn: "四暗刻单骑" },
    Tuiso => { ja: "字一色", zh_cn: "字一色" },
    Ryuiso => { ja: "緑一色", zh_cn: "绿一色" },
    Chinroto => { ja: "清老頭", zh_cn: "清老头" },
    Shosushi => { ja: "小四喜", zh_cn: "小四喜" },
    Daisushi => { ja: "大四喜", zh_cn: "大四喜" },
    Sukantu => { ja: "四槓子", zh_cn: "四杠子" },
    Churen => { ja: "九蓮宝燈", zh_cn: "九莲宝灯" },
    JunseiChuren => { ja: "純正九蓮宝燈", zh_cn: "纯正九莲宝灯" },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct YakuEntry {
    pub kind: YakuKind,
    pub value: u8,
}

impl YakuEntry {
    pub fn new(kind: YakuKind, value: u8) -> Self {
        Self { kind, value }
    }

    pub fn name(&self) -> &'static str {
        self.kind.name()
    }
}

#[cfg(test)]
mod tests {
    use crate::finder::yaku::{YakuEntry, YakuKind};

    #[cfg(not(feature = "zh-cn"))]
    #[test]
    fn default_names_are_japanese() {
        assert_eq!(YakuKind::Riichi.name(), "立直");
        assert_eq!(YakuKind::Dora.name(), "ドラ");
        assert_eq!(YakuEntry::new(YakuKind::Dora, 1).name(), "ドラ");
    }

    #[cfg(feature = "zh-cn")]
    #[test]
    fn zh_cn_names_are_simplified_chinese() {
        assert_eq!(YakuKind::Riichi.name(), "立直");
        assert_eq!(YakuKind::Dora.name(), "宝牌");
        assert_eq!(YakuKind::SuankoTanki.name(), "四暗刻单骑");
        assert_eq!(YakuEntry::new(YakuKind::Dora, 1).name(), "宝牌");
    }
}

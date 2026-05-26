#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum YakuKind {
    Dora,
    AkaDora,
    UraDora,
    Riichi,
    Tanyao,
    MenzenTsumo,
    Zikaze,
    Bakaze,
    Haku,
    Hatu,
    Chun,
    Pinfu,
    Iipeco,
    Chankan,
    Rinshan,
    Haitei,
    Hotei,
    Ipatu,
    DoubleRiichi,
    SanshokuDoko,
    SanshokuDojun,
    Ixtukitukan,
    ToiToi,
    Sananko,
    Sankantu,
    Shosangen,
    Honroto,
    Chanta,
    RiyanPeco,
    Junchan,
    Honitu,
    Chinitu,
    Tenho,
    Chiho,
    Daisangen,
    Suanko,
    SuankoTanki,
    Tuiso,
    Ryuiso,
    Chinroto,
    Shosushi,
    Daisushi,
    Sukantu,
    Churen,
    JunseiChuren,
}

impl YakuKind {
    pub fn name(&self) -> &'static str {
        match self {
            YakuKind::Dora => localized_name("ドラ", "宝牌"),
            YakuKind::AkaDora => localized_name("赤ドラ", "赤宝牌"),
            YakuKind::UraDora => localized_name("裏ドラ", "里宝牌"),
            YakuKind::Riichi => localized_name("立直", "立直"),
            YakuKind::Tanyao => localized_name("断么九", "断幺九"),
            YakuKind::MenzenTsumo => localized_name("門前清自摸和", "门前清自摸和"),
            YakuKind::Zikaze => localized_name("役牌:自風牌", "役牌:自风牌"),
            YakuKind::Bakaze => localized_name("役牌:場風牌", "役牌:场风牌"),
            YakuKind::Haku => localized_name("役牌:白", "役牌:白"),
            YakuKind::Hatu => localized_name("役牌:發", "役牌:发"),
            YakuKind::Chun => localized_name("役牌:中", "役牌:中"),
            YakuKind::Pinfu => localized_name("平和", "平和"),
            YakuKind::Iipeco => localized_name("一盃口", "一杯口"),
            YakuKind::Chankan => localized_name("搶槓", "抢杠"),
            YakuKind::Rinshan => localized_name("嶺上開花", "岭上开花"),
            YakuKind::Haitei => localized_name("海底自摸", "海底摸月"),
            YakuKind::Hotei => localized_name("河底撈魚", "河底捞鱼"),
            YakuKind::Ipatu => localized_name("一発", "一发"),
            YakuKind::DoubleRiichi => localized_name("ダブルリーチ", "两立直"),
            YakuKind::SanshokuDoko => localized_name("三色同刻", "三色同刻"),
            YakuKind::SanshokuDojun => localized_name("三色同順", "三色同顺"),
            YakuKind::Ixtukitukan => localized_name("一気通貫", "一气通贯"),
            YakuKind::ToiToi => localized_name("対対和", "对对和"),
            YakuKind::Sananko => localized_name("三暗刻", "三暗刻"),
            YakuKind::Sankantu => localized_name("三槓子", "三杠子"),
            YakuKind::Shosangen => localized_name("小三元", "小三元"),
            YakuKind::Honroto => localized_name("混老頭", "混老头"),
            YakuKind::Chanta => localized_name("混全帯么九", "混全带幺九"),
            YakuKind::RiyanPeco => localized_name("二盃口", "二杯口"),
            YakuKind::Junchan => localized_name("純全帯么九", "纯全带幺九"),
            YakuKind::Honitu => localized_name("混一色", "混一色"),
            YakuKind::Chinitu => localized_name("清一色", "清一色"),
            YakuKind::Tenho => localized_name("天和", "天和"),
            YakuKind::Chiho => localized_name("地和", "地和"),
            YakuKind::Daisangen => localized_name("大三元", "大三元"),
            YakuKind::Suanko => localized_name("四暗刻", "四暗刻"),
            YakuKind::SuankoTanki => localized_name("四暗刻単騎", "四暗刻单骑"),
            YakuKind::Tuiso => localized_name("字一色", "字一色"),
            YakuKind::Ryuiso => localized_name("緑一色", "绿一色"),
            YakuKind::Chinroto => localized_name("清老頭", "清老头"),
            YakuKind::Shosushi => localized_name("小四喜", "小四喜"),
            YakuKind::Daisushi => localized_name("大四喜", "大四喜"),
            YakuKind::Sukantu => localized_name("四槓子", "四杠子"),
            YakuKind::Churen => localized_name("九蓮宝燈", "九莲宝灯"),
            YakuKind::JunseiChuren => localized_name("純正九蓮宝燈", "纯正九莲宝灯"),
        }
    }
}

#[cfg(feature = "zh-cn")]
fn localized_name(_: &'static str, zh_cn: &'static str) -> &'static str {
    zh_cn
}

#[cfg(not(feature = "zh-cn"))]
fn localized_name(default: &'static str, _: &'static str) -> &'static str {
    default
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct YakuEntry {
    pub kind: YakuKind,
    pub name: String,
    pub value: u8,
}

impl YakuEntry {
    pub fn new(kind: YakuKind, value: u8) -> Self {
        Self {
            kind,
            name: kind.name().to_string(),
            value,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::finder::yaku::YakuKind;

    #[cfg(not(feature = "zh-cn"))]
    #[test]
    fn default_names_are_japanese() {
        assert_eq!(YakuKind::Riichi.name(), "立直");
        assert_eq!(YakuKind::Dora.name(), "ドラ");
    }

    #[cfg(feature = "zh-cn")]
    #[test]
    fn zh_cn_names_are_simplified_chinese() {
        assert_eq!(YakuKind::Riichi.name(), "立直");
        assert_eq!(YakuKind::Dora.name(), "宝牌");
        assert_eq!(YakuKind::SuankoTanki.name(), "四暗刻单骑");
    }
}

use super::tag_type::tag_type::TagType;

pub mod tags {
    pub fn select_tags(tag: Option<super::TagType>) -> Vec<&'static str> {
        match tag {
            Some(super::TagType::FirstPerson) => {
                vec![
                    "僕",
                    "ボク",
                    "ﾎﾞｸ",
                    "俺",
                    "オレ",
                    "ｵﾚ",
                    "小生",
                    "オジサン",
                    "ｵｼﾞｻﾝ",
                    "おじさん",
                    "オイラ",
                ]
            }
            Some(super::TagType::DayOfWeek) => {
                vec!["月", "火", "水", "木", "金", "土", "日"]
            }
            Some(super::TagType::Location) => {
                vec!["愛知", "青森", "秋田", "石川", "茨城", "岩手", "愛媛", "大分", "大阪", "岡山", "沖縄", "香川", "鹿児島", "神奈川", "岐阜", "京都", "熊本", "群馬", "高知", "埼玉", "佐賀", "滋賀", "静岡", "島根", "千葉", "東京", "徳島", "栃木", "鳥取", "富山", "長崎", "長野", "奈良", "新潟", "兵庫", "広島", "福井", "福岡", "福島", "北海道", "三重", "宮城", "宮崎", "山形", "山口", "山梨", "和歌山"]
            }
            Some(super::TagType::Restaurant) => {
                vec!["お寿司🍣", "イタリアン🍝", "バー🍷", "ラーメン屋さん🍜", "中華🍜"]
            }
            Some(super::TagType::Food) => {
                vec![
                    "お惣菜",
                    "サラダ",
                    "おにぎり🍙",
                    "きんぴらごぼう",
                    "ピッツァ🍕",
                    "パスタ🍝",
                    "スイーツ🍮",
                    "ケーキ🎂"
                ]
            }
            Some(super::TagType::Weather) => {
                vec!["曇り", "晴れ", "快晴", "大雨", "雨", "雪", "台風🌀"]
            }
            Some(super::TagType::Nanchatte) => {
                vec![
                    "ﾅﾝﾁｬｯﾃ{EMOJI_POS}",
                    "ナンチャッテ{EMOJI_POS}",
                    "なんちゃって{EMOJI_POS}",
                    "なんてね{EMOJI_POS}",
                    "冗談{EMOJI_POS}",
                    "" // おじさんはたまに本気
                ]
            }
            Some(super::TagType::Hotel) => {
                vec!["ホテル🏨", "ホテル🏩", "旅館"]
            }
            Some(super::TagType::Date) => {
                vec!["デート❤", "カラオケ🎤", "ドライブ🚗"]
            }
            Some(super::TagType::Metaphor) => {
                vec!["天使", "女神", "女優さん", "お姫様"]
            }
            None => {
                vec![]
            }
        }
    }
}

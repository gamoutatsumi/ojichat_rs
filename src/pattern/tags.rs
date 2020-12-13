use super::tag_type::TAG_TYPE;

pub fn select_tags(tag: &str) -> Vec<&'static str> {
    let tag_type = &TAG_TYPE;
    if tag == tag_type["first_person"] {
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
    } else if tag == tag_type["day_of_week"] {
        vec!["月", "火", "水", "木", "金", "土", "日"]
    } else if tag == tag_type["location"] {
        vec!["愛知", "青森", "秋田", "石川", "茨城", "岩手", "愛媛", "大分", "大阪", "岡山", "沖縄", "香川", "鹿児島", "神奈川", "岐阜", "京都", "熊本", "群馬", "高知", "埼玉", "佐賀", "滋賀", "静岡", "島根", "千葉", "東京", "徳島", "栃木", "鳥取", "富山", "長崎", "長野", "奈良", "新潟", "兵庫", "広島", "福井", "福岡", "福島", "北海道", "三重", "宮城", "宮崎", "山形", "山口", "山梨", "和歌山"]
    } else if tag == tag_type["restaurant"] {
        vec!["お寿司🍣", "イタリアン🍝", "バー🍷", "ラーメン屋さん🍜", "中華🍜"]
    } else if tag == tag_type["food"] {
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
    } else if tag == tag_type["weather"] {
        vec!["曇り", "晴れ", "快晴", "大雨", "雨", "雪", "台風🌀"]
    } else if tag == tag_type["nanchatte"] {
        vec![
            "ﾅﾝﾁｬｯﾃ{EMOJI_POS}",
            "ナンチャッテ{EMOJI_POS}",
            "なんちゃって{EMOJI_POS}",
            "なんてね{EMOJI_POS}",
            "冗談{EMOJI_POS}",
            "" // おじさんはたまに本気
        ]
    } else if tag == tag_type["hotel"] {
        vec!["ホテル🏨", "ホテル🏩", "旅館"]
    } else if tag == tag_type["date"] {
        vec!["デート❤", "カラオケ🎤", "ドライブ🚗"]
    } else if tag == tag_type["metaphor"] {
        vec!["天使", "女神", "女優さん", "お姫様"]
    } else {
        vec![]
    }
}

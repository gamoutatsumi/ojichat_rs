use super::emoji_type::EMOJI_TYPE;

pub fn select_tags(tag: &str) -> Vec<&'static str> {
    let emoji_type = &EMOJI_TYPE;
    if tag == emoji_type["emoji_pos"] {
        vec![
            "😃♥ ",
            "😃☀ ",
            "😃",
            "😃✋",
            "❗",
            "😄",
            "😆",
            "😚",
            "😘",
            "💕",
            "💗",
            "😍",
            "🎵",
            "(^_^)",
            "(^o^)",
            "(^з<)",
            "（笑）",
        ]
    } else if tag == emoji_type["emoji_neg"] {
        vec![
            "💦",
            "💔",
            "😱",
            "😰",
            "(◎ ＿◎;)",
            "(T_T)",
            "^^;",
            "(^_^;",
            "(・_・;",
            "(￣Д￣；；",
            "(^▽^;)",
            "(-_-;)",
        ]
    } else if tag == emoji_type["emoji_neut"] {
        vec![
            "💤",
            "😴",
            "🙂",
            "🤑",
            "✋",
            "😪",
            "🛌",
            "😎",
            "😤",
            "（￣▽￣）",
            "(＃￣З￣)",
            "(^^;;",
        ]
    } else if tag == emoji_type["emoji_ask"] {
        vec!["⁉", "❓", "❗❓", "🤔", "😜⁉️", "✋❓", "（￣ー￣?）"]
    } else {
        vec![]
    }
}

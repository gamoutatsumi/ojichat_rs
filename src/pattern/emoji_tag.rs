use super::emoji_type::emoji_type::EmojiType;

pub mod emoji_tag {
    pub fn select_tags(tag: Option<super::EmojiType>) -> Vec<&'static str> {
        match tag {
            Some(super::EmojiType::EmojiPos) => {
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
            }
            Some(super::EmojiType::EmojiNeg) => {
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
            }
            Some(super::EmojiType::EmojiNeut) => {
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
            }
            Some(super::EmojiType::EmojiAsk) => {
                vec![
                    "⁉",
                    "❓",
                    "❗❓",
                    "🤔",
                    "😜⁉️",
                    "✋❓",
                    "（￣ー￣?）",
                ]
            }
            None => {
                vec![]
            }
        }
    }
}

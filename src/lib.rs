mod generator;
mod pattern;

pub mod ojichat {
    use super::generator::Generator;
    /// Returns a generated OJISAN-MESSAGE.
    pub fn get_message(
        target: Option<String>,
        emoji_num: Option<usize>,
        punctuation_level: Option<usize>,
    ) -> String {
        let emoji_num: usize = emoji_num.unwrap_or(4);
        let punctuation_level: usize = punctuation_level.unwrap_or(0);
        Generator::get_message(target, emoji_num, punctuation_level)
    }
}

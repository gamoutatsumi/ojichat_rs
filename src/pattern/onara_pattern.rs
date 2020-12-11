use super::onara_messages::onara_messages;
use super::tags::tags;
use super::ojisan_emotion::OjisanEmotion;
use super::tag_type::tag_type::TagType;
use super::emoji_type::emoji_type::EmojiType;
use counted_array::counted_array;

struct OnaraPattern {
    pattern: counted_array!([OjisanEmotion; _])
}

impl OnaraPattern {
    fn get_message(&self, target: &str, emoji_num: &i64) -> &'static str {
        let template = "";
        for emotion in self.pattern {
            template += super::onara_messages::onara_messages::select_template(emotion);
        }
    }
}

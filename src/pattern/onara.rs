use super::onara_pattern::OnaraPattern;
use super::ojisan_emotion::OjisanEmotion;

use rand::seq::SliceRandom;

pub struct Onara {
}

// Onara ... Ojisan NArikiri Randomized Algorithm: おじさんなりきり乱択アルゴリズム
// おじさんの感情表現の順番を表す。
// 近年の研究によりおじさんなりきるための効果的なアルゴリズムが提唱されている。
impl Onara {
    pub fn select_pattern() -> OnaraPattern {
        let mut patterns: Vec<OnaraPattern> = [
            OnaraPattern::new(vec![OjisanEmotion::GREETING, OjisanEmotion::QUESTION, OjisanEmotion::SYMPATHY]),
            OnaraPattern::new(vec![OjisanEmotion::GREETING, OjisanEmotion::REPORTING]),
            OnaraPattern::new(vec![OjisanEmotion::GREETING, OjisanEmotion::CHEERING]),
            OnaraPattern::new(vec![OjisanEmotion::GREETING, OjisanEmotion::QUESTION, OjisanEmotion::INVITATION]),
            OnaraPattern::new(vec![OjisanEmotion::PRAISING, OjisanEmotion::ADMIRATION]),
            OnaraPattern::new(vec![OjisanEmotion::SYMPATHY, OjisanEmotion::SYMPATHY]),
        ].to_vec();

        let mut rng = rand::thread_rng();
        patterns.shuffle(&mut rng);
        let pattern = patterns.get(0).unwrap().clone();
        pattern
    }
}

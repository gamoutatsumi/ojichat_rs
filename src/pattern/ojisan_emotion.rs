#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum OjisanEmotion {
    // GREETING ... 挨拶
    GREETING,
    // QUESTION ... 疑問
    QUESTION,
    // REPORTING ... 報告
    REPORTING,
    // CHEERING ... 応援
    CHEERING,
    // INVITATION ... 誘い
    INVITATION,
    // SYMPATHY ... 気遣い/慰め/同情
    SYMPATHY,
    // PRAISING ... 褒める
    PRAISING,
    // ADMIRATION ... 自分が参った表現(感服)
    ADMIRATION,
}

//! A module that defines the characters associated with the number of pronunciations.

/// Characters that are ignored when counting pronunciation by syllable unit.
pub const SYLLABLE_CHARS: [char; 3] = ['ン', 'ッ', 'ー'];

/// Characters that are pronounced as a set with the previous character.
pub const LOWER_CASE: [char; 8] = ['ァ', 'ィ', 'ゥ', 'ェ', 'ォ', 'ャ', 'ュ', 'ョ'];

/// Symbols that cannot be counted as pronunciation numbers.
pub const SYMBOLS: [char; 50] = [
    '～', '「', '」', '。', '、', '!', '！', '?', '？', '"', '#', '$', '%', '&', '\'', '(', ')',
    '（', '）', '-', '―', '=', '＝', '^', '＾', '|', '\\', '｜', '￥', '@', '`', '[', ']', '{',
    '}', '｛', '｝', ';', '；', ':', '：', '+', '＋', '*', '＊', '<', '＜', '>', '＞', '_',
];

// pub const VOICELESS: [char; 10] = ['キ', 'ク', 'シ', 'ス', 'チ', 'ツ', 'ヒ', 'フ', 'ピ', 'プ'];
// pub const A_ROW: [char; 5] = ['ア', 'イ', 'ウ', 'エ', 'オ'];
// pub const K_ROW: [char; 5] = ['カ', 'キ', 'ク', 'ケ', 'コ'];
// pub const S_ROW: [char; 5] = ['サ', 'シ', 'ス', 'セ', 'ソ'];
// pub const T_ROW: [char; 5] = ['タ', 'チ', 'ツ', 'テ', 'ト'];
// pub const H_ROW: [char; 5] = ['ハ', 'ヒ', 'フ', 'ヘ', 'ホ'];
// pub const P_ROW: [char; 5] = ['パ', 'ピ', 'プ', 'ぺ', 'ポ'];

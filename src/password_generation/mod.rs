/// The enum `Unicode Group` represents which characters of an alphabet
/// (e.g. the Unicode groups `Basic_latin`, `Latin_1_Supplement` and `Latin_Extended`)
/// can be used while generating the random password.
pub enum UnicodeGroup {
    Basic_Latin,
    Latin_1_Supplement,
    Latin_Extended,
    Greek,
    Cyrillic,
    Arabic,
    Hiragana,
    Katakana,
    Hangul
}
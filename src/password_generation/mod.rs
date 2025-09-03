use rand::{Rng, RngCore};
use rand::rngs::ThreadRng;

/// The enum `Unicode Group` represents which characters of an alphabet
/// (e.g. the Unicode groups `Basic_latin`, `Latin_1_Supplement` and `Latin_Extended`)
/// can be used while generating the random password.
pub enum UnicodeGroup {
    BasicLatin,
    Latin1Supplement,
    LatinExtended,
    Greek,
    Cyrillic,
    Arabic,
    Hiragana,
    Katakana,
    Hangul
}

/// This function borrows a ThreadRng both to choose a random Unicode group
/// and to generate a random character from that group.
pub fn generate_random_character(rng: &mut ThreadRng) -> char {
    let unicode_grp_chosen = choose_unicode_group(rng
        .next_u32());
}

/// This function returns a Unicode group to generate a character from
/// based on the `number` passed.
fn choose_unicode_group(number: u32) -> Option<UnicodeGroup> {
    match number % 9 {
        0 => Some(UnicodeGroup::BasicLatin),
        1 => Some(UnicodeGroup::Latin1Supplement),
        2 => Some(UnicodeGroup::LatinExtended),
        3 => Some(UnicodeGroup::Greek),
        4 => Some(UnicodeGroup::Cyrillic),
        5 => Some(UnicodeGroup::Arabic),
        6 => Some(UnicodeGroup::Hiragana),
        7 => Some(UnicodeGroup::Katakana),
        8 => Some(UnicodeGroup::Hangul),
        _ => None
    }
}
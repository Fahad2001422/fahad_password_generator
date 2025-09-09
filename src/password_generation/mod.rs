use std::process;
use rand::{Rng, RngCore};
use rand::rngs::ThreadRng;

/// The enum `Unicode Group` represents which characters of an alphabet
/// (e.g. the Unicode groups `Basic_latin`, `Latin_1_Supplement` and `Latin_Extended`)
/// can be used while generating the random password.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum UnicodeGroup {
    BasicLatin,
    Latin1Supplement,
    Greek,
    Cyrillic,
    Hebrew,
    Arabic,
    Hiragana,
    Katakana,
    Hangul,
}

/// This function borrows a ThreadRng both to choose a random Unicode group
/// and to generate a random Unicode code point from that group.
pub fn generate_random_code_point(rng: &mut ThreadRng) -> u32 {
    let unicode_grp_chosen = choose_unicode_group(rng
        .next_u32());

    // Return a random Unicode code point using the RNG based on the chosen Unicode group.
    match unicode_grp_chosen {
        Some(UnicodeGroup::BasicLatin) => {
            let random_character = rng.random_range(33..=126) as u32;
            random_character
        },
        Some(UnicodeGroup::Latin1Supplement) => {
            let random_character = rng.random_range(192..=255) as u32;
            random_character
        },
        Some(UnicodeGroup::Greek) => {
            // Greek characters in Unicode range from U+0386 to U+3CE, both inclusive.
            // However, several non-printable characters are inserted in between,
            // which leaves me able to choose only the range between U+03A3 to U+03CE.
            let random_character = rng.random_range(931..=974);
            random_character
        },
        Some(UnicodeGroup::Cyrillic) => {
            let random_character = rng.random_range(1040..=1103);
            random_character
        },
        Some(UnicodeGroup::Hebrew) => {
            let random_character = rng.random_range(1488..=1514);
            random_character
        }
        Some(UnicodeGroup::Arabic) => {
            let random_character = rng.random_range(1569..=1610);
            random_character
        },
        Some(UnicodeGroup::Hiragana) => {
            let random_character = rng.random_range(12353..=12436);
            random_character
        },
        Some(UnicodeGroup::Katakana) => {
            let random_character = rng.random_range(12449..=12532);
            random_character
        },
        // When Hangul is chosen, it generates a random Unicode code point
        // from the "Hangul Syllables" Unicode block, which represents
        // the actual syllables used in normal Korean writing.
        Some(UnicodeGroup::Hangul) => {
            let random_character = rng.random_range(44032..=55203);
            random_character
        },
        None => {
            // The Option<T> variable `unicode_grp_chosen` cannot be None,
            // obviously because the modulo `a % b` operator always returns a number
            // between 0 and b, both inclusive.
            // However, I included a None match arm just for correctness.
            eprintln!("Error: There was not a Unicode group selected, this is an internal bug.");
            eprintln!("Submit a bug report to https://github.com/Fahad2001422/fahad_password_generator");
            process::exit(17);
        }
    }
}

/// This function returns a randomly Unicode chosen group to
/// generate a Unicode code point from that group based on
/// the `number` passed.
fn choose_unicode_group(number: u32) -> Option<UnicodeGroup> {
    match number % 9 {
        0 => Some(UnicodeGroup::BasicLatin),
        1 => Some(UnicodeGroup::Latin1Supplement),
        2 => Some(UnicodeGroup::Greek),
        3 => Some(UnicodeGroup::Cyrillic),
        4 => Some(UnicodeGroup::Hebrew),
        5 => Some(UnicodeGroup::Arabic),
        6 => Some(UnicodeGroup::Hiragana),
        7 => Some(UnicodeGroup::Katakana),
        8 => Some(UnicodeGroup::Hangul),
        _ => None
    }
}

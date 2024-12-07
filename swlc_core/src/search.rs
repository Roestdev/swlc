use crate::model_definition::{Tanach, VerseReference};
use std::borrow::Cow;

pub fn search_text_in_tanach<'a>(tanach: &'a Tanach, needle: &'a str) -> Result<(), &'a str> {
    let char_types = get_character_types(needle);
    //println!("{:?}",char_types);
    if char_types.non_hebrew {
        let error = r#"
        Sorry, the given text is either
          a) an invalid command or
          b) it contains at least one invalid Hebrew character.
        Please try again!
        "#;
        return Err(error);
    }
    // loop tanach
    // 1. filter tanach based on char types
    // 2. search needle

    Ok(())
}
pub fn get_character_types(s: &str) -> HebrewCharacterTypes {
    let mut found_character_types = HebrewCharacterTypes::new();
    for c in s.chars() {
        match c {
            c if is_accent(c) => found_character_types.accent = true,
            c if is_mark(c) => found_character_types.mark = true,
            c if is_vowel_point(c) => found_character_types.vowel_point = true,
            c if is_punctuation(c) => found_character_types.punctuation = true,
            c if is_letter_final(c) => found_character_types.letter_final = true,
            c if is_letter_regular(c) => found_character_types.letter_normal = true,
            c if is_yod_triangle(c) => found_character_types.yod_triangle = true,
            c if is_ligature_yiddish(c) => found_character_types.ligature_yiddish = true,
            c if c.is_whitespace() => found_character_types.whitespace = true,
            _ => found_character_types.non_hebrew = true,
        }
    }
    found_character_types.letter =
        found_character_types.letter_normal | found_character_types.letter_final;
    found_character_types
}
pub fn is_valid_hebrew_text(text: &str) -> bool {
    for c in text.chars() {
        match c {
            c if is_hebrew_char_class(c) => (),
            c if c.is_whitespace() => (),
            _ => return false,
        }
    }
    true
}
pub fn is_hebrew_char_class(c: char) -> bool {
    // U+0590 .. U+05FE
    matches!(c, '\u{0590}'..='\u{05FE}')
}
pub fn search_text(_text: &str) {
    println!("SEARCH TEXT::searching text is valid");
}

#[derive(Debug, Default)]
pub struct HebrewCharacterTypes {
    accent: bool,
    mark: bool,
    vowel_point: bool,
    punctuation: bool,
    letter: bool,
    letter_normal: bool,
    letter_final: bool,
    yod_triangle: bool,
    ligature_yiddish: bool,
    whitespace: bool,
    non_hebrew: bool,
}
impl HebrewCharacterTypes {
    fn new() -> Self {
        Default::default()
    }
}

pub fn is_accent(c: char) -> bool {
    // 0591 .. 05AE
    matches!(c, '\u{0591}'..='\u{05AE}')
}
pub fn is_mark(c: char) -> bool {
    // 05AF + 05C4 + 05C5
    matches!(c, '\u{05AF}' | '\u{05C4}' | '\u{05C5}')
}
pub fn is_vowel_point(c: char) -> bool {
    // 05B0 .. 05BD + 05BF + 05C1 .. 05C2 + 05C7
    matches!(c,
        '\u{05B0}'..='\u{05BD}'
        | '\u{05BF}'
        | '\u{05C1}'..='\u{05C2}'
        | '\u{05C7}'
    )
}
pub fn is_punctuation(c: char) -> bool {
    // 05BE + 05C0 + 05C3 + 05C6 + 05F3 + 05F4
    matches!(
        c,
        '\u{05BE}' | '\u{05C0}' | '\u{05C3}' | '\u{05C6}' | '\u{05F3}'..='\u{05F4}'
    )
}
pub fn is_letter(c: char) -> bool {
    // 05D0 .. 05EA
    matches!(c, '\u{05D0}'..='\u{05EA}')
}
pub fn is_letter_regular(c: char) -> bool {
    // 05D0..05D9 + 05DB..05DC + 05DE + 05E0..05E2 + 05E4 + 05E6..05EA
    matches!(
        c,
        '\u{05D0}'..='\u{05D9}'
        | '\u{05DB}'..='\u{05DC}'
        | '\u{05DE}'
        | '\u{05E0}'..='\u{05E2}'
        | '\u{05E4}'
        | '\u{05E6}'..='\u{05EA}'
    )
}
pub fn is_letter_final(c: char) -> bool {
    // 05DA + 05DD + 05DF + 05E3 + 05E5
    matches!(
        c,
        '\u{05DA}' | '\u{05DD}' | '\u{05DF}' | '\u{05E3}' | '\u{05E5}'
    )
}
pub fn is_consonant_regular(c: char) -> bool {
    // 05D0..05D9 + 05DB..05DC + 05DE + 05E0..05E2 + 05E4 + 05E6..05EA
    matches!(
        c,
        '\u{05D0}'..='\u{05D9}'
        | '\u{05DB}'..='\u{05DC}'
        | '\u{05DE}'
        | '\u{05E0}'..='\u{05E2}'
        | '\u{05E4}'
        | '\u{05E6}'..='\u{05EA}'
    )
}
pub fn is_consonant_final(c: char) -> bool {
    // 05DA + 05DD + 05DF + 05E3 + 05E5
    matches!(
        c,
        '\u{05DA}' | '\u{05DD}' | '\u{05DF}' | '\u{05E3}' | '\u{05E5}'
    )
}
pub fn is_yod_triangle(c: char) -> bool {
    matches!(c, '\u{05EF}')
}
pub fn is_ligature_yiddish(c: char) -> bool {
    // 05F0 .. 05F2
    matches!(c, '\u{05F0}'..='\u{05F2}')
}

/*fn dummy_search(tanach: Tanach, search_for: &str) {
        let _sr  = tanach
            .books
            .into_iter()
            .flat_map(|b| {
                b.references_and_verses
                    .iter().for_each(|(r,t)|)
            })
            .collect::<Vec<(VerseReference,String)>>();

}*/

pub fn remove(s: &str) -> Cow<'_, str> {
    // {"letter": 5, "vowel_point": 3}
    let mut found_character_types = HebrewCharacterTypes::new();
    found_character_types.vowel_point = false;
    println!("{:?}", found_character_types);
    s.chars()
        .filter(|&c| {
            c.is_whitespace()
                | c.is_ascii()
                | is_letter(c)
                | (is_vowel_point(c) & found_character_types.vowel_point)
                | (is_accent(c) & found_character_types.accent)
                | (is_mark(c) & found_character_types.mark)
                | (is_punctuation(c) & found_character_types.punctuation)
                | (is_yod_triangle(c) & found_character_types.yod_triangle)
                | (is_ligature_yiddish(c) & found_character_types.ligature_yiddish)
        })
        .collect()
}

/*pub fn remove_character_types(s:&str,types: &CharacterTypes) -> Cow<'_, str> {


}
*/

use rand::Rng;

const LETTERS: &str = "\
ABCDEFGHIJKLMNOPQRSTUVWXYZ\
abcdefghijklmnopqrstuvwxyz";
const NUMBERS: &str = "0123456789";
const SPECIAL_CHARS: &str = "!@#$%^&*";

pub fn generate_pw(length: u32, numbers_allowed: bool, special_chars_allowed: bool) -> String {
    let char_set = format!(
        "{}{}{}",
        LETTERS,
        if numbers_allowed { NUMBERS } else { "" },
        if special_chars_allowed {
            SPECIAL_CHARS
        } else {
            ""
        }
    );

    (0..length)
        .map(|_| {
            let idx = rand::thread_rng().gen_range(0..char_set.as_bytes().len());
            char_set.as_bytes()[idx] as char
        })
        .collect()
}

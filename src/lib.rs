use std::borrow::Cow;

const INVALID_CHARS: [char; 9] = ['<', '>', ':', '"', '/', '\\', '|', '?', '*'];

pub fn alter_invalid_win_chars(name: Cow<str>) -> Option<String> {
    if let Some(idx) = name.find(INVALID_CHARS.as_ref()) {
        let mut new_name;
        let (valid_part, invalid_part) = name.split_at(idx);
        new_name = valid_part.to_owned();
        new_name += &invalid_part.replace(INVALID_CHARS.as_ref(), "-");
        Some(new_name)
    } else {
        None
    }
}

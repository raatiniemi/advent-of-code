use std::fs;

/// Read the contents of a file on a given path.
pub fn read_contents_of_file(path: &str) -> Vec<String> {
    return fs::read_to_string(path)
        .expect(&format!("Unable to read file at path {}", path).as_str())
        .split_terminator("\n")
        .map(|v| { v.to_string() })
        .collect();
}

/// Map a [Vec] of [String] to a [Vec] of [i32].
pub fn map_to_i32(vec: Vec<String>) -> Vec<i32> {
    return vec.iter()
        .filter_map(|v| {
            let result = v.parse::<i32>();
            if result.is_err() {
                eprintln!("Unable to parse i32 from: {}", v);
                None
            } else {
                result.ok()
            }
        })
        .collect();
}

/// Map a [Vec] of [String] to a [Vec] of [i64].
pub fn map_to_i64(vec: Vec<String>) -> Vec<i64> {
    return vec.iter()
        .filter_map(|v| {
            let result = v.parse::<i64>();
            if result.is_err() {
                eprintln!("Unable to parse i64 from: {}", v);
                None
            } else {
                result.ok()
            }
        })
        .collect();
}

/// Read character at index from String.
///
/// With a valid index, the character (as [String]) at the index will be returned as [Option.Some].
///
/// ```rust
/// # use twentytwenty::character_at_index;
/// let value = String::from("abc");
/// let character = character_at_index(1, &value);
///
/// assert_eq!(character.is_some(), true);
/// assert_eq!(character.unwrap(), String::from("b"));
/// ```
///
/// With an invalid index, [Option.None] will be returned.
///
/// ```rust
/// # use twentytwenty::character_at_index;
/// let value = String::from("abc");
/// let character = character_at_index(7, &value);
///
/// assert_eq!(character.is_none(), true);
/// ```
pub fn character_at_index(index: i32, s: &String) -> Option<String> {
    let index = index as usize;
    if index >= s.len() {
        return None;
    }

    return s.chars()
        .nth(index)
        .map(|v| { String::from(v) });
}

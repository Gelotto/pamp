pub fn pad(
    input: &str,
    target_length: usize,
) -> String {
    let mut result = String::from(input);
    while result.len() < target_length {
        result.push('\0');
    }
    result
}

pub fn unpad(input: &String) -> String {
    input.trim_end_matches('\0').to_string()
}

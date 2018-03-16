/// This function accepts a camel-case or title-case name and converts it to snake-case.
///
/// # Examples
/// ```
/// use translate::*;
/// assert_eq!(translate("thisIsACamel"), "this_is_a_camel");
/// assert_eq!(translate("ThisIsATitle"), "this_is_a_title");
/// ```
pub fn translate(s: &str) -> String {
    let mut result = Vec::new();

    let mut index = 0;
    for c in s.chars() {
        let mut cc = c;

        if cc >= 'A' && cc <= 'Z' {
            if index > 0 {
                result.push('_');
            }

            cc = ((cc as u8) + 32) as char; // add 32 because 65 is A and 97 is a and 97 - 65 = 32; this lowercases a capital letter
        }

        result.push(cc);
        index += 1;
    }

    return result.into_iter().collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translate_camel_case() {
        let test = "thisIsACamel";
        let result = translate(test);
        assert_eq!(result, "this_is_a_camel");
    }

    #[test]
    fn test_translate_title_case() {
        let test = "ThisIsATitle";
        let result = translate(test);
        assert_eq!(result, "this_is_a_title");
    }
}
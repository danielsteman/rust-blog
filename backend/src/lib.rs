use regex::Regex;

pub fn find_words_between_colons(text: &str) -> Vec<String> {
    let re = Regex::new(r":\s*([^:\s]+)\s*:?").unwrap();
    let mut words = Vec::new();

    for capture in re.captures_iter(text) {
        if let Some(word) = capture.get(1) {
            words.push(word.as_str().to_string());
        }
    }

    words
}

pub fn find_emojis(text: &str) -> &str {
    text
}

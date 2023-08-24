use backend::find_emojis;
use backend::find_words_between_colons;

#[test]
fn test_parse_emoji() {
    let text = "Hello";
    let emojis = find_emojis(text);
    assert_eq!(emojis, "Hello");
}

#[test]
fn test_find_words_between_colons() {
    let text = "Hello :smiley:, :crab:, and such:hi:";
    let shortcodes = find_words_between_colons(text);
    assert_eq!(shortcodes, vec!["smiley", "crab", "hi"])
}

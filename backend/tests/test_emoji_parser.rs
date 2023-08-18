use backend::find_emojis;

#[test]
fn test_parse_emoji() {
    let text = "Hello";
    let emojis = find_emojis(text);
    assert_eq!(emojis, "Hello");
}

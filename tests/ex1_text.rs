use rust_lab::ex1_text::*;

#[test]
fn counts_words_case_insensitive_and_ignores_punctuation() {
    let f = word_frequency("The cat and the hat. THE end!");
    assert_eq!(f.get("the"), Some(&3));
    assert_eq!(f.get("cat"), Some(&1));
    assert_eq!(f.get("end"), Some(&1));
    assert_eq!(f.len(), 5);
}

#[test]
fn empty_text_has_no_words() {
    assert!(word_frequency("").is_empty());
    assert!(word_frequency("... !!! 123").is_empty());
}

#[test]
fn longest_word_basic() {
    assert_eq!(longest_word("a bb ccc dd eee"), Some("ccc")); // اولی برنده است
    assert_eq!(longest_word("hi, there!"), Some("there"));
}

#[test]
fn longest_word_none() {
    assert_eq!(longest_word(""), None);
    assert_eq!(longest_word("1 2 3"), None);
}

#[test]
fn longest_word_borrows_from_input() {
    let text = String::from("borrow checker rocks");
    let w = longest_word(&text).unwrap();
    assert_eq!(w, "checker");
}

#[test]
fn top_n_sorts_by_count_then_alphabet() {
    let f = word_frequency("b a c a b a d");
    // a:3  b:2  c:1  d:1
    assert_eq!(
        top_n(&f, 3),
        vec![("a".to_string(), 3), ("b".to_string(), 2), ("c".to_string(), 1)]
    );
    assert_eq!(top_n(&f, 100).len(), 4);
    assert!(top_n(&f, 0).is_empty());
}

use stf_core::{TextFragment, build_url};

#[test]
fn simple_text_produces_minimal_directive() {
    let fragment = TextFragment::new(String::from("iceberg"), None, None, None);

    assert_eq!(
        build_url("https://example.com", &fragment).unwrap(),
        "https://example.com/#:~:text=iceberg"
    );
}

#[test]
fn rejects_empty_text_through_the_public_api() {
    let fragment = TextFragment::new(String::new(), None, None, None);

    assert!(build_url("https://example.com", &fragment).is_err());
}

#[test]
fn rejects_malformed_base_url() {
    let fragment = TextFragment::new(String::from("hello"), None, None, None);

    assert!(build_url("not a url", &fragment).is_err());
}

#[test]
fn preserves_port_and_userinfo_on_base() {
    let fragment = TextFragment::new(String::from("widget"), None, None, None);

    assert_eq!(
        build_url("https://example.com:8080/inventory", &fragment).unwrap(),
        "https://example.com:8080/inventory#:~:text=widget"
    );
}

#[test]
fn ampersand_in_text_is_encoded_not_treated_as_separator() {
    let fragment = TextFragment::new(String::from("Tom & Jerry"), None, None, None);

    assert_eq!(
        build_url("https://example.com", &fragment).unwrap(),
        "https://example.com/#:~:text=Tom%20%26%20Jerry"
    );
}

#[test]
fn handles_the_long_block_of_text_case() {
    let text = "It is here that Wittgenstein’s rejection of general explanations, \
    and definitions based on sufficient and necessary conditions, is best \
    pronounced. Instead of these symptoms of the philosopher’s “craving for \
    generality,” he points to ‘family resemblance’ as the more suitable \
    analogy for the means of connecting particular uses of the same word. \
    There is no reason to look, as we have done traditionally—and \
    dogmatically—for one, essential core in which the meaning of a word is \
    located and which is, therefore, common to all uses of that word. We \
    should, instead, travel with the word’s uses through “a complicated \
    network of similarities overlapping and criss-crossing” (PI 66). Family \
    resemblance also serves to exhibit the lack of boundaries and the \
    distance from exactness that characterize different uses of the same concept.";

    let fragment = TextFragment::new(text.to_string(), None, None, None);
    let url = build_url(
        "https://plato.stanford.edu/entries/wittgenstein/",
        &fragment,
    )
    .expect("a long unicode passage should still produce a valid URL");

    assert!(url.starts_with("https://plato.stanford.edu/entries/wittgenstein/#:~:text="));

    assert!(!url.contains('‘'));
    assert!(!url.contains('“'));
    assert!(!url.contains('—'));

    let (_, encoded) = url.split_once("text=").unwrap();
    let decoded = urlencoding::decode(encoded).unwrap();

    assert_eq!(decoded, text);
}

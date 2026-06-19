use crate::fragment::TextFragment;

pub fn build_url(base: &str, fragment: &TextFragment) -> Result<String, FragmentError> {
    todo!()
}

#[derive(Debug, PartialEq)]
pub struct FragmentError;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_full_url() {
        let mut fragment =
            TextFragment::new(String::from("human"), Some(String::from("URL")), None, None);

        assert_eq!(
            build_url("https://example.com", &fragment),
            Ok(String::from("https://example.com#:~:text=human,URL"))
        );

        fragment = TextFragment::new(
            String::from(
                "The%20first%20recorded%20idea%20of%20using%20digital%20electronics%20for%20computing%20was%20the%201931%20paper%20%22The%20Use%20of%20Thyratrons%20for%20High%20Speed%20Automatic%20Counting%20of%20Physical%20Phenomena%22%20by%20C.%20E.%20Wynn-Williams",
            ),
            None,
            None,
            None,
        );

        assert_eq!(
            build_url("https://example.com", &fragment),
            Ok(String::from(
                "https://example.com#:~:text=The%20first%20recorded%20idea%20of%20using%20digital%20electronics%20for%20computing%20was%20the%201931%20paper%20%22The%20Use%20of%20Thyratrons%20for%20High%20Speed%20Automatic%20Counting%20of%20Physical%20Phenomena%22%20by%20C.%20E.%20Wynn-Williams"
            ))
        );

        fragment = TextFragment::new(
            String::from("linked%20URL"),
            Some(String::from("defining%20a%20value")),
            None,
            None,
        );

        assert_eq!(
            build_url("https://example.com", &fragment),
            Ok(String::from(
                "https://example.com#:~:text=linked%20URL,defining%20a%20value"
            ))
        );

        fragment = TextFragment::new(
            String::from("%D9%85%D9%90%D8%B5%D8%B1"),
            None,
            Some(String::from("%D8%A7%D9%84%D8%A8%D8%AD%D8%B1%D9%8A%D9%86")),
            None,
        );

        assert_eq!(
            build_url("https://example.com", &fragment),
            Ok(String::from(
                "https://example.com#:~:text=%D8%A7%D9%84%D8%A8%D8%AD%D8%B1%D9%8A%D9%86-,%D9%85%D9%90%D8%B5%D8%B1"
            ))
        );

        fragment = TextFragment::new(
            String::from("The%20Referer"),
            Some(String::from("be%20sent")),
            Some(String::from("downgrade:")),
            Some(String::from("to%20origins")),
        );

        assert_eq!(
            build_url("https://example.com", &fragment),
            Ok(String::from(
                "https://example.com#:~:text=downgrade:-,The%20Referer,be%20sent,-to%20origins"
            ))
        );
    }
}

use crate::fragment::TextFragment;

pub fn build_url(base: &str, fragment: &TextFragment) -> Result<String, FragmentError> {
    todo!()
}

#[derive(Debug, PartialEq)]
pub struct FragmentError;

#[cfg(test)]
mod tests {
    use super::*;

    fn should_return_full_url() {
        let mut fragment =
            TextFragment::new(String::from("human"), Some(String::from("URL")), None, None);

        assert_eq!(fragment.to_directive(), "#:~:text=human,URL");

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
    }
}

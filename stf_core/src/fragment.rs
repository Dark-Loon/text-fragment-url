/// Represent the text fragment directive
/// See: https://wicg.github.io/scroll-to-text-fragment/
pub struct TextFragment {
    pub start: String,
    pub end: Option<String>,
    pub prefix: Option<String>,
    pub suffix: Option<String>,
}

impl TextFragment {
    pub fn new(
        start: String,
        end: Option<String>,
        prefix: Option<String>,
        suffix: Option<String>,
    ) -> TextFragment {
        TextFragment {
            start: start,
            end: end,
            prefix: prefix,
            suffix: suffix,
        }
    }

    pub fn to_directive(self) -> String {
        let mut string = String::from("#:~:text=");
        let comma = String::from(",");

        if let Some(value) = self.prefix {
            string.push_str(&value);
            string.push_str(&comma);
        }

        string.push_str(&self.start);

        if let Some(value) = self.end {
            string.push_str(&comma);
            string.push_str(&value);
        }

        if let Some(value) = self.suffix {
            string.push_str(&comma);
            string.push_str(&value);
        }

        string
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_directive() {
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
            fragment.to_directive(),
            "#:~:text=The%20first%20recorded%20idea%20of%20using%20digital%20electronics%20for%20computing%20was%20the%201931%20paper%20%22The%20Use%20of%20Thyratrons%20for%20High%20Speed%20Automatic%20Counting%20of%20Physical%20Phenomena%22%20by%20C.%20E.%20Wynn-Williams"
        );

        fragment = TextFragment::new(
            String::from("linked%20URL"),
            Some(String::from("defining%20a%20value")),
            None,
            None,
        );

        assert_eq!(
            fragment.to_directive(),
            "#:~:text=linked%20URL,defining%20a%20value"
        );

        // fragment = TextFragment::new(
        //     String::from("البحرين-"),
        //     None,
        //     Some(String::from("مِصر")),
        //     None,
        // );

        // assert_eq!(
        //     fragment.to_directive(),
        //     "#:~:text=%D8%A7%D9%84%D8%A8%D8%AD%D8%B1%D9%8A%D9%86-,%D9%85%D8%B5%D8%B1"
        // );

        fragment = TextFragment::new(
            String::from("The%20Referer"),
            Some(String::from("be%20sent")),
            Some(String::from("downgrade:-")),
            Some(String::from("-to%20origins")),
        );

        assert_eq!(
            fragment.to_directive(),
            "#:~:text=downgrade:-,The%20Referer,be%20sent,-to%20origins"
        );
    }
}

use std::env;

fn main() {
    let result = match env::args().nth(1) {
        Some(input) if determine_xml(&input) => "Valid",
        _ => "Invalid",
    };

    println!("{result}");
}

/// returns true when every tag is matched and the tags are correctly nested.
fn determine_xml(input: &str) -> bool {
    let bytes = input.as_bytes();
    let mut open_tags = Vec::new();
    let mut cursor = 0;

    while cursor < bytes.len() {
        if bytes[cursor] != b'<' {
            cursor += 1;
            continue;
        }

        let tag_start = cursor + 1;
        let mut tag_end = tag_start;

        while tag_end < bytes.len() && bytes[tag_end] != b'>' {
            // a new tag cannot begin before the current tag is terminated.
            if bytes[tag_end] == b'<' {
                return false;
            }
            tag_end += 1;
        }

        if tag_end == bytes.len() {
            return false;
        }

        let tag = &input[tag_start..tag_end];
        if tag.is_empty() {
            return false;
        }

        if let Some(closing_tag) = tag.strip_prefix('/') {
            if closing_tag.is_empty() || open_tags.pop() != Some(closing_tag) {
                return false;
            }
        } else {
            open_tags.push(tag);
        }

        cursor = tag_end + 1;
    }

    open_tags.is_empty()
}

#[cfg(test)]
mod tests {
    use crate::determine_xml;

    #[test]
    fn validates_supported_inputs() {
        let cases = [
            (
                "assignment example: balanced nested tags",
                "<Design><Code>hello world</Code></Design>",
            ),
            ("nested tags and siblings", "<a><b>text</b><c></c></a>"),
            ("multiple top-level pairs", "<a></a><b></b>"),
            (
                "matching tag contents that look like attributes",
                r#"<tutorial date="07/04/2000">XML</tutorial date="07/04/2000">"#,
            ),
            ("explicit empty input", ""),
            ("text-only input", "hello world"),
        ];

        for (name, input) in cases {
            assert!(determine_xml(input), "expected valid: {name}");
        }
    }

    #[test]
    fn rejects_unsupported_or_unbalanced_inputs() {
        let cases = [
            (
                "assignment example: trailing opening tag",
                "<Design><Code>hello world</Code></Design><People>",
            ),
            (
                "assignment example: crossed nesting",
                "<People><Design><Code>hello world</People></Code></Design>",
            ),
            (
                "assignment example: different raw tag contents",
                "<People age=”1”>hello world</People>",
            ),
            ("opening tag only", "<a>"),
            ("closing tag only", "</a>"),
            ("different names", "<a></b>"),
            ("unterminated tag", "<a"),
            ("empty opening tag", "<></>"),
            ("nested opening delimiter", "<a<b></b></a>"),
            ("unsupported self-closing syntax", "<x/>"),
        ];

        for (name, input) in cases {
            assert!(!determine_xml(input), "expected invalid: {name}");
        }
    }
}

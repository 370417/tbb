//! Functions for manipulating markdown

use pulldown_cmark::{CodeBlockKind, Event, Parser, Tag};

/// Calls a closure for each code block in the input.
pub fn for_each_code_block<F>(markdown: &str, fun: F)
where
    F: Fn(&str),
{
    let parser = Parser::new(markdown);
    let mut is_in_code_block = false;
    parser.for_each(|event| match event {
        Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(lang))) => {
            is_in_code_block = lang.as_ref() == "console";
        }
        Event::End(Tag::CodeBlock(_)) => {
            is_in_code_block = false;
        }
        Event::Text(text) if is_in_code_block => fun(&text),
        _ => {}
    })
}

/// Replaces the contents of every code block with the output of `rewriter`.
/// Only affects code blocks with language "console".
pub fn rewrite<F>(markdown: &str, rewriter: F) -> String
where
    F: Fn(&str) -> String,
{
    // Iterate backwards to avoid invlidating the range indices,
    // and replace code blocks when we come across them.
    // This is slow but simple.
    let mut markdown_bytes = markdown.as_bytes().to_vec();
    let parser = Parser::new(markdown);
    let mut is_in_code_block = false;
    parser
        .into_offset_iter()
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .for_each(|(event, range)| match event {
            Event::End(Tag::CodeBlock(CodeBlockKind::Fenced(lang))) => {
                is_in_code_block = lang.as_ref() == "console";
            }
            Event::Start(Tag::CodeBlock(_)) => {
                is_in_code_block = false;
            }
            Event::Text(text) if is_in_code_block => {
                let rewritten = rewriter(&text).as_bytes().to_owned();
                markdown_bytes.splice(range, rewritten);
            }
            _ => {}
        });
    std::str::from_utf8(&markdown_bytes)
        .map(str::to_owned)
        .expect("rewritten markdown is not utf8")
}

#[cfg(test)]
mod tests {
    use super::*;

    const MARKDOWN: &str = "Hello, World\n\
    \n\
    ```console\n\
    abcdef\n\
    abcdef\n\
    ```\n\
    \n\
    > Quote **bold**\n\
    \n\
    ```\n\
    language not declared correctly,\n\
    should be ignored\n\
    ```\n\
    \n\
    ```console\n\
    123456\n\
    123456\n\
    ```\n";

    const MARKDOWN_AFTER_REWRITE: &str = "Hello, World\n\
    \n\
    ```console\n\
    abcdef\
    ```\n\
    \n\
    > Quote **bold**\n\
    \n\
    ```\n\
    language not declared correctly,\n\
    should be ignored\n\
    ```\n\
    \n\
    ```console\n\
    123456\
    ```\n";

    #[test]
    fn test_check_correctness() {
        for_each_code_block(MARKDOWN, |str| {
            assert!(str == "abcdef\nabcdef\n" || str == "123456\n123456\n");
        });
    }

    #[test]
    fn test_rewrite() {
        let output = rewrite(MARKDOWN, |str| str.lines().last().unwrap().to_owned());
        assert_eq!(output, MARKDOWN_AFTER_REWRITE);
    }
}

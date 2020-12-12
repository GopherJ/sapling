use super::Ast;

/// How many spaces corespond to one indentation level
const INDENT_WIDTH: usize = 4;

/// A category of text that should be syntax highlighted the same color.
///
/// Standard categories include:
/// - `"default"`: text that shouldn't be highlighted a specific colour: used for things like
///   punctuation.
/// - `"const"`: constant values like 'true', 'false'
/// - `"literal"`: literal values like strings or integers
/// - `"comment"`: code comments
/// - `"ident"`: code identifers, such as variable names and functions
/// - `"keyword"`: a name that's reserved by the language for some purpose (e.g. `if`, `while` in
///   nearly all languages or `use` in Rust)
/// - `"preproc"`: any preprocessor directive like `#define` in C, and `#[derive(...)]`s in Rust
/// - `"type"`: a datatype, e.g. `int`, `long` from C or `usize`, `f64`, `String` in Rust
/// - `"special"`: special bits of text like escaped characters (`\n`, etc.) in string literals.
/// - `"underlined"`: (leftover from Vim - do we really need this?)
/// - `"error"`: any code that is an error.
pub type SyntaxCategory = &'static str;

/// A single piece of a node that can be rendered to the screen
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum DisplayToken {
    /// Some text should be rendered to the screen
    Text(String, SyntaxCategory),
    /// Add some number of spaces worth of whitespace
    Whitespace(usize),
    /// Put the next token onto a new line
    Newline,
    /// Add another indent level to the code
    Indent,
    /// Remove an indent level from the code
    Dedent,
}

/// A wrapper for [`DisplayToken`] that will be returned by [`Ast::display_tokens`] and allows for
/// child references to be recursively expanded.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum RecTok<'arena, Node> {
    Tok(DisplayToken),
    Child(&'arena Node),
}

/// Write a stream of display tokens to a string
pub fn write_tokens<'arena, Node: Ast<'arena>>(
    root: &'arena Node,
    string: &mut String,
    format_style: &Node::FormatStyle,
) {
    let mut indentation_string = String::new();

    // Process the token string
    for (_id, tok) in root.display_tokens(format_style) {
        match tok {
            DisplayToken::Text(s, _) => {
                // Push the string we've been given
                string.push_str(&s);
            }
            DisplayToken::Whitespace(n) => {
                // Push 'n' many spaces
                for _ in 0..n {
                    string.push(' ');
                }
            }
            DisplayToken::Newline => {
                // Push a newline and keep indentation
                string.push('\n');
                string.push_str(&indentation_string);
            }
            DisplayToken::Indent => {
                // Add `INDENT_WIDTH` spaces to the indentation_string
                for _ in 0..INDENT_WIDTH {
                    indentation_string.push(' ');
                }
            }
            DisplayToken::Dedent => {
                // Remove `INDENT_WIDTH` spaces to the indentation_string
                for _ in 0..INDENT_WIDTH {
                    let popped_char = indentation_string.pop();
                    debug_assert_eq!(popped_char, Some(' '));
                }
            }
        }
    }
}

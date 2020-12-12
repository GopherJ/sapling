use crate::ast::display_token::SyntaxCategory;
use tuikit::prelude::Color;

/// Setting this flag to `true` will override the current syntax highlighting with a debug view
/// where every node is highlighted according to its hash value.
///
/// This mode is not useful for text editing, but is very useful for debugging.
pub const DEBUG_HIGHLIGHTING: bool = false;

/// A mapping from syntax highlighting categories to terminal [`Color`]s
pub type ColorScheme = std::collections::HashMap<SyntaxCategory, Color>;

/// Return the default [`ColorScheme`] of Sapling
pub fn default_color_scheme() -> ColorScheme {
    hmap::hmap! {
        "default" => Color::WHITE,
        "const" => Color::RED,
        "literal" => Color::YELLOW,
        "comment" => Color::GREEN,
        "indent" => Color::CYAN,
        "keyword" => Color::BLUE,
        "preproc" => Color::MAGENTA,
        "type" => Color::LIGHT_YELLOW,
        "special" => Color::LIGHT_GREEN,
        "underlined" => Color::LIGHT_RED,
        "error" => Color::LIGHT_RED
    }
}

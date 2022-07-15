//! Non-lazy continuation.
//!
//! This is a tiny helper that [flow][] constructs can use to make sure that
//! the following line is not lazy.
//! For example, [html (flow)][html_flow] and code ([fenced][code_fenced],
//! [indented][code_indented]), stop when next line is lazy.
//!
//! [flow]: crate::content::flow
//! [code_fenced]: crate::construct::code_fenced
//! [code_indented]: crate::construct::code_indented
//! [html_flow]: crate::construct::html_flow

use crate::token::Token;
use crate::tokenizer::{Code, State, StateFnResult, Tokenizer};

/// Start of continuation.
///
/// ```markdown
/// > | * ```js
///            ^
///   | b
/// ```
pub fn start(tokenizer: &mut Tokenizer, code: Code) -> StateFnResult {
    match code {
        Code::CarriageReturnLineFeed | Code::Char('\n' | '\r') => {
            tokenizer.enter(Token::LineEnding);
            tokenizer.consume(code);
            tokenizer.exit(Token::LineEnding);
            (State::Fn(Box::new(after)), None)
        }
        _ => (State::Nok, None),
    }
}

/// After line ending.
///
/// ```markdown
///   | * ```js
/// > | b
///     ^
/// ```
fn after(tokenizer: &mut Tokenizer, code: Code) -> StateFnResult {
    if tokenizer.lazy {
        (State::Nok, None)
    } else {
        (State::Ok, Some(vec![code]))
    }
}

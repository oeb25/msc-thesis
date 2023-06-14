use mist_syntax::ast::SyntaxKind;
use syntect::{highlighting::Style, util::as_latex_escaped};

use crate::colors::PALETTE;

struct Theme {
    kw: Style,
    literal: Style,
    punct: Style,
}

const DEFAULT_THEME: Theme = Theme {
    kw: PALETTE.teal_600,
    literal: PALETTE.teal_500,
    punct: PALETTE.gray_500,
};

pub fn highlight_mist(code: &str, ignore_errors: bool) -> String {
    let parse = mist_syntax::parse(code);
    if !ignore_errors {
        for err in parse.errors() {
            eprintln!("{:?}", miette::Error::new(err.clone()));
        }
    }

    parse
        .tree()
        .syntax()
        .preorder_with_tokens()
        .filter_map(|event| {
            let node_or_token = match event {
                mist_syntax::WalkEvent::Enter(it) => it,
                mist_syntax::WalkEvent::Leave(_) => return None,
            };

            Some(match node_or_token {
                mist_syntax::NodeOrToken::Node(_) => return None,
                mist_syntax::NodeOrToken::Token(it) => {
                    let style = match it.kind() {
                        kind if kind.is_keyword() => DEFAULT_THEME.kw,
                        kind if kind.is_literal() => DEFAULT_THEME.literal,
                        kind if kind.is_punct() => DEFAULT_THEME.punct,
                        _ if ["fold", "unfold"].contains(&it.text()) => DEFAULT_THEME.kw,
                        SyntaxKind::WHITESPACE => return Some(it.to_string()),
                        _ => Style::default(),
                    };
                    as_latex_escaped(&[(style, &it.to_string())])
                }
            })
        })
        .collect()
}

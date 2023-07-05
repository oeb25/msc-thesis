use itertools::Itertools;
use mist_syntax::{
    ast::{self, SyntaxKind},
    AstNode,
};
use syntect::{
    highlighting::{FontStyle, Style},
    util::as_latex_escaped,
};

use crate::colors::PALETTE;

struct Theme {
    kw: Style,
    literal: Style,
    fun: Style,
    punct: Style,
    trivia: Style,
    block_label: Style,
}

const DEFAULT_THEME: Theme = Theme {
    kw: PALETTE.teal_600,
    literal: PALETTE.teal_500,
    fun: PALETTE.teal_800,
    punct: PALETTE.gray_400,
    trivia: PALETTE.gray_300,
    block_label: Style {
        font_style: FontStyle::ITALIC,
        ..PALETTE.gray_500
    },
};

#[derive(Debug, Default, Clone)]
pub struct HighlightingOptions {
    pub ignore_errors: bool,
    pub show_numbers: bool,
    pub viper_compat: bool,
}

impl HighlightingOptions {
    pub fn highlight(&self, code: &str) -> String {
        highlight_mist(code, self)
    }
}

fn highlight_mist(code: &str, opts: &HighlightingOptions) -> String {
    let s = highlight_mist_inner(code, 0, opts);
    if opts.viper_compat {
        s.replace("int", "Int")
            .replace("bool", "Bool")
            .replace("invariant", "predicate")
            .replace("{fn}", "{method}")
            .replace("{let}", "{var}")
    } else {
        s
    }
}
fn highlight_mist_inner(code: &str, line_offset: usize, opts: &HighlightingOptions) -> String {
    if let Some((fst, snd)) = code.split_once("\n//@ break\n") {
        if !opts.show_numbers {
            eprintln!("fst: {fst:?}");
            eprintln!("snd: {snd:?}");
        }

        let fst_latex = highlight_mist_inner(fst, line_offset, opts);
        let snd_latex = highlight_mist_inner(snd, line_offset + fst.lines().count() - 1, opts);
        format!("{fst_latex}\n\n{snd_latex}")
    } else if let Some((fst, snd)) = code.split_once("\n//@ align\n") {
        let l_max = fst.lines().map(|l| l.len()).max().unwrap_or_default();
        let r_max = snd.lines().map(|l| l.len()).max().unwrap_or_default();

        let target = if opts.show_numbers { 60 - 7 } else { 60 };
        let max = if l_max + r_max > target {
            l_max
        } else {
            l_max + (target - (l_max + r_max)) / 2
        };

        let fst_latex = highlight_actual_mist(fst, line_offset, opts);
        let snd_latex = highlight_actual_mist(snd, line_offset + fst.lines().count(), opts);

        let n_max_lines = fst
            .lines()
            .count()
            .max(fst_latex.lines().count().max(snd_latex.lines().count()));

        (0..n_max_lines)
            .map(|idx| {
                let fst_line = fst.lines().nth(idx).unwrap_or_default();
                let fst_latex = fst_latex.lines().nth(idx).unwrap_or_default();
                let snd_latex = snd_latex.lines().nth(idx).unwrap_or_default();

                let lgap = " ".repeat(max - fst_line.len() + 1);
                let rgap = if opts.show_numbers { "" } else { " " };

                format!(r"{fst_latex}{lgap}\textcolor{{Gray200}}{{|}}{rgap}{snd_latex}")
            })
            .join("\n")
    } else {
        highlight_actual_mist(code, line_offset, opts)
    }
}

fn highlight_actual_mist(code: &str, line_offset: usize, opts: &HighlightingOptions) -> String {
    let parse = mist_syntax::parse(code);
    if !opts.ignore_errors {
        for err in parse.errors() {
            eprintln!("{:?}", miette::Error::new(err.clone()));
        }
    }

    let mut block_label = false;

    let mut skip_subtree = None;

    let s: String = parse
        .tree()
        .syntax()
        .preorder_with_tokens()
        .filter_map(|event| {
            let node_or_token = match (event, skip_subtree) {
                (mist_syntax::WalkEvent::Enter(it), None) => it,
                (mist_syntax::WalkEvent::Enter(_), Some(depth)) => {
                    skip_subtree = Some(depth + 1);
                    return None;
                }
                (mist_syntax::WalkEvent::Leave(_), None | Some(0)) => {
                    skip_subtree = None;
                    return None;
                }
                (mist_syntax::WalkEvent::Leave(_), Some(depth)) => {
                    skip_subtree = Some(depth - 1);
                    return None;
                }
            };

            Some(match node_or_token {
                mist_syntax::NodeOrToken::Node(n) => {
                    let s = n.to_string();
                    let style = match n.kind() {
                        SyntaxKind::NAME | SyntaxKind::NAME_REF
                            if n.parent().is_some_and(|p| {
                                ast::Fn::can_cast(p.kind())
                                    || p.parent()
                                        .is_some_and(|gp| ast::CallExpr::can_cast(gp.kind()))
                            }) =>
                        {
                            skip_subtree = Some(0);
                            DEFAULT_THEME.fun
                        }
                        SyntaxKind::NAME | SyntaxKind::NAME_REF
                            if n.parent().is_some_and(|p| {
                                ast::Struct::can_cast(p.kind())
                                    || ast::StructExpr::can_cast(p.kind())
                                    || ast::NamedType::can_cast(p.kind())
                            }) =>
                        {
                            skip_subtree = Some(0);
                            DEFAULT_THEME.fun
                        }
                        _ => return None,
                    };
                    as_latex_escaped(&[(style, &s)])
                }
                mist_syntax::NodeOrToken::Token(it) => {
                    let s = it.to_string();
                    if block_label && s == ":" {
                        block_label = false;
                        return Some(as_latex_escaped(&[(DEFAULT_THEME.block_label, &s)]));
                    }
                    block_label = false;
                    let style = match it.kind() {
                        _ if s.starts_with('B') && s.ends_with(|c: char| c.is_numeric()) => {
                            block_label = true;
                            DEFAULT_THEME.block_label
                        }
                        kind if kind.is_keyword() => DEFAULT_THEME.kw,
                        kind if kind.is_literal() => DEFAULT_THEME.literal,
                        kind if kind.is_punct() => DEFAULT_THEME.punct,
                        SyntaxKind::WHITESPACE => return Some(s),
                        kind if kind.is_trivia() => {
                            let latex = as_latex_escaped(&[(DEFAULT_THEME.trivia, &s)]);
                            return Some(latex);
                        }
                        _ if ["fold", "unfold", "branch"].contains(&&*s) => DEFAULT_THEME.kw,
                        _ => Style::default(),
                    };
                    as_latex_escaped(&[(style, &s)])
                }
            })
        })
        .collect();
    let s = s.replace("\n}", "}\n");

    if opts.show_numbers {
        let line =
            |ln| format!(r"\textcolor{{Gray300}}{{\raisebox{{.1ex}}{{\footnotesize {ln:>2}}}}}");

        s.lines()
            .enumerate()
            .map(|(idx, l)| {
                let ln = line(line_offset + idx + 1);
                format!(r"{ln} {l}")
            })
            .join("\n")
    } else {
        s
    }
}

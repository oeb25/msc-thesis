mod colors;
mod highlight;

use std::{
    collections::HashMap,
    fmt::{self, Write},
    str::FromStr,
};

use itertools::Itertools;
use miette::{Context, IntoDiagnostic, Result};
use pandoc_types::definition::{Attr, Block, Format, Inline};
use ungrammar::NodeData;

use crate::{colors::Pallete, highlight::highlight_mist};

fn main() -> Result<()> {
    miette::set_panic_hook();

    let stdin = std::io::stdin();
    let input_data = stdin.lock();

    let mut doc: pandoc_types::definition::Pandoc = serde_json::from_reader(input_data)
        .into_diagnostic()
        .wrap_err("could not create pandoc document")?;

    let mut focus = vec![];

    for b in &mut doc.blocks {
        for e in walk_block(b)? {
            match e {
                Event::Subfigure => {}
                Event::Focus(new_b) => {
                    focus.push(new_b);
                }
            }
        }
    }

    if !focus.is_empty() {
        doc.blocks = focus;
    }

    doc.blocks.insert(
        0,
        Block::latex(
            Pallete::iter()
                .map(|(name, style)| {
                    let r = (style.foreground.r as f32) / (u8::MAX as f32);
                    let g = (style.foreground.g as f32) / (u8::MAX as f32);
                    let b = (style.foreground.b as f32) / (u8::MAX as f32);
                    format!(
                        r"\definecolor{{{}}}{{rgb}} {{{r},{b},{g}}}",
                        heck::AsPascalCase(name)
                    )
                })
                .format(""),
        ),
    );

    println!("{}", serde_json::to_string(&doc).into_diagnostic()?);

    Ok(())
}

enum Event {
    Subfigure,
    Focus(Block),
}

fn attr_map(attrs: &[(String, String)]) -> HashMap<&str, &str> {
    attrs
        .iter()
        .map(|(k, v)| (k.as_str(), v.as_str()))
        .collect()
}

trait FormatExt {
    fn latex(content: impl fmt::Display) -> Self;
}

impl FormatExt for Block {
    fn latex(content: impl fmt::Display) -> Block {
        Block::RawBlock(Format("latex".to_string()), content.to_string())
    }
}
impl FormatExt for Inline {
    fn latex(content: impl fmt::Display) -> Inline {
        Inline::RawInline(Format("latex".to_string()), content.to_string())
    }
}

macro_rules! raw_latex {
    ($($t:tt)*) => {
        FormatExt::latex(format!($($t)*))
    };
}

fn walk_block(b: &mut Block) -> Result<Vec<Event>, miette::ErrReport> {
    let mut events = vec![];

    match b {
        Block::CodeBlock(attr, src) => {
            if attr.classes.iter().any(|s| s == "ungram") {
                let show_names = !attr.classes.iter().any(|s| s == "noNames");
                let (label, src) = extract_label_and_id(attr, src);
                let lines = wrap_latex_in_figure(
                    &fmt_ungrammar(src, show_names).wrap_err("formatting ungrammar")?,
                    label,
                );
                *b = Block::latex(lines)
            } else if attr.classes.iter().any(|s| s == "mist") {
                let ignore_errors = attr.classes.iter().any(|s| s == "ignoreErrors");
                let number_lines = attr.classes.iter().any(|s| s == "numberLines");
                let (label, src) = extract_label_and_id(attr, src);
                let lines = highlight_mist(src, ignore_errors);
                *b = Block::latex(generate_latex_code_block(&lines, number_lines, label))
            } else if attr.classes.iter().any(|s| s == "folding-tree") {
                let root = attr_map(&attr.attributes).get("root").copied();
                let (label, src) = extract_label_and_id(attr, src);
                let latex = wrap_latex_in_figure(&fmt_folding_tree(root, src)?, label);
                *b = Block::latex(latex)
            }
        }
        Block::Plain(_) => {}
        Block::Para(_) => {}
        Block::LineBlock(_) => {}
        Block::RawBlock(_, _) => {}
        Block::BlockQuote(_) => {}
        Block::OrderedList(_, _) => {}
        Block::BulletList(_) => {}
        Block::DefinitionList(_) => {}
        Block::Header(_, _, _) => {}
        Block::HorizontalRule => {}
        Block::Table(_) => {}
        Block::Div(attr, content) => {
            if attr.classes.iter().any(|s| s == "figure") {
                content.insert(0, Block::latex(r"\begin{figure}[H]\centering"));

                let label = if !attr.identifier.is_empty() {
                    format!(r"\label{{{}}}", attr.identifier)
                } else {
                    String::new()
                };

                content.push(raw_latex!(r"{label}\end{{figure}}"));
            } else if attr.classes.iter().any(|s| s == "subfigure") {
                events.push(Event::Subfigure);

                let atributes = attr_map(&attr.attributes);
                let alignment = atributes.get("align").unwrap_or(&"t");
                let subfigure_attrs = atributes.get("width").unwrap_or(&"0.2");

                let tail = if atributes.get("newline") == Some(&"true") {
                    ""
                } else {
                    r"\@ifnextchar\par{\@gobble}{}"
                };

                content.insert(
                    0,
                    raw_latex!(
                        r"
                            \begin{{subfigure}}[{alignment}]{{{subfigure_attrs}\textwidth}}
                                \vskip 0pt
                                \centering
                            "
                    ),
                );
                content.push(raw_latex!(r"\end{{subfigure}}{tail}"));
            } else if attr.classes.iter().any(|s| s == "lemma") {
                content.insert(0, Block::latex(r"\begin{lemma}"));

                let label = if !attr.identifier.is_empty() {
                    format!(r"\label{{{}}}", attr.identifier)
                } else {
                    String::new()
                };

                content.push(raw_latex!(r"{label}\end{{lemma}}"));
            } else if attr.classes.iter().any(|s| s == "proof") {
                content.insert(
                    0,
                    Block::latex(r"\begin{proof}\hspace{1em}\@ifnextchar\par{\@gobble}{}"),
                );

                append_to_end(content.last_mut().unwrap(), r"\phantom{}\qed");

                content.push(Block::latex(r"\end{proof}"));
            } else if attr.classes.iter().any(|s| s == "definition") {
                content.insert(0, Block::latex(r"\begin{definition}"));

                let label = if !attr.identifier.is_empty() {
                    format!(r"\label{{{}}}", attr.identifier)
                } else {
                    String::new()
                };

                content.push(raw_latex!(r"{label}\end{{definition}}"));
            } else if attr.classes.iter().any(|s| s == "focus") {
                let mut new_b = content[0].clone();
                walk_block(&mut new_b)?;
                *b = new_b.clone();
                events.push(Event::Focus(new_b));
                return Ok(events);
            } else {
                dbg!(&attr);
            }

            let mut subfigures = vec![];

            for (idx, b) in content.iter_mut().enumerate() {
                for event in walk_block(b)? {
                    match event {
                        Event::Subfigure => subfigures.push(idx),
                        Event::Focus(new_b) => {
                            events.push(Event::Focus(new_b.clone()));
                            *b = new_b;
                            break;
                        }
                    }
                }
            }
        }
        Block::Null => {}
    }

    Ok(events)
}

fn append_to_end(b: &mut Block, content: impl fmt::Display) {
    match b {
        Block::Para(xs) => xs.push(raw_latex!("{content}")),
        b => todo!("append_to_end of {b:?}"),
    }
}

fn extract_label_and_id<'a>(attr: &'a Attr, src: &'a str) -> (Option<(&'a str, &'a str)>, &'a str) {
    if !attr.identifier.is_empty() {
        let (caption, code) = if src.starts_with("//") {
            let (caption, code) = src.split_once('\n').unwrap();
            (caption.trim_start_matches([' ', '/']), code)
        } else {
            ("", src)
        };
        (Some((attr.identifier.as_str(), caption)), code)
    } else {
        (None, src)
    }
}

fn fmt_ungrammar(src: &str, show_names: bool) -> Result<String> {
    let mut buf = String::new();

    let grammar = ungrammar::Grammar::from_str(src)
        .into_diagnostic()
        .wrap_err("parsing ungrammar source")?;

    let longest_node = grammar
        .iter()
        .map(|n| grammar[n].name.as_str())
        .max_by_key(|n| n.len())
        .unwrap_or("");

    writeln!(
        buf,
        r"\setlength{{\ungrammarl}}{{\widthof{{<{longest_node}>}}+2em}}"
    )
    .into_diagnostic()?;
    writeln!(buf, r"\grammarindent\the\ungrammarl").into_diagnostic()?;

    writeln!(buf, r"\begin{{grammar}}").into_diagnostic()?;
    for n in grammar.iter() {
        let NodeData { name, rule } = &grammar[n];

        if name.ends_with('_') {
            continue;
        }

        writeln!(buf, r"<{}> ::= ", name.trim_end_matches('_')).into_diagnostic()?;

        let mut fmtr = Formatter {
            grammar: &grammar,
            buf: &mut buf,
            show_names,
        };
        fmtr.fmt_rule(rule, false).into_diagnostic()?;

        writeln!(buf).into_diagnostic()?;
        writeln!(buf).into_diagnostic()?;
    }
    writeln!(buf, r"\end{{grammar}}").into_diagnostic()?;

    struct Formatter<'a> {
        grammar: &'a ungrammar::Grammar,
        buf: &'a mut String,
        show_names: bool,
    }

    impl Formatter<'_> {
        fn fmt_rule(&mut self, r: &ungrammar::Rule, nested: bool) -> fmt::Result {
            match r {
                ungrammar::Rule::Labeled { label, rule } => {
                    if self.show_names {
                        write!(self.buf, "{label}:")?;
                    }
                    self.fmt_rule(rule, true)?;
                }
                ungrammar::Rule::Node(n) => write!(
                    self.buf,
                    "<{}>",
                    self.grammar[*n].name.trim_end_matches('_')
                )?,
                ungrammar::Rule::Token(t) => write!(self.buf, "`{}'", self.grammar[*t].name)?,
                ungrammar::Rule::Seq(rs) => {
                    if nested {
                        write!(self.buf, "(")?;
                    }
                    for r in rs {
                        write!(self.buf, " ")?;
                        self.fmt_rule(r, true)?;
                    }
                    if nested {
                        write!(self.buf, ")")?;
                    }
                }
                ungrammar::Rule::Alt(alts) => {
                    if nested {
                        write!(self.buf, "(")?;
                    }
                    let mut first = true;
                    for r in alts {
                        if !first {
                            if nested || alts.len() <= 2 {
                                write!(self.buf, r" | ")?;
                            } else {
                                write!(self.buf, r" \alt ")?;
                            }
                        }
                        first = false;
                        self.fmt_rule(r, true)?;
                    }
                    if nested {
                        write!(self.buf, ")")?;
                    }
                }
                ungrammar::Rule::Opt(r) => {
                    self.fmt_rule(r, true)?;
                    write!(self.buf, "?")?
                }
                ungrammar::Rule::Rep(r) => {
                    self.fmt_rule(r, true)?;
                    write!(self.buf, "*")?
                }
            }
            Ok(())
        }
    }

    Ok(buf)
}

fn fmt_folding_tree(root: Option<&str>, src: &str) -> Result<String> {
    let ft = folding_tree::FoldingTree::try_from(src).into_diagnostic()?;

    // eprintln!("{src} => {ft}");

    let mut buf = String::new();

    for event in ft.preorder() {
        match event {
            folding_tree::WalkEvent::Enter((edge, _folding)) => if let Some(edge) = edge {
                write!(
                    buf,
                    "[${edge}$,rounded corners=0mm,color=Teal900,edge=Teal600,edge=thick "
                )
            } else if let Some(root) = root {
                write!(buf, "[{root} ")
            } else {
                write!(buf, "[,minimum size=0.1mm,circle,fill=Teal600 ")
            }
            .into_diagnostic()?,
            folding_tree::WalkEvent::Leave((_, folding)) => match folding {
                folding_tree::Folding::Uninitialized
                | folding_tree::Folding::Folded
                | folding_tree::Folding::Unfolded => write!(buf, "]").into_diagnostic()?,
            },
        }
    }

    Ok(format!(
        r#"
        \begin{{forest}}
            forked edges,
            {buf}
        \end{{forest}}
        "#
    ))
}

fn generate_latex_code_block(
    lines: &str,
    numbers: bool,
    label_and_caption: Option<(&str, &str)>,
) -> String {
    let latex = format!(
        r"\makeatletter
\def\verbatim@nolig@list{{\do\`\do\<\do\>\do\'\do\-}}
\makeatother
% ...
\begin{{Verbatim}}[commandchars=\\\{{\}}{}]
{lines}
\end{{Verbatim}}",
        if numbers { ",numbers=left" } else { "" }
    );
    wrap_latex_in_figure(&latex, label_and_caption)
}

fn wrap_latex_in_figure(latex: &str, label_and_caption: Option<(&str, &str)>) -> String {
    if let Some((label, caption)) = label_and_caption {
        format!(
            r"\begin{{figure}}[H]
\centering
{latex}
\caption{{{caption}}}
\label{{{label}}}
\end{{figure}}"
        )
    } else {
        latex.to_string()
    }
}

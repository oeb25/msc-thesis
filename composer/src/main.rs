mod canvas;

use std::{
    collections::{HashMap, VecDeque},
    fs, io,
};

use camino::{Utf8Path, Utf8PathBuf};
use canvas::Canvas;
use clap::Parser;
use itertools::{Either, Itertools};
use miette::{bail, miette, Context, IntoDiagnostic, Result};
use regex::Regex;
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

use crate::canvas::Node;

#[derive(Debug, clap::Parser)]
struct Cli {
    #[clap(subcommand)]
    cmd: Cmd,
}

#[derive(Debug, clap::Subcommand)]
enum Cmd {
    Build {
        #[clap(short, long)]
        output: Option<Utf8PathBuf>,
        entry: Utf8PathBuf,
    },
}

#[derive(Debug, Default, Clone)]
struct Database {
    base_path: Utf8PathBuf,
    entry: Utf8PathBuf,
    files: HashMap<Utf8PathBuf, (File, FileKindKind)>,
    resolutions: HashMap<String, Result<Utf8PathBuf, ()>>,
    queue: Vec<Utf8PathBuf>,
    processed: HashMap<Utf8PathBuf, ProcessedFile>,
}

#[derive(Debug, Clone)]
struct File {
    #[allow(unused)]
    path: Utf8PathBuf,
    kind: FileKind,
}

#[derive(Debug, Clone)]
enum FileKind {
    Markdown {
        frontmatter: Option<Frontmatter>,
        content: String,
    },
    Canvas(Canvas),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct Frontmatter {
    #[serde(rename = "tags")]
    ty: String,
    title: Option<String>,
    #[serde(default)]
    attrs: HashMap<String, serde_json::Value>,
    #[serde(flatten)]
    rest: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FileKindKind {
    Markdown,
    Canvas,
    Citation,
}

#[derive(Debug, Clone)]
struct ProcessedFile {
    content: Vec<ProcessedFileContent>,
}

#[derive(Debug, Clone)]
enum ProcessedFileContent {
    RawLatex(String),
    Markdown(String),
    Embed(Utf8PathBuf),
    Reference(Utf8PathBuf),
    Cite {
        keys: Vec<String>,
        pages: Option<String>,
    },
}

impl Database {
    fn new(base_path: Utf8PathBuf, entry: Utf8PathBuf) -> Database {
        Database {
            base_path,
            entry,
            files: Default::default(),
            resolutions: Default::default(),
            queue: Default::default(),
            processed: Default::default(),
        }
    }

    fn ingest(
        &mut self,
        relative_to: Option<&Utf8Path>,
        path: impl AsRef<Utf8Path>,
    ) -> Result<(Utf8PathBuf, FileKindKind)> {
        let path = match relative_to {
            Some(base) => base.join(path),
            None => self.base_path.join(path),
        };
        let path = path
            .canonicalize_utf8()
            .into_diagnostic()
            .wrap_err_with(|| format!("could not canonicalize '{path}'"))?;

        if self.files.contains_key(&path) {
            let kind_kind = self.files.get(&path).unwrap().1;
            return Ok((path, kind_kind));
        }

        let content = fs::read_to_string(&path)
            .into_diagnostic()
            .wrap_err_with(|| format!("failed to read '{path}'"))?;

        let (kind, kind_kind) = match path.extension() {
            Some("md") => {
                let (frontmatter, content) = if let Some(rest) = content.strip_prefix("---\n") {
                    let (frontmatter, rest) = rest.split_once("---\n").ok_or_else(|| {
                        miette!("failed to find end of frontmatter of '{}'", path)
                    })?;
                    let frontmatter: Frontmatter = serde_yaml::from_str(frontmatter)
                        .into_diagnostic()
                        .wrap_err_with(|| {
                            miette!("failed to parse frontmatter yaml of '{}'", path)
                        })?;
                    (Some(frontmatter), rest.to_string())
                } else {
                    (None, content)
                };
                let kind_kind = if frontmatter.as_ref().is_some_and(|f| f.ty == "citation") {
                    FileKindKind::Citation
                } else {
                    FileKindKind::Markdown
                };
                (
                    FileKind::Markdown {
                        frontmatter,
                        content,
                    },
                    kind_kind,
                )
            }
            Some("canvas") => {
                let c = serde_json::from_str(&content)
                    .into_diagnostic()
                    .wrap_err_with(|| format!("failed to parse canvas from '{path}'"))?;
                (FileKind::Canvas(c), FileKindKind::Canvas)
            }
            _ => bail!("unrecognized extension of '{:?}'", path),
        };

        let f = File {
            path: path.clone(),
            kind,
        };
        self.files.insert(path.clone(), (f, kind_kind));
        self.queue.push(path.clone());

        Ok((path, kind_kind))
    }

    fn resolve(&mut self, p: &str) -> Result<Utf8PathBuf> {
        if let Some(r) = self.resolutions.get(p).cloned() {
            return r.map_err(|()| miette!("failed to resolve '{}'", p));
        }

        let res = WalkDir::new(&self.base_path).into_iter().find_map(|e| {
            let e = e.unwrap();
            if p == e.path().file_stem().unwrap() {
                Some(Utf8PathBuf::try_from(e.path().to_path_buf()).unwrap())
            } else {
                None
            }
        });

        match res {
            Some(r) => {
                let resolved = r.canonicalize_utf8().into_diagnostic().wrap_err_with(|| {
                    miette!("failed to canonicalize '{r}' during resolution of '{}'", p)
                })?;
                self.resolutions.insert(p.to_string(), Ok(resolved.clone()));
                Ok(resolved)
            }
            None => {
                self.resolutions.insert(p.to_string(), Err(()));
                Err(miette!("failed to resolve '{}'", p))
            }
        }
    }

    fn work(&mut self) -> Result<()> {
        while let Some(f) = self.queue.pop() {
            if self.processed.contains_key(&f) {
                bail!("file already processed: '{f}'");
            }

            let pf = match self.files[&f].0.kind.clone() {
                FileKind::Markdown {
                    frontmatter: Some(Frontmatter { ty, .. }),
                    content,
                } if ty == "outline" => {
                    let outline = content
                        .trim()
                        .lines()
                        .map(|l| {
                            if let Some((tabs, link)) = l.split_once("- ") {
                                let _indentation = tabs.len();
                                let name = link.trim_matches(|c| "[]".contains(c));
                                let path = self.resolve(name)?;
                                let (_, _kind_kind) = self.ingest(None, &path)?;

                                Ok(ProcessedFileContent::Embed(path))
                            } else {
                                todo!("{l}")
                            }
                        })
                        .collect::<Result<_>>()?;

                    ProcessedFile { content: outline }
                }
                FileKind::Markdown {
                    frontmatter,
                    content,
                } => {
                    let mut p_content = vec![];
                    let mut tail_content = vec![];

                    let file_name = f.file_stem().unwrap();
                    let file_label = format!("{}", heck::AsKebabCase(file_name));
                    let title = if let Some(Frontmatter {
                        title: Some(title), ..
                    }) = &frontmatter
                    {
                        title.to_string()
                    } else {
                        file_name.to_string()
                    };

                    match frontmatter {
                        Some(Frontmatter { ty, .. }) if ty == "figure" => {
                            let head = format!("\n::: {{.figure #{file_label}}}\n");
                            p_content.push(ProcessedFileContent::Markdown(head));

                            // let caption = rest
                            //     .get("caption")
                            //     .cloned()
                            //     .unwrap_or_else(|| file_name.to_string());

                            // let tail = format!("\n\\caption{{{caption}}}\n\n:::\n");
                            // tail_content.push(ProcessedFileContent::Markdown(tail));

                            let tail = "\n\n:::\n".to_string();
                            tail_content.push(ProcessedFileContent::Markdown(tail));
                        }
                        Some(Frontmatter { ty, attrs, .. }) if ty == "subfigure" => {
                            let attrs = attrs.iter().map(|(k, v)| format!("{k}={v}")).format(" ");
                            let head = format!("\n::::: {{.subfigure #{file_label} {attrs}}}\n");
                            p_content.push(ProcessedFileContent::Markdown(head));

                            let tail = "\n\n:::::\n".to_string();
                            tail_content.push(ProcessedFileContent::Markdown(tail));
                        }
                        Some(Frontmatter { ty, .. }) if ty == "chapter" => {
                            p_content.push(ProcessedFileContent::Markdown(format!(
                                "\n# {title} {{#{file_label}}}"
                            )));
                        }
                        Some(Frontmatter { ty, .. }) if ty == "section" => {
                            p_content.push(ProcessedFileContent::Markdown(format!(
                                "\n## {title} {{#{file_label}}}"
                            )));
                        }
                        Some(Frontmatter { ty, .. }) if ty == "subsection" => {
                            p_content.push(ProcessedFileContent::Markdown(format!(
                                "\n### {title} {{#{file_label}}}"
                            )));
                        }
                        Some(Frontmatter { ty, .. }) if ty == "subsubsection" => {
                            p_content.push(ProcessedFileContent::Markdown(format!(
                                "\n#### {title} {{#{file_label}}}"
                            )));
                        }
                        Some(Frontmatter { ty, .. }) if ty == "paragraph" => {
                            p_content.push(ProcessedFileContent::Markdown(format!(
                                "\n\\paragraph{{{title}}}"
                            )));
                        }
                        Some(Frontmatter { ty, .. }) if ty == "appendix" => {
                            let stripped_file_name = file_name
                                .split_once(" – ")
                                .map(|(_, n)| n)
                                .unwrap_or(file_name);
                            p_content.push(ProcessedFileContent::Markdown(format!(
                                "\n# {stripped_file_name} {{#{file_label}}}"
                            )));
                        }
                        Some(Frontmatter { ty, .. }) if ty == "citation" => {
                            p_content.clear();
                        }
                        Some(Frontmatter { ty, .. }) if ty == "abstract" => {
                            p_content.push(ProcessedFileContent::RawLatex(
                                r"\chapter*{\centering Abstract}".to_string(),
                            ));
                        }
                        Some(Frontmatter { ty, .. }) if ty == "acknowledgements" => {
                            p_content.push(ProcessedFileContent::RawLatex(
                                r"\chapter*{\centering Acknowledgements}".to_string(),
                            ));
                        }
                        Some(Frontmatter { ty, .. })
                            if [
                                "definition",
                                "lemma",
                                "proposition",
                                "theorem",
                                "proof",
                                "example",
                            ]
                            .contains(&ty.as_str()) =>
                        {
                            p_content.push(ProcessedFileContent::RawLatex(format!(
                                r"\begin{{{ty}}}\label{{{file_label}}} \@ifnextchar\par{{\@gobble}}{{}}"
                            )));
                            tail_content
                                .push(ProcessedFileContent::RawLatex(format!(r"\end{{{ty}}}")));
                        }
                        Some(Frontmatter { ty, .. }) => todo!("{ty}"),
                        _ => {}
                    }

                    let mut content = content;
                    while let Some((a, bc)) = content.split_once("%%") {
                        if let Some((_, c)) = bc.split_once("%%") {
                            content = a.to_string() + c;
                        } else {
                            break;
                        }
                    }

                    // NOTE: Process references and embeds
                    let regex = Regex::new(r"(!?)\[\[([^\]]+)\]\]").unwrap();
                    let mut just_cited = false;

                    for m in regex.split(&content).map(Either::Left).interleave(
                        regex.captures_iter(&content).map(|c| {
                            let (_, [embed, inner]) = c.extract();
                            let embed = !embed.is_empty();
                            Either::Right((embed, inner))
                        }),
                    ) {
                        match m {
                            Either::Left(md) => {
                                just_cited = just_cited && md.chars().all(|c| c == ' ');
                                p_content.push(ProcessedFileContent::Markdown(md.to_string()));
                            }
                            Either::Right((embed, inner)) => {
                                let (file, heading) =
                                    if let Some((file, heading)) = inner.split_once('#') {
                                        (file, Some(heading))
                                    } else {
                                        (inner, None)
                                    };

                                let path = self.resolve(file)?;
                                let (_, kind_kind) = self.ingest(None, &path)?;

                                if embed {
                                    just_cited = false;
                                    p_content.push(ProcessedFileContent::Embed(path))
                                } else {
                                    match kind_kind {
                                        FileKindKind::Markdown | FileKindKind::Canvas => {
                                            just_cited = false;
                                            p_content.push(ProcessedFileContent::Reference(path))
                                        }
                                        FileKindKind::Citation => {
                                            let key = path.file_stem().unwrap().to_string();
                                            let pages = heading.map(|h| h.to_string());
                                            if just_cited {
                                                while let Some(c) = p_content.last_mut() {
                                                    match c {
                                                        ProcessedFileContent::Markdown(_) => {
                                                            p_content.pop();
                                                        }
                                                        ProcessedFileContent::Cite {
                                                            keys,
                                                            pages: prev_pages,
                                                        } => {
                                                            if prev_pages.is_none() {
                                                                *prev_pages = pages;
                                                            }
                                                            keys.push(key);
                                                            break;
                                                        }
                                                        ProcessedFileContent::RawLatex(_)
                                                        | ProcessedFileContent::Embed(_)
                                                        | ProcessedFileContent::Reference(_) => {
                                                            todo!()
                                                        }
                                                    }
                                                }
                                            } else {
                                                p_content.push(ProcessedFileContent::Cite {
                                                    keys: vec![key],
                                                    pages,
                                                })
                                            }
                                            just_cited = true;
                                        }
                                    }
                                }
                            }
                        }
                    }

                    p_content.extend_from_slice(&tail_content);
                    ProcessedFile { content: p_content }
                }
                FileKind::Canvas(c) => {
                    let mut g = petgraph::Graph::new();

                    let node_map: HashMap<_, _> = c
                        .nodes
                        .iter()
                        .map(|n| (n.id(), g.add_node(n.clone())))
                        .collect();

                    for e in &c.edges {
                        let from = node_map[&e.from_node];
                        let to = node_map[&e.to_node];
                        g.add_edge(from, to, ());
                    }

                    let root = node_map
                        .iter()
                        .min_by_key(|n| {
                            g.neighbors_directed(*n.1, petgraph::Direction::Incoming)
                                .count()
                        })
                        .unwrap();

                    let mut content = vec![];

                    let mut dfs = petgraph::visit::Dfs::new(&g, *root.1);
                    while let Some(e) = dfs.next(&g) {
                        match g.node_weight(e).unwrap() {
                            Node::File { file, .. } => {
                                let (child, _) = self.ingest(None, file)?;
                                content.push(ProcessedFileContent::Embed(child));
                            }
                            Node::Group { .. } => todo!(),
                            Node::Text { residue, .. } => todo!("{residue:?}"),
                        }
                    }
                    ProcessedFile { content }
                }
            };
            self.processed.insert(f, pf);
        }

        Ok(())
    }

    fn output(&mut self, mut buf: impl io::Write) -> Result<()> {
        let mut queue = VecDeque::new();

        queue.push_back((0, self.entry.clone()));

        while let Some((pos, f)) = queue.pop_back() {
            let pf = self
                .processed
                .get(&f)
                .ok_or_else(|| miette!("file '{}' has not been processed yet", f))?;

            if pos == pf.content.len() {
                continue;
            }

            for (idx, c) in pf.content.iter().enumerate().skip(pos) {
                match c {
                    ProcessedFileContent::Markdown(md) => {
                        let md = md.replace("#todo", "Todo");
                        write!(buf, "{md}").into_diagnostic()?
                    }
                    ProcessedFileContent::RawLatex(tex) => {
                        writeln!(buf, "\n```{{=tex}}\n{tex}\n```").into_diagnostic()?
                    }
                    ProcessedFileContent::Embed(p) => {
                        writeln!(buf).into_diagnostic()?;
                        queue.push_back((idx + 1, f));
                        queue.push_back((0, p.clone()));
                        break;
                    }
                    ProcessedFileContent::Reference(p) => {
                        let label = heck::AsKebabCase(p.file_stem().unwrap());
                        write!(buf, r"\cref{{{label}}}").into_diagnostic()?;
                    }
                    ProcessedFileContent::Cite { keys, pages } => {
                        let key = keys.iter().map(|k| k.trim_start_matches('@')).join(", ");
                        let pages = pages.as_ref().map(|s| s.as_str()).unwrap_or_default();
                        write!(buf, r"\cite[{pages}]{{{key}}}").into_diagnostic()?;
                        // if let Some(pages) = pages {
                        //     write!(buf, r"[see {key} {pages}]")
                        // } else {
                        //     write!(buf, r"{key}")
                        // }
                        // .into_diagnostic()?;
                    }
                }
            }
        }

        Ok(())
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.cmd {
        Cmd::Build { output, entry } => {
            let base_path = entry.parent().ok_or_else(|| {
                miette!("failed to determine base directory from parent of '{entry}'")
            })?;

            let mut db = Database::new(
                base_path.to_path_buf(),
                entry
                    .canonicalize_utf8()
                    .into_diagnostic()
                    .wrap_err_with(|| miette!("failed to canonicalize '{}'", entry))?,
            );

            db.ingest(None, entry.file_name().unwrap())
                .wrap_err_with(|| format!("failed to ingest '{entry}'"))?;

            db.work()?;

            match output {
                Some(path) => {
                    db.output(
                        fs::File::create(path)
                            .into_diagnostic()
                            .wrap_err_with(|| miette!("failed to create file 'path'"))?,
                    )
                    .wrap_err("failed to write output")?;
                }
                None => {
                    db.output(&mut io::stdout().lock())
                        .wrap_err("failed to write output")?;
                }
            }
        }
    }

    Ok(())
}

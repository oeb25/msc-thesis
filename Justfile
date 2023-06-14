# Replace this with a specific order if you want, like: "src/00-prelude.md src/02-introduction.md"
SOURCES := "full.md"
# SOURCES := "/Users/oeb25/Projects/thesis/obsidian/Mist/Folding\\ tree\\ structure.md"
# SOURCES := "src/chapters/*.md"
FINAL_PDF := "thesis.pdf"

[private]
@default:
    -just zotero
    devenv shell -- just write

write: generate-pdf
    open -a skim {{FINAL_PDF}} || open {{FINAL_PDF}}
    @watchexec -d 200 -rc -i full.md -e md,tex,sty,bib,rs -- just generate-pdf

zotero:
    curl -fs 'http://127.0.0.1:23119/better-bibtex/export/library?/1/library.biblatex' -o References.bib

compile-filters:
    cargo build --bin thesis-filters --release

generate-pdf:
    @just FINAL_PDF="debug.tex" pandoc
    clear
    @just pandoc

pandoc: compile-filters
    cargo run --bin composer -- build Mist/Outline.md > full.md
    pandoc \
        --template ./thesis-template-2015/Thesis.tex \
        --number-sections \
        --filter thesis-filters \
        -M secnos-warning-level=1 \
        --filter pandoc-secnos \
        --citeproc \
        --pdf-engine xelatex \
        src/prelude.md \
        {{SOURCES}} \
        src/references.md \
        -o {{FINAL_PDF}}

debug-latex:
    @just FINAL_PDF="debug.tex" pandoc

# Replace this with a specific order if you want, like: "src/00-prelude.md src/02-introduction.md"
SOURCES := "src/chapters/*.md"
APPENDIX := "src/appendix/*.md"
FINAL_PDF := "thesis.pdf"

write: generate-pdf
    open -a skim {{FINAL_PDF}}
    @watchexec -e md,tex,sty,bib -- just generate-pdf

generate-pdf:
    pandoc \
        --template ./thesis-template-2015/Thesis.tex \
        --number-sections \
        -M secnos-warning-level=1 \
        --filter pandoc-secnos \
        --citeproc \
        src/prelude.md \
        {{SOURCES}} \
        src/appendix.md \
        {{APPENDIX}} \
        src/references.md \
        -o {{FINAL_PDF}}

debug-latex:
    @just FINAL_PDF="debug.tex" generate-pdf

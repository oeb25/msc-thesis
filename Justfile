# Replace this with a specific order if you want, like: "src/00-prelude.md src/02-introduction.md"
SOURCES := "src/*.md"
APPENDIX := "src/appendix/*.md"
FINAL_PDF := "thesis.pdf"

write: generate-pdf
    open -a skim {{FINAL_PDF}}
    @watchexec -e md,tex,bib -- just generate-pdf

generate-pdf:
    pandoc \
        --template ./thesis-template-2015/Thesis.tex \
        --citeproc \
        {{SOURCES}} \
        {{APPENDIX}} \
        -o {{FINAL_PDF}}

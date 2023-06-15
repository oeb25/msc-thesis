---
title: The Mist Programming Langauge
author: Oliver Bøving
toc: true
date: 15th July
year: 2023
bibliography: [ZoteroLib.bib]
citeproc: true
classoption:
    - 10pt
    - twoside
documentclass: book
keywords: LaTeX, Pandoc
header-includes:
    - \usepackage{standalone}
    - \usepackage{pdfpages}
    - \usepackage{chngpage}
    - \usepackage{bussproofs}
    - \newcommand{\AXC}[1]{\AxiomC{#1}}
    - \newcommand{\UIC}[1]{\UnaryInfC{#1}}
    - \newcommand{\BIC}[1]{\BinaryInfC{#1}}
    - \newcommand{\TIC}[1]{\TrinaryInfC{#1}}
    - \usepackage{syntax}
    - \usepackage{mathabx}
    - \usepackage{stmaryrd}
    - \usepackage{fancyvrb,fontspec,xcolor}
    - \setmonofont[Contextuals={Alternate},Scale=MatchLowercase]{FiraCode Nerd Font}
    - \newlength{\ungrammarl}
    - \usepackage{lscape}
    - \usepackage[edges]{forest}
    - \usetikzlibrary{chains,shadows.blur}
    - \usepackage{float}
    - \usepackage{caption}
    - \usepackage{subcaption}
    - \usepackage{tabularx}
    # Custom envs
    - \newtheorem{case}{Case}
    # Common notation
    - \newcommand{\lineref}[1]{{\footnotesize (line {#1})}}
    - \newcommand{\Rho}{\mathrm{P}}
    # - \newcommand{\leafin}[2]{#1\;\varepsilon\;#2}
    - \newcommand{\leaves}{\mathcal{L}}
    - \newcommand{\leafin}[2]{#1 \in \leaves(#2)}
    - \newcommand{\requires}{\reflectbox{$\mathrel{\leadsto}$}}
    # Function aliases
    - \newcommand{\fold}{\texttt{fold}}
    - \newcommand{\unfold}{\texttt{unfold}}
    - \newcommand{\prefix}{\texttt{prefix}}
    - \newcommand{\parents}{\texttt{parents}}
    - \newcommand{\fields}{\texttt{fields}}
    - \newcommand{\cut}{\texttt{cut}}
    - \newcommand{\T}{\mathcal{T}}
    - \newcommand{\Ts}{\mathbf{T}}
    - \newcommand{\slots}{\mathbf{Slots}}
    - \newcommand{\places}{\mathbf{Places}}
    - \newcommand{\A}{\mathbf{A}}
    - \newcommand{\inst}{\iota}
    # - \newcommand{\tinto}{\texttt{transitionInto}}
    - \newcommand{\tinto}{\ensuremath{\Delta}}
    # Lattice operators
    - \newcommand{\smaller}{\sqsubseteq}
    - \newcommand{\larger}{\sqsupseteq}
    - \newcommand{\join}{\sqcup}
    - \newcommand{\meet}{\sqcap}
    # Semantics
    - \newcommand{\sem}[1]{\mathcal{S}\llbracket#1 \rrbracket}
    - \newcommand{\fsem}[1]{\widecheck{\mathcal{S}}\llbracket#1 \rrbracket}
    - \newcommand{\bsem}[1]{\widehat{\mathcal{S}}\llbracket#1 \rrbracket}
    - \newcommand{\ssem}[1]{\overline{\mathcal{S}}\llbracket#1 \rrbracket}
    - \newcommand{\dom}{\texttt{dom}}
    - \usepackage{varwidth}
    - \renewenvironment{prooftree}{\varwidth{.9\textwidth}\centering\leavevmode}{\DisplayProof\endvarwidth}

    - \usepackage{framed}

    - \renewenvironment{leftbar}[2][\hsize] { \def\FrameCommand { {\color{#2}\vrule width 3pt} \hspace{0pt} } \MakeFramed{\hsize#1\advance\hsize-\width\FrameRestore} } {\endMakeFramed}

    - \newenvironment{lemma}{\begin{leftbar}{Orange300}\begin{ilemma}}{\end{ilemma}\end{leftbar}}
    - \newenvironment{theorem}{\begin{leftbar}{Emerald500}\begin{itheorem}}{\end{itheorem}\end{leftbar}}
    - \newenvironment{definition}{\begin{leftbar}{Teal500}\begin{idefinition}}{\end{idefinition}\end{leftbar}}
    - \renewenvironment{proof}{\begin{leftbar}{Slate200}\begin{iproof}}{\end{iproof}\end{leftbar}}
    - \newenvironment{example}{\begin{leftbar}{Slate600}\begin{iexample}\normalfont}{\end{iexample}\end{leftbar}}
    - \newenvironment{remark}{\begin{leftbar}{Green300}\begin{iremark}\normalfont}{\end{iremark}\end{leftbar}}
# preface: |
#     This thesis was prepared at DTU Compute in fulfilment of the requirements for acquiring an M.Sc. in Engineering.

#     The thesis deals with ...

#     The thesis consists of ...
# acknowledgements: |
    # I would like to thank my cat...
---

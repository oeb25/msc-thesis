---
title: The Mist Programming Langauge
author: Oliver Bøving
toc: true
date: 15th July
year: 2023
bibliography: [ZoteroLib.bib]
natbib: true
# citeproc: true
# csl: https://www.zotero.org/styles/acm-siggraph?source=1
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
    - \usepackage{footnote}
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
    # - \usepackage[verbose]{backref}
    # Custom envs
    - \newtheorem{case}{Case}
    # Common notation
    - \newcommand{\lineref}[1]{\text{\footnotesize (line {#1})}}
    - \newcommand{\Rho}{\mathrm{P}}
    - \newcommand{\F}{\mathcal{F}}
    - \newcommand{\keyword}[1]{\textcolor{Teal600}{\texttt{#1}}}
    # - \newcommand{\leafin}[2]{#1\;\varepsilon\;#2}
    - \newcommand{\leaves}{\mathcal{L}}
    - \newcommand{\leafin}[2]{#1 \in \leaves(#2)}
    - \newcommand{\requires}{\reflectbox{$\mathrel{\leadsto}$}}
    - \newcommand{\Requires}{\:{\tiny\substack{\requires\\[-3pt]\requires}}\:}
    # - \newcommand{\compat}{\textcolor{red}\heartsuit}
    # - \newcommand{\incompat}{\heartsuit}
    - \newcommand{\compat}{\sim}
    - \newcommand{\incompat}{\not\sim}
    - \newcommand{\Compat}{\text{$\compat$}}
    # Function aliases
    - \newcommand{\PowerSet}{\text{\normalfont PowerSet}}
    - \newcommand{\fold}{\texttt{fold}}
    - \newcommand{\unfold}{\texttt{unfold}}
    - \newcommand{\ifold}{\textcolor{Teal600}{\texttt{fold}}}
    - \newcommand{\iunfold}{\textcolor{Teal600}{\texttt{unfold}}}
    - \newcommand{\iuse}{\textcolor{Teal600}{\texttt{use}}}
    - \newcommand{\foldable}{\texttt{foldable}}
    - \newcommand{\unfoldable}{\texttt{unfoldable}}
    - \newcommand{\prefix}{\texttt{prefix}}
    - \newcommand{\parents}{\texttt{parents}}
    - \newcommand{\fields}{\texttt{fields}}
    - \newcommand{\cover}{\texttt{cover}}
    - \newcommand{\siblings}{\texttt{siblings}}
    - \newcommand{\cut}{\texttt{cut}}
    - \newcommand{\T}{\mathcal{T}}
    - \newcommand{\Ts}{\mathbf{T}}
    - \newcommand{\locals}{\mathbf{Locals}}
    - \newcommand{\places}{\mathbf{Places}}
    - \newcommand{\A}{\mathbf{A}}
    # - \newcommand{\tinto}{\texttt{transitionInto}}
    - \newcommand{\tinto}{\ensuremath{\Delta}}
    # Lattice operators
    - \newcommand{\smaller}{\sqsubseteq}
    - \newcommand{\larger}{\sqsupseteq}
    - \newcommand{\join}{\sqcup}
    - \newcommand{\meet}{\sqcap}
    # Semantics
    - \newcommand{\inst}{\iota}
    - \newcommand{\Inst}{\mathbf{Inst}}
    - \newcommand{\sems}{\mathcal{S}}
    - \newcommand{\fsems}{\widecheck{\sems}}
    - \newcommand{\bsems}{\widehat{\sems}}
    - \newcommand{\sem}[1]{\sems\llbracket#1 \rrbracket}
    - \newcommand{\fsem}[1]{\fsems\llbracket#1 \rrbracket}
    - \newcommand{\bsem}[1]{\bsems\llbracket#1 \rrbracket}
    - \newcommand{\dom}{\texttt{dom}}
    - \newcommand{\pread}{\texttt{read}}
    - \newcommand{\eval}{\texttt{eval}}
    - \newcommand{\Mem}{\mathbf{Mem}}
    - \newcommand{\Expr}{\mathbf{Expr}}
    - \newcommand{\expr}{\texttt{expr}}
    - \newcommand{\Value}{\mathbf{Value}}
    - \newcommand{\State}{\mathbf{State}}
    - \newcommand{\pp}{\phi}
    - \newcommand{\Pp}{\Phi}

    - \newcommand{\lookmarker}[1]{\text{$^{_{#1}}$}}

    - \usepackage{varwidth}
    - \renewenvironment{prooftree}{\varwidth{.9\textwidth}\centering\leavevmode}{\DisplayProof\endvarwidth}

    - \usepackage{framed}

    - \renewenvironment{leftbar}[2][\hsize] { \def\FrameCommand { {\color{#2}\vrule width 3pt} \hspace{0pt} } \MakeFramed{\hsize#1\advance\hsize-\width\FrameRestore} } {\endMakeFramed}

    - \newenvironment{lemma}{\begin{leftbar}{Orange300}\begin{ilemma}}{\end{ilemma}\end{leftbar}}
    - \newenvironment{proposition}{\begin{leftbar}{Purple300}\begin{iproposition}}{\end{iproposition}\end{leftbar}}
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

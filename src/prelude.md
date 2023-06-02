---
title: The Mist Programming Langauge
author: Oliver Bøving
toc: true
date: 15th July
year: 2023
bibliography: [References.bib]
citeproc: true
classoption:
    - 10pt
    - twoside
documentclass: book
keywords: LaTeX, Pandoc
header-includes:
    - \usepackage{cleveref}
    - \usepackage{ebproof}
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
    - \newcommand{\leafin}[2]{#1\;\varepsilon\;#2}
    - \newcommand{\requires}{\reflectbox{$\mathrel{\leadsto}$}}
    # Function aliases
    - \newcommand{\fold}{\texttt{fold}}
    - \newcommand{\unfold}{\texttt{unfold}}
    - \newcommand{\prefixes}{\texttt{prefixes}}
    - \newcommand{\parents}{\texttt{parents}}
    - \newcommand{\T}{\mathcal{T}}
    - \newcommand{\A}{\mathbf{A}}
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
# preface: |
#     This thesis was prepared at DTU Compute in fulfilment of the requirements for acquiring an M.Sc. in Engineering.

#     The thesis deals with ...

#     The thesis consists of ...
# acknowledgements: |
    # I would like to thank my cat...
---

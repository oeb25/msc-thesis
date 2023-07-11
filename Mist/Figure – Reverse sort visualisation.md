---
tags: figure
---


```tikz
\usetikzlibrary{positioning}

\newcommand{\block}[1]{\texttt{B#1}}
\providecommand{\A}{\mathbf{A}}
\newcommand{\trans}[2]{$\delta(\phi_#1', \phi_#2)$}

\newcommand{\reveresed}[3]{
\draw[->, rounded corners] (#1-.7,-#2) -- (#1-.5,-#2) |- (#1-.3,-#3);
\draw[->, rounded corners] (#1-.7,-#3) -- (#1-.5,-#3) |- (#1-.3,-#2);
}
\newcommand{\same}[2]{
\draw[->] (#1-.7,-#2) -- (#1-.3,-#2);
}

\begin{document}
\begin{tikzpicture}[
    % node distance=28mm and 15mm,
    node distance=2mm,
    box/.style = {
        on grid,
        rounded corners,
        color=darkgray,
        fill=lightgray,
        align=center,
        inner xsep=2mm,
        inner ysep=3mm,
    },
    almost/.style={
      color=white,
      fill=violet,
      opacity=1,
    },
    sorted/.style={
      color=white,
      fill=teal,
      opacity=1,
    },
    pp/.style = {
        align=center,
        font=\footnotesize
    },
    tr/.style = {
        -latex,
        align=center,
        font=\footnotesize,
        rounded corners,
        color=teal
    },
    replace/.style = {
      ->
    },
]
  % \draw[help lines] (-5,-5) grid (5,5);
\node[box] at (-1,0) {8};
\node[box] at (-1,-1) {6};
\node[box] at (-1,-2) {5};
\node[box] at (-1,-3) {7};
\node[box] at (-1,-4) {9};
\node[box] at (-1,-5) {4};
\node[box,almost] at (0,0) {6};
\node[box] at (0,-1) {8};
\node[box] at (0,-2) {5};
\node[box] at (0,-3) {7};
\node[box] at (0,-4) {9};
\node[box] at (0,-5) {4};
\reveresed{0}{0}{1};
\node[box,almost] at (1,0) {5};
\node[box] at (1,-1) {8};
\node[box] at (1,-2) {6};
\node[box] at (1,-3) {7};
\node[box] at (1,-4) {9};
\node[box] at (1,-5) {4};
\reveresed{1}{0}{2};
\same{1}{1};
\node[box,sorted] at (2,0) {4};
\node[box] at (2,-1) {9};
\node[box] at (2,-2) {7};
\node[box] at (2,-3) {6};
\node[box] at (2,-4) {8};
\node[box] at (2,-5) {5};
\reveresed{2}{0}{5};
\reveresed{2}{1}{4};
\reveresed{2}{2}{3};
\node[box,sorted] at (3,0) {4};
\node[box,almost] at (3,-1) {7};
\node[box] at (3,-2) {9};
\node[box] at (3,-3) {6};
\node[box] at (3,-4) {8};
\node[box] at (3,-5) {5};
\reveresed{3}{1}{2};
\node[box,sorted] at (4,0) {4};
\node[box,almost] at (4,-1) {6};
\node[box] at (4,-2) {9};
\node[box] at (4,-3) {7};
\node[box] at (4,-4) {8};
\node[box] at (4,-5) {5};
\reveresed{4}{1}{3};
\same{4}{2};
\node[box,sorted] at (5,0) {4};
\node[box,sorted] at (5,-1) {5};
\node[box] at (5,-2) {8};
\node[box] at (5,-3) {7};
\node[box] at (5,-4) {9};
\node[box] at (5,-5) {6};
\reveresed{5}{1}{5};
\same{5}{3};
\reveresed{5}{2}{4};
\node[box,sorted] at (6,0) {4};
\node[box,sorted] at (6,-1) {5};
\node[box,almost] at (6,-2) {7};
\node[box] at (6,-3) {8};
\node[box] at (6,-4) {9};
\node[box] at (6,-5) {6};
\reveresed{6}{2}{3};
\node[box,sorted] at (7,0) {4};
\node[box,sorted] at (7,-1) {5};
\node[box,sorted] at (7,-2) {6};
\node[box] at (7,-3) {9};
\node[box] at (7,-4) {8};
\node[box] at (7,-5) {7};
\reveresed{7}{2}{5};
\reveresed{7}{3}{4};
\node[box,sorted] at (8,0) {4};
\node[box,sorted] at (8,-1) {5};
\node[box,sorted] at (8,-2) {6};
\node[box,almost] at (8,-3) {8};
\node[box] at (8,-4) {9};
\node[box] at (8,-5) {7};
\reveresed{8}{3}{4};
\node[box,sorted] at (9,0) {4};
\node[box,sorted] at (9,-1) {5};
\node[box,sorted] at (9,-2) {6};
\node[box,sorted] at (9,-3) {7};
\node[box] at (9,-4) {9};
\node[box] at (9,-5) {8};
\reveresed{9}{3}{5};
\same{9}{4};
\node[box,sorted] at (10,0) {4};
\node[box,sorted] at (10,-1) {5};
\node[box,sorted] at (10,-2) {6};
\node[box,sorted] at (10,-3) {7};
\node[box,sorted] at (10,-4) {8};
\node[box] at (10,-5) {9};
\reveresed{10}{4}{5};
\node[box,sorted] at (11,0) {4};
\node[box,sorted] at (11,-1) {5};
\node[box,sorted] at (11,-2) {6};
\node[box,sorted] at (11,-3) {7};
\node[box,sorted] at (11,-4) {8};
\node[box,sorted] at (11,-5) {9};
\same{11}{5};
\end{tikzpicture}
\end{document}
```

> [!caption]
> A visualisation of the 12 sublist reversals performed in the execution of `rev_sort([8,6,5,7,9,4])`, where each column going left to right is a step in the algorithm. The $\text{\textcolor{teal}{teal}}$ nodes are those in the sorted prefix, $\text{\textcolor{violet}{violet}}$ nodes are candidates to join the sorted prefix, and the arrows indicate which elements are swapped as sublists are reversed.

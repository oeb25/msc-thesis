---
tags: subfigure
attrs:
  width: 0.65
---

```tikz
\usetikzlibrary{positioning}

\newcommand{\block}[1]{\texttt{B#1}}
\providecommand{\A}{\mathbf{A}}
\newcommand{\trans}[2]{$\delta(\phi_#1', \phi_#2)$}

\begin{document}
\begin{tikzpicture}[
    % node distance=28mm and 15mm,
    node distance=2mm,
    box/.style = {
        on grid,
        rounded corners,
        color=white,
        fill=teal,
        align=center,
        inner sep=4mm,
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
]

    \begin{scope}
        % \draw[help lines] (-5,-5) grid (5,5);

        \node[box] (B0) at (-3,  2) {\block{0}};
        \node[box] (B1) at ( 0, 0) {\block{1}};
        \node[box] (B2) at (-3, 0) {\block{2}};
        \node[box] (B3) at ( 3, 0){\block{3}};
        \node[box] (B4) at ( 3, -2){\block{4}};
    \end{scope}

    \begin{scope}
        \newcommand{\pp}[1]{$\phi_#1$}

        \node[pp,anchor=south west] at (B1.north) {\pp{1}};
        \node[pp,anchor=north west] at (B1.south) {\pp{1'}};

        \node[pp,anchor=north west] at (B2.south) {\pp{2}};
        \node[pp,anchor=south west] at (B2.north) {\pp{2'}};

        \node[pp,anchor=north west] at (B3.south) {\pp{3}};
        \node[pp,anchor=south west] at (B3.north) {\pp{3'}};

        \node[pp,anchor=east] at (B0.west) {\pp{0}};
        \node[pp,anchor=south west] at (B0.east) {\pp{0'}};

        \node[pp,anchor=north east] at (B4.west) {\pp{4}};
        \node[pp,anchor=west] at (B4.east) {\pp{4'}};
    \end{scope}

    \draw[tr] (B0.east)  -- node[below] {\trans{0}{1}} ( 0, 2) -- (B1.north);

    \draw[tr] (B1.south) -- ( 0,-1) -- node[above] {\trans{1}{2}} (-3,-1) -- (B2.south);
    \draw[tr] (B2.north) -- (-3, 1) -- node[below] {\trans{2}{1}} ( 0, 1) -- (B1.north);

    \draw[tr] (B1.south) -- ( 0,-1) -- node[above] {\trans{1}{3}} ( 3,-1) -- (B3.south);
    \draw[tr] (B3.north) -- ( 3, 1) -- node[below] {\trans{3}{1}} ( 0, 1) -- (B1.north);

    \draw[tr] (B1.south) -- ( 0,-2) -- node[above] {\trans{1}{4}} (B4.west);
\end{tikzpicture}
\end{document}
```

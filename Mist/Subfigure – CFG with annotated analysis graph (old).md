---
tags: subfigure
attrs:
  width: 0.3
---

```tikz
\usetikzlibrary{positioning}

\newcommand{\block}[1]{\texttt{B#1}}
\providecommand{\A}{\mathbf{A}}
\newcommand{\trans}[2]{$\delta(\phi_#1', \phi_#2)$}

\begin{document}
\begin{tikzpicture}[
    auto,
    box/.style = {rounded corners,color=white,fill=teal,
        align=center,inner sep=4mm},
    pp/.style = {align=center, font=\footnotesize},
    tr/.style = {align=center, font=\footnotesize, color=teal},
]
    \begin{scope}[node distance=28mm and 5mm]
        \node[box] (b0)                  {\block{0}};
        \node[box] (b1) [below=of b0]    {\block{1}};
        \node[box] (b2) [right=of b1,xshift=20mm]    {\block{2}};
        \node[box] (b3) [below=of b1]    {\block{3}};
        \node[box] (b4) [right=of b3]    {\block{4}};
    \end{scope}
    \begin{scope}[node distance=-.5mm]
        \node[pp] (pp0) [above=of b0]    {$\phi_0$};
        \node[pp] (pp0') [below=of b0,xshift=3mm]   {$\phi_0'$};
        \node[pp] (pp1) [above=of b1,xshift=3mm]    {$\phi_1$};
        \node[pp] (pp1') [below=of b1,xshift=3mm]   {$\phi_1'$};
        \node[pp] (pp2) [left=of b2,yshift=-2.5mm]    {$\phi_2$};
        \node[pp] (pp2') [above=of b2,xshift=3mm]   {$\phi_2'$};
        \node[pp] (pp3) [above=of b3,xshift=3mm]    {$\phi_3$};
        \node[pp] (pp3') [below=of b3,xshift=3mm]   {$\phi_3'$};
        \node[pp] (pp4) [above=of b4,xshift=3mm]    {$\phi_4$};
        \node[pp] (pp4') [below=of b4]   {$\phi_4'$};
    \end{scope}

    \begin{scope}[rounded corners,color=teal,-latex]
        \draw (b0) -- node[tr,above,xshift=.5mm,yshift=2.5mm,rotate=90] {\trans{0}{1}} (b1);
        \draw (b1.south) |- ([yshift=23mm,xshift=-6mm]b4.north) |- node[tr,xshift=8mm] {\trans{1}{2}} (b2.west);
        \draw (b2.north) |- ([yshift=5mm]b1.north) node[tr,yshift=2.5mm,xshift=-1.5mm,left,midway] {\trans{1}{2}} -- (b1.north);
        \draw (b1) -- node[tr,above,xshift=-0mm,rotate=90] {\trans{1}{3}} (b3);
        \draw (b1.south) |- ([yshift=23mm]b4.north) -- node[tr,above,xshift=0mm,yshift=3mm,rotate=-90] {\trans{1}{4}} (b4.north);
        \draw (b3.south) -- ++(0,-3mm) -| ([xshift=-3mm]b1.west) node[tr,left,xshift=-2.5mm,rotate=90] {\trans{3}{1}} |- ([yshift=5mm]b1.north) -- (b1.north);
    \end{scope}
\end{tikzpicture}
\end{document}
```

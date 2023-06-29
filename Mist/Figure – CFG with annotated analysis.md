---
tags: figure
---

> [!subfigure|width=0.2]
> ```tikz
> \usetikzlibrary{positioning}
>
> \begin{document}
> \begin{tikzpicture}[
>     auto,
>     box/.style = {rounded corners,color=white,fill=teal,
>         align=center,inner sep=4mm},
>     pp/.style = {align=center},
>     tr/.style = {align=center, font=\footnotesize, color=teal},
> ]
>     \begin{scope}[node distance=20mm and 5mm]
>         \node[box] (b1)                  {$\alpha_1$};
>         \node[box] (b2) [below=of b1]    {$\alpha_2$};
>         \node[box] (b3) [below=of b2]    {$\alpha_3$};
>         \node[box] (b4) [right=of b3]    {$\alpha_4$};
>     \end{scope}
>     \begin{scope}[node distance=-.5mm]
>         \node[pp] (pp1) [above=of b1]    {$\phi_1$};
>         \node[pp] (pp1') [below=of b1,xshift=3mm]   {$\phi_1'$};
>         \node[pp] (pp2) [above=of b2,xshift=3mm]    {$\phi_2$};
>         \node[pp] (pp2') [below=of b2,xshift=3mm]   {$\phi_2'$};
>         \node[pp] (pp3) [above=of b3,xshift=3mm]    {$\phi_3$};
>         \node[pp] (pp3') [below=of b3,xshift=3mm]   {$\phi_3'$};
>         \node[pp] (pp4) [above=of b4,xshift=3mm]    {$\phi_4$};
>         \node[pp] (pp4') [below=of b4]   {$\phi_4'$};
>     \end{scope}
>
>     \begin{scope}[rounded corners,color=teal,-latex]
>         \draw (b1) -- node[tr] {$\delta(\phi_1', \phi_2)$} (b2);
>         \draw (b2) -- node[tr,yshift=-1.5mm] {$\delta(\phi_2', \phi_3)$} (b3);
>         \draw (b2.south) |- ([yshift=13mm]b4.north) -- node[tr,yshift=2mm] {$\delta(\phi_2', \phi_4)$} (b4.north);
>         \draw (b3.south) -- ++(0,-3mm) -| ([xshift=-3mm]b2.west) node[tr,left] {$\delta(\phi_3', \phi_2)$} |- ([yshift=4mm]b2.north) -- (b2.north);
>     \end{scope}
> \end{tikzpicture}
> \end{document}
> ```

> [!subfigure|width=0.3]
> $$
> \begin{gathered}
> \begin{aligned}
>     \A(\phi_1) &\smaller \bsem{\alpha_1}(\A(\phi_2)) \\
>     \A(\phi_2) &\smaller \bsem{\alpha_2}(\A(\phi_3)) \\
>     \A(\phi_2) &\smaller \bsem{\alpha_2}(\A(\phi_4)) \\
>     \A(\phi_3) &\smaller \bsem{\alpha_3}(\A(\phi_2)) \\
> \end{aligned} \\[1em]
> \bsem{{\color{Teal900} \delta(\phi_i, \phi_j)}}(\A(\phi_j)) = \tinto[\A(\phi_j), \A(\phi_i)](\A(\phi_j)) \\
> \bsem{\alpha_i}(\A(\phi_i')) = \A(\phi_i) \\
> \fsem{\alpha_i}(\A(\phi_i)) = \A(\phi_i')
> \end{gathered}
> $$

> [!caption]
> An abstract CFG with annotated program points and constraints.

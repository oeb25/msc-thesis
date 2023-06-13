---
tags: figure
---

> [!subfigure|width=0.1]
> ```{.folding-tree root="$\\T_1$"}
> { x X y X }
> ```

> [!subfigure|width=0.07,align=t]
> $$
> \begin{gathered}
> \text{\footnotesize $\requires .x.u$} \\[-5pt]
> \longrightarrow
> \end{gathered}
> $$

> [!subfigure|width=0.125]
> ```{.folding-tree root="$\\T_2$"}
> { x { u X v X } y X }
> ```

> [!subfigure|width=0.07,align=t]
> $$
> \begin{gathered}
> \text{\footnotesize $\requires .y.a.g$} \\[-5pt]
> \longrightarrow
> \end{gathered}
> $$

> [!subfigure|width=0.19]
> ```{.folding-tree root="$\\T_3$"}
> { x { u X v X } y { a { f X g X h X } b X } }
> ```

> [!subfigure|width=0.06,align=t]
> $$
> \begin{gathered}
> \text{\footnotesize $\requires .x$} \\[-5pt]
> \longrightarrow
> \end{gathered}
> $$

> [!subfigure|width=0.13]
> ```{.folding-tree root="$\\T_4$"}
> { x X y { a { f X g X h X } b X } }
> ```

> [!subfigure|width=0.09,align=t]
> $$
> \begin{gathered}
> \text{\footnotesize $\requires .y$} \\[-5pt]
> \longrightarrow
> \end{gathered}
> $$

> [!subfigure|width=0.1]
> ```{.folding-tree root="$\\T_5$"}
> { x X y X }
> ```

> [!caption]
> A sequence of folding trees showing how $\requires$ can perform the necessary foldings for making fields accessible. Each successive tree operates on the previous one, where the operation performed is indicated by arrow between them.

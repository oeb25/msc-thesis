---
tags: figure
---

> [!subfigure|width=0.13]
> ```{.folding-tree root="$s$"}
> { x X y X }
> ```

> [!subfigure|width=0.1,align=t]
> $$
> \begin{gathered}
> \text{\footnotesize \unfold\;$s.y$} \\[-5pt]
> \longrightarrow
> \end{gathered}
> $$

> [!subfigure|width=0.13]
> ```{.folding-tree root="$s$"}
> { x X y { a X b X } }
> ```

> [!subfigure|width=0.1,align=t]
> $$
> \begin{gathered}
> \text{\footnotesize \unfold\;$s.y.a$} \\[-5pt]
> \longrightarrow
> \end{gathered}
> $$

> [!subfigure|width=0.13]
> ```{.folding-tree root="$s$"}
> { x X y { a { f X g X h X } b X } }
> ```

> [!subfigure|width=0.1,align=t]
> $$
> \begin{gathered}
> \text{\footnotesize \fold\;$s.y.a$} \\[-5pt]
> \longrightarrow
> \end{gathered}
> $$

> [!subfigure|width=0.13]
> ```{.folding-tree root="$s$"}
> { x X y { a X b X } }
> ```

> [!subfigure|width=0.1,align=t]
> $$
> \begin{gathered}
> \text{\footnotesize \fold\;$s.y$} \\[-5pt]
> \longrightarrow
> \end{gathered}
> $$

> [!subfigure|width=0.13]
> ```{.folding-tree root="$s$"}
> { x X y X }
> ```

> [!caption]
> A sequence of unfolds and folds. Each successive tree operates on the previous one, where the operation performed is indicated by the arrow between them.

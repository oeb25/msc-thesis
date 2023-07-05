---
tags: figure
---

![[Subfigure – CFG with annotated analysis graph]]

> [!subfigure|width=0.3]
> $$
> \newcommand{\block}[1]{\texttt{B#1}}
> \begin{gathered}
> \\[-1.5em]
> \begin{aligned}
>     \A(\phi_0) &\smaller \bsem{\block{0}}(\A(\phi_1)) \\
>     \A(\phi_1) &\smaller \bsem{\block{1}}(\A(\phi_2)) \\
>     \A(\phi_1) &\smaller \bsem{\block{1}}(\A(\phi_3)) \\
>     \A(\phi_1) &\smaller \bsem{\block{1}}(\A(\phi_4)) \\
>     \A(\phi_2) &\smaller \bsem{\block{2}}(\A(\phi_1)) \\
>     \A(\phi_3) &\smaller \bsem{\block{3}}(\A(\phi_1)) \\
> \end{aligned} \\[0.55em]
> \A(\phi_4) \smaller \T_\diamond \\
> \end{gathered}
> $$

$$
\begin{gathered}
\bsem{{\color{Teal900} \delta(\phi_i, \phi_j)}}(\A(\phi_j)) = \tinto[\A(\phi_j), \A(\phi_i)](\A(\phi_j)) \\[.3em]
\sem{\texttt{B}_i}(\sigma,\A(\phi_i)) = \langle\sigma',\A(\phi_i')\rangle
\end{gathered}
$$

> [!caption]
> A CFG of the basic blocks of [[Example – SubFib FIR]] with annotated program points and constraints.

---
tags: definition
---

Let $\T_1, \T_2$ be folding trees, then $\tinto(\T_1, \T_2)$ computes $\mathcal{F}_1,\dots,\mathcal{F}_n$ being the folds and unfolds sufficient to transform $\T_1$ into $\T_2$, that is
$$
\begin{aligned}
\tinto[\T_1, \T_2] = \;& \mathcal{F}_n \circ \dots \circ \mathcal{F}_1 \\
\text{such that } [&\mathcal{F}_n \circ \dots \circ \mathcal{F}_1](\T_1) = \T_2.
\end{aligned}
$$
Formally $\tinto : \Ts \to \Ts \to (\Ts \to \Ts)$, but it is helpful to think of the result as a sequence of foldings.
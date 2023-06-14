---
tags: definition
---

Let $\T_1$ and $\T_2$ be folding trees, then we say that $\T_1$ is _smaller than_ $\T_2$ iff every leaf of $\T_1$ is contained in $\T_2$, that is
$$
\T_1 \smaller \T_2 = \leaves(\T_1) \subseteq \T_2,
$$
or equivalently, since $\leaves(\T_1) \subseteq \T_1$ and $\T_1$ is prefix closed
$$
\T_1 \smaller \T_2 = \T_1 \subseteq \T_2.
$$

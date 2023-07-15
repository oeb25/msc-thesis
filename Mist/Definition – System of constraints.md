---
tags: definition
---

Let $\pp_i \in \Pp$ be the program point immediately before $\inst_i \in \Inst$, and let $\phi_j$ be a program point immediately after $\pp_i'$, then the analysis analysis assignment $\A$ is constrained by
$$
\A(\pp_i) \smaller \bsem{\inst_i}(\A(\pp_j)).
$$
Additionally, let $\T_\diamond \in \Ts$ be the initial element that must be satisfied at the exit of the final block $\pp_\blacktriangleleft$, i.e.
$$
\A(\pp_\blacktriangleleft) \smaller \T_\diamond.
$$

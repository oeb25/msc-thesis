---
tags: definition
---

Let $\mathcal{F} : \places \to \Ts \to \Ts$ be a _fold_ or _unfold_ of a place in a folding tree. Let $\rho$ be a place and $\T$ a folding tree, then we define $\fold$ and $\unfold$
$$
\begin{aligned}
\fold(\rho, \T) &= \T \setminus \fields(\rho) \\[2pt]
\text{where }& \fields(\rho) \subseteq \leaves(\T) \land \foldable(\rho) \\[1em]
\unfold(\rho, \T) &= \T \cup \fields(\rho) \\[2pt]
\text{where }& \rho \in \leaves(\T) \land \foldable(\rho) \\
\end{aligned}
$$
Additionally, we allow $\fold$ and $\unfold$ to be _partially applied_, which is to say that $\fold\;\rho = \lambda \T. \fold(\rho, \T)$ is a function of type $\T \to \T$ which folds $\rho$ in the provided tree, and likewise for $\unfold$.
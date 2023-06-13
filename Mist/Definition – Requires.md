---
tags: definition
---
Let $\requires : \Ts \to \places \to \Ts$ be the recursive function that takes a folding tree $\T$ and a place $\rho$ and produces a new tree, where $\rho$ is folded, defined like so
$$
\T \requires \rho = \begin{cases}
	\T & \text{if } \rho \in \leaves(\T) \land \rho \in \T, \\
	\fold(\rho, \T \requires \rho.f_1 \requires \dots \requires \rho.f_n) & \text{if } \rho \notin \leaves(\T) \land \rho \in \T \land \fields(\rho) \subseteq \T, \\
	\unfold(\rho', \T \requires \rho') & \text{if } \rho \notin \leaves(\T) \land \rho \notin \T \land \rho = \rho'.f_i, \\
\end{cases}
$$
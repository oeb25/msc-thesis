---
tags: definition
---
Let $\requires : \Ts \to \places \to \Ts$ be the recursive function that takes a folding tree $\T$ and a place $\rho$ and produces a new tree, where $\rho$ is folded. Let this operation be called _requires_ and be defined by
$$
\T \requires \rho = \begin{cases}
	\T & \text{if } \rho \in \leaves(\T) \land \rho \in \T, \\
	\fold(\rho, \T \Requires \fields(\rho)) &
		\text{if } \rho \notin \leaves(\T) \land \rho \in \T \land \fields(\rho) \subseteq \T \\
	\unfold(\rho', \T \requires \rho') & \text{if } \rho \notin \leaves(\T) \land \rho \notin \T \land \rho = \rho'.f_i. \\
\end{cases}
$$
Let $\Requires: \Ts \to \PowerSet(\places) \hookrightarrow \Ts$ be a partial function that requires all places of a compatible set $\Rho=\{\rho_1, \dots,\rho_n\}$ in a folding tree $\T$ s.t.
$$
\T \Requires \Rho = \T \requires \rho_1 \requires \dots \requires \rho_n.
$$

%%$$
\forall \T \in \Ts, \Rho \subseteq \places : \Compat\Rho \implies \Rho \subseteq \leaves(\T \Requires \Rho)
$$
$$
\T \mathop{\requires}\limits_{\rho \in \Rho} \rho
$$%%
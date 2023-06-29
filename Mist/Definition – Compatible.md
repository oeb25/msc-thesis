---
tags: definition
---

Let $\rho_1, \rho_2 \in \places$ be places, then we say that _$\rho_1$ and $\rho_2$ are compatible places_, denoted by $\rho_1 \compat \rho_2$, and defined by
$$
\rho_1 \compat \rho_2 =\rho_1 \neq \rho_2 \to \rho_1 \notin \prefix(\rho_2) \land \rho_2 \notin \prefix(\rho_1).
$$
%%$$
\rho_1 \not\compat \rho_2 = \rho_1 < \rho_2 \lor \rho_2 < \rho_1.
$$%%
Conversely, let two places be _incompatible_, denoted by $\rho_1 \incompat \rho_2$ if they are not compatible.

Additionally, let $\Rho \subseteq \places$ be a set of places, then we say that _$\Rho$ is a compatible set_, denoted by $\Compat \Rho$, iff elements of $\Rho$ are pair-wise compatible, i.e.
$$
	\Compat\Rho = \forall \rho_1, \rho_2 \in \Rho : \rho_1 \compat \rho_2.
$$

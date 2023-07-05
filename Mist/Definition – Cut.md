---
tags: definition
---
Let $\rho$ be a place and $\T$ a folding tree, then we define
$$\cut : \places \to \Ts \to \PowerSet(\places)$$
as the function that computes the set of leaves in $\T$ that are compatible with $\rho$. Formally defined as
%%$$
\cut(\rho,\T) = \{ \rho' \mid \rho' \in \leaves(\T) \land \prefix(\rho) \not\subseteq \prefix(\rho') \land \prefix(\rho') \not\subseteq \prefix(\rho) \}.
$$%%
%%$$
\cut(\rho,\T) = \{ \rho' \mid \rho' \in \leaves(\T) \land \rho \notin \prefix(\rho') \land \rho' \notin \prefix(\rho) \}.
$$%%
$$
\cut(\rho,\T) = \{ \rho' \in \leaves(\T) \mid \rho \compat \rho' \}.
$$

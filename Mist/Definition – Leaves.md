---
tags: definition
---
Let $\T$ be a folding tree, then $\leaves(\T)$ is the set of all leaves (and thus folded places) in $\T$ computed by
%%$$
\leaves(\T) = \{ \rho \mid \rho \in \T \land \fields(\rho) \not\subseteq \T \}.
$$%%
%%$$
\leaves(\T) = \{ \rho \mid \rho \in \T \land (\unfoldable(\rho) \lor \fields(\rho) \not\subseteq \T) \}.
$$%%
$$
\leaves(\T) = \{ \rho \in \T \mid \forall \rho' \in \fields(\rho) : \rho' \notin \T \}.
$$

This is analogous to the definition of _frontier positions_ [[@comonTreeAutomataTechniques1997#p. 14]].
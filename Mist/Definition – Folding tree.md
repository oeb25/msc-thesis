---
tags: definition
---

%%
Let $\T$ be a folding tree represented as a set of places, where every prefix of a place in $\T$ is also in $\T$. Nodes in the tree can either be _folded_ or _unfolded_, where folded nodes are leaves and unfolded nodes are internal nodes. All places have a set of associated fields, and unfolded nodes will have all of their fields as children.

Formally, we say that the any folding tree is a _prefix-closed set of places_, that is
%%

Let $\T \subseteq \PowerSet(\places)$ be a _folding tree_ represented as a prefix closed set of places
$$
\forall \rho \in \T : \prefix(\rho) \subseteq \T.
$$
Let $\Ts$ represent the set of all folding trees.

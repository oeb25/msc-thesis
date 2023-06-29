---
tags: section
---

A _folding tree_ is a data structure, denoted by $\T$, that maintains the folding state of places. It works by representing data types as a tree, where nodes are fields of potentially nested `struct`s. The leaves of the tree are all the places that are _folded_, while the internal nodes are _unfolded_ places, both uniquely described by paths from the root.

![[Figure – A visualization of the maximally unfolded tree given by `S`]]

![[Definition – Folding tree]]

![[Example – Folding tree]]

In addition to nodes being folded and unfolded, nodes can also be _foldable_ and _unfoldable_.

![[Definition – Foldable]]

When working with folding trees, it is useful to be able to refer to the set of folded places. We introduce a function $\leaves$, which computes this.

![[Definition – Leaves]]

> [!example]
> Consider again `struct S` from [[Figure – Breaking invariant]] and $\T_s$ from [[Example – Folding tree]], then we can compute the folded places in $\T_s$ as:
> $$\leaves(\T_s) = \{ s.x.u,\; s.x.v\;, s.y.a.f,\; s.y.a.g,\; s.y.a.h,\; s.y.b \}.$$

Something we can derive from the definition of $\leaves$, is that the places in the leaves of any folding tree will never be ancestors or decedents of each other. We call this property _compatible_.

![[Definition – Compatible]]

![[Proposition – Leaves are compatible]]

> [!proof]
> Let $\T$ be any folding tree, and $\rho_1, \rho_2 \in \leaves(\T)$ be arbitrary leaves from $\T$.
> 
> We prove by contradiction that $\rho_1 \notin \prefix(\rho_2)$, by assuming that $\rho_1 \neq \rho_2$ and $\rho_1 \in \prefix(\rho_2)$. Since $\rho_1$ is in the prefix of $\rho_2$, and $\rho_1 \neq \rho_2$, then a field of $\rho_1$ must be in $\prefix(\rho_2)$, call this $\rho_1.f_i$, and thus $\rho_1.f_i \in \T$ since $\T$ is prefix closed. However, since $\rho_1 \in \leaves(\T)$, then we know that $\rho_1.f_i \notin \T$, resulting in a contradiction.
> 
> Since $\rho_1, \rho_2$ were chosen arbitrarily, and we showed, without loss of generality, that $\rho_1 \notin \prefix(\rho_2)$ when $\rho_1 \neq \rho_2$, then $\Compat\leaves(\T)$.

This property comes up in many places when working with folding trees. Other prominent instances of compatible sets are $\fields(\rho)$  for any place $\rho$.

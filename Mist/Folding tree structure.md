---
tags: section
---

A _folding tree_ is a data structure, denoted by $\T$, that maintains the folding state of places. It represents data types as trees, where nodes are fields of potentially nested `struct`s. The leaves of the tree are all the places that are _folded_, while the internal nodes are _unfolded_ places, both uniquely described by paths from the root.

![[Figure – A visualization of the maximally unfolded tree given by `S`]]

These folding trees must satisfy specific properties concerning structure. At the core of these are sets called the _cover_ sets, which encapsulate all ancestors of a place, like $\prefix$, but extend to siblings of ancestors as well.

![[Definition – Place cover]]

> [!example]
> To get a grasp for which places are in which $\cover$ sets, we compute the $\cover$ set for all places in [[Figure – A visualization of the maximally unfolded tree given by `S`]]. In the table below, the group of places in the first column all have the same $\cover$ set; in the second column, that $\cover$ set is listed.
> $$
> \begin{array}{l|l}
> \rho \in \places & \cover(\rho) \\ \hline
> s & \{s\} \\
> s.x,\; s.y & \cover(s)\cup\{s.x,s.y\} \\
> s.x.u,\; s.x.v & \cover(s.x)\cup\{s.x.u,s.x.v\} \\
> s.y.a,\; s.y.b & \cover(s.y)\cup\{s.y.a,s.y.b\} \\
> s.y.a.f,\; s.y.a.g,\; s.y.h & \cover(s.y.a)\cup\{s.y.a.f,s.y.a.g,s.y.a.h\} \\
> \end{array}
> $$

What defines a folding tree is a tree of places where if a place $\rho$ is present, then so is $\cover(\rho)$.

![[Definition – Folding tree]]

![[Example – Folding tree]]

In addition to being folded and unfolded, nodes can be _foldable_ and _unfoldable_.

![[Definition – Foldable]]

When working with folding trees, referring to the set of folded places is helpful. We introduce a function $\leaves$, which computes this.

![[Definition – Leaves]]

> [!example]
> Consider again `struct S` from [[Figure – Breaking invariant]] and $\T_s$ from [[Example – Folding tree]], then we can compute the folded places in $\T_s$ as:
> $$\leaves(\T_s) = \{ s.x.u,\; s.x.v\;, s.y.a.f,\; s.y.a.g,\; s.y.a.h,\; s.y.b \}.$$

Something we can derive from the definition of $\leaves$ is that the places in the leaves of any folding tree will never be ancestors or decedents of each other. We call this property _compatible_.

![[Definition – Compatible]]

![[Proposition – Leaves are compatible]]

> [!proof]
> Let $\T$ be any folding tree, and $\rho_1, \rho_2 \in \leaves(\T)$ be arbitrary leaves from $\T$. We need to show that $\rho_1 \compat \rho_2$, which amounts to showing $\rho_1 \not< \rho_2$ and $\rho_2 \not< \rho_1$. Without loss of generality, we will show $\rho_1 \not< \rho_2$ by contradiction.
>
> We start by assuming that $\rho_1 < \rho_2$, then a field, $\rho_1.f_i$, must satisfy $\rho_1.f_i \leq \rho_2$, and thus $\rho_1.f_i \in \T$ since $\T$ is cover-closed. However, since $\rho_1 \in \leaves(\T)$, then we know that $\rho_1.f_i \notin \T$, resulting in a contradiction.
>
> Since $\rho_1, \rho_2$ were chosen arbitrarily, and we showed, without loss of generality, that $\rho_1 \not< \rho_2$, then $\Compat\leaves(\T)$.

This property comes up in many places when working with folding trees. Other prominent instances of compatible sets are $\fields(\rho)$  for any place $\rho$.

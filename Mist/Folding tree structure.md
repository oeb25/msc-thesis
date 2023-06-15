---
tags: section
---

A _folding tree_ is a data structure, denoted by $\T$, that maintains the folding state of places. It works by representing data types as a tree, where nodes are fields of potentially nested `struct`s. The leaves of the tree are all the places that are _folded_, while the internal nodes are _unfolded_ places, both uniquely described by paths from the root.

![[Figure – A visualization of the maximally unfolded tree given by `S`]]

![[Definition – Folding tree]]

![[Example – Folding tree]]

When working with folding trees, it is useful to be able to refer to the set of folded places. We introduce a function $\leaves$, which computes this.

![[Definition – Leaves]]

> [!example]
> Consider again `struct S` from [[Figure – Breaking invariant]] and $\T_s$ from [[Example – Folding tree]], then we can compute the folded places in $\T_s$ as:
> $$\leaves(\T_s) = \{ s.x.u,\; s.x.v\;, s.y.a.f,\; s.y.a.g,\; s.y.a.h,\; s.y.b \}.$$

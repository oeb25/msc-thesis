---
tags: example
---
Consider the `struct S` from the running example [[Figure – Breaking invariant]]. The tree in [[Figure – A visualization of the maximally unfolded tree given by `S`]] displays the maximally unfolded version of `S`. In this figure, the root refers to slot `s`, and each child is denotes a field on the parent. Maximally unfolded means that no leaf in the tree can be unfolded, due to them being primitive types in this case.

The set representing this tree is:
$$\T_s = \{ s,\; s.x,\; s.x.u,\; s.x.v,\; s.y,\; s.y.a,\; s.y.a.f,\; s.y.a.g,\; s.y.a.h,\; s.y.b \}.$$
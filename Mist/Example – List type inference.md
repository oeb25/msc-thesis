---
tags: example
---

Consider the following program, where `fn f` produces the list `[1, 2]` by concatenation.
```{.mist .numberLines}
fn f() -> [int] {
  let list = [];
  [1] + list + [2]
}
```
When the compiler type checks the list declaration $\lineref{2}$, the variable has no explicit type, and must thus be determined from the expression. The expression `[]`, however, does not have enough context (yet) to completely figure out what type it has, so it instantiates a new _free_ type, call this `'a`. Then `list` is given the type `['a]`, that is a list of `'a`, where `'a` is an unconstrained free type.

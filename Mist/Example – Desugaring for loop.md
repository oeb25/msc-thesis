---
tags: example
---

Consider the Mist program below shows an example of desugaring a `for`-loop into a `while`-loop:
```{.mist .numberLines}
// Original
fn f() {
  let mut sum = 0;
  for i in 0..10
    inv sum == i * (i - 1) / 2
  {
    sum = sum + i;
  }
  assert sum == 45;
}
//@ align
// Desugared
fn f() {
  let mut sum = 0;
  let mut i = 0;
  while i < 10
    inv 0 <= i <= 10
    inv sum == i * (i - 1) / 2
    dec 10 - i
  {
    sum = sum + i;
    i = i + 1;
  }
  assert sum == 45;
}
```
The `inv`ariants of the first loop are kept $\lineref{5, 17}$, and a new is added to bound `i` $\lineref{16}$ and one for termination $\lineref{28}$, which is incremented at the end of the loop body $\lineref{21}$.

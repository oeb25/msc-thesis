---
tags: example
---

Pairwise sum of two lists to produce a new of equal length.

```{.mist .numberLines}
fn add(x: [int], y: [int]) -> [int]
  req x.len == y.len
  ens result.len == x.len,
      forall i in 0..x.len { result[i] == x[i] + y[i] }
{
  let mut z = [];
  for idx in 0..x.len
    inv z.len == idx,
        forall i in 0..idx { z[i] == x[i] + y[i] }
  {
    z = z + [x[idx] + y[idx]];
  }
  z
}

```
---
tags: figure
---

```{.mist .numberLines}
pure fn sum_to(n: int) -> int
  req 0 <= n
  ens result ==
    n * (n - 1) / 2
{
  let mut sum = 0;
  for i in 0..n
    inv sum ==
      i * (i - 1) / 2
  {
    sum = sum + i;
  }
  let res =
    n * (n - 1) / 2;
  assert sum == res;
  res
}
//@ align
pure fn sum_of(r: Range[int]) -> int
  req 0 <= r.min, r.min <= r.max
  ens result ==
    sum_to(r.max) - sum_to(r.min)
{
  let mut sum = 0;
  for i in r
    inv sum ==
      sum_to(i) - sum_to(r.min)
  {
    sum = sum + i;
  }
  let res =
    sum_to(r.max) - sum_to(r.min);
  assert sum == res;
  res
}
```

> [!caption]
> Two functions, `fn sum_to` and `fn sum_of` computing $\sum_{i=0}^{n-1}i$ and $\sum_{i=n}^{m-1}i$, shown to equal their closed form counter parts.

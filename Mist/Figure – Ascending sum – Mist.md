---
tags: figure
---

```{.mist .numberLines}
struct Sorted {
  xs: [int],
  ghost min: int,
}

invariant Sorted {
  forall i in 0..self.xs.len, j in i..self.xs.len {
    self.xs[i] <= self.xs[j]
  }
}
invariant Sorted {
  if self.xs != [] { self.xs[0] == self.min }
}

fn sum(s: &Sorted) -> int
  ens result >= s.min * s.xs.len
  dec _
{
  let mut sum = 0;
  for index in 0..s.xs.len
    inv sum >= s.min * index
  {
    sum = sum + s.xs[index];
  }
  sum
}
```

> [!caption]
> A Mist program computing the sum of a sorted list.

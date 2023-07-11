---
tags: figure
---

```{.mist .ignoreErrors .numberLines .viperCompat}
struct Sorted {
  let xs: seq<int>
  ghost let min: int
}

ghost invariant IsSorted(s: Sorted)
  reads s
{
  (forall i, j :: 0 <= i <= j < |s.xs| ==> s.xs[i] <= s.xs[j]) &&
  (s.xs != [] ==> s.xs[0] == s.min)
}

fn sum(s: Sorted) returns (res: int)
  requires IsSorted(s)
  ensures res >= s.min * |s.xs|
  // modifies s
{
  res := 0;
  for index := 0 to |s.xs|
    inv res >= s.min * index
  {
    res := res + s.xs[index];
  }
}
```

> [!caption]
> A Dafny of the [[Figure – Ascending sum – Mist]] program.
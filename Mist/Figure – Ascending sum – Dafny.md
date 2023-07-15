---
tags: figure
---

```{.mist .ignoreErrors .numberLines .viperCompat}
struct Sorted {
  let xs: seq<int>
  ghost let min: int
  ghost invariant IsSorted() reads this {
    (forall i, j :: 0 <= i <= j < |xs| ==> xs[i] <= xs[j]) &&
    (xs != [] ==> xs[0] == min)
  }
}

fn sum(s: Sorted) returns (res: int)
  requires s.IsSorted()
  ensures s.IsSorted()
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
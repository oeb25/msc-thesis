---
tags: figure
---

```{.mist .numberLines .ignoreErrors .viperCompat}
let xs: Seq[int]
let min: int

invariant Sorted(s: Ref) {
  acc(s.xs, write) && acc(s.min, write) &&
  (forall i: int, j: int :: 0 <= i && i < j && j < |s.xs| ==>
    s.xs[i] <= s.xs[j]) &&
  (s.xs != Seq() ==> s.xs[0] == s.min)
}

define SortedEq(a, b) (a.xs == b.xs && a.min == b.min)

fn sum(s: Ref, p: Perm) returns (res: int)
  requires p > none && acc(Sorted(s), p)
  ensures acc(Sorted(s), p) && unfolding acc(Sorted(s), p) in
      SortedEq(s, old(unfolding acc(Sorted(s), p) in s)) &&
      res >= s.min * |s.xs|
{
  res := 0
  let index: int := 0
  while (index < unfolding acc(Sorted(s), p) in |s.xs|)
    inv acc(Sorted(s), p) && unfolding acc(Sorted(s), p) in
      SortedEq(s, old(unfolding acc(Sorted(s), p) in s)) &&
      0 <= index && index <= |s.xs| &&
      res >= s.min * index
  {
    unfold acc(Sorted(s), p)
    res := res + s.xs[index]
    index := index + 1
    fold acc(Sorted(s), p)
  }
}
```

> [!caption]
> A Viper implementation of the [[Figure – Ascending sum – Mist]] program.
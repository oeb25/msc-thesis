---
tags: figure
---

```{.mist .numberLines}
pure ghost fn reversed(a: [int], b: [int]) -> bool
  req a.len == b.len
{ forall idx in 0..a.len { a[a.len - idx - 1] == b[idx] } }
pure ghost fn reverse(s: [int]) -> [int]
  ens s.len == result.len, reversed(s, result)
  proof {
    let mut res = [];
    for i in 0..s.len
      inv i == res.len, reversed(res, s[0..i])
    { res = [s[i]] + res; }
    res
  }

pure ghost fn same_elements(a: [int], b: [int]) -> bool
  { a.len == b.len && a.to_multi_set() == b.to_multi_set() }
ghost fn lemma_reverse_same_elements(a: [int])
  ens same_elements(a, reverse(a))
  proof {}
```

> [!caption]
> First part of a Mist program sorting a list by reversing sublists. Second part is found in [[Figure – Reverse sort (Part 2) – Mist]].

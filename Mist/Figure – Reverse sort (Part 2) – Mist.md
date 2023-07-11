---
tags: figure
---

```{.mist .numberLines offset="18"}
pure fn sorted(s: [int]) -> bool
  { forall i in 0..s.len, j in i..s.len { s[i] <= s[j] } }
pure fn all_smaller(a: [int], b: [int]) -> bool
  { forall i in 0..a.len, j in 0..b.len { a[i] <= b[j] } }
fn rev_sort(s: [int]) -> [int]
  ens result.len == s.len, same_elements(result, s), sorted(result)
  dec _
{
  let mut res = s;
  for iter in 0..res.len
    inv same_elements(res, s), sorted(res[0..iter])
    inv all_smaller(res[0..iter], res[iter..])
  {
    for idx in iter..res.len
      inv same_elements(res, s), sorted(res[0..iter])
      inv all_smaller(res[0..iter], res[iter..])
      inv all_smaller([res[iter]], res[iter..idx])
    {
      if res[idx] < res[iter] {
        lemma_reverse_same_elements(res[iter..idx+1]);
        res = res[0..iter] + reverse(res[iter..idx+1]) + res[idx+1..];
      }
    }
  }
  res
}
```

> [!caption]
> A continuation of [[Figure – Reverse sort (Part 1) – Mist]] sorting a list by reversing sublists.
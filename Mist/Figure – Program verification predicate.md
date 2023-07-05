---
tags: figure
---

```{.mist .ignoreErrors .viperCompat}
invariant Sorted(s: Seq[int]) {
  forall i, j :: 0 <= i < j < |s| ==> s[i] <= s[j]
}
fn fails(s: Seq[int]) requires |s| == 3 && s[0] > 0 && Sorted(s) {
  let sum: Int := s[0] + s[1] + s[2];
  assert sum >= 3; // verification error
}
fn works(s: Seq[int]) requires |s| == 3 && s[0] > 0 && Sorted(s) {
  unfold Sorted(s);
  let sum: Int := s[0] + s[1] + s[2];
  assert sum >= 3; // successfully verifies
}
```

> [!caption]
> A `@vpr invariant` for a sorted list in psuedo-Viper, and two functions: one fails verification, the other does not.

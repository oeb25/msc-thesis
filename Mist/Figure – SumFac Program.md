---
tags: figure
---

```{.mist .numberLines}
struct SumFac {
  n: int,
  sum: int,
  fac: int,
}
invariant SumFac {
  self.n >= 0 &&
  self.sum == sum(self.n) &&
  self.fac == fac(self.n)
}
//@ align
pure fn sum(n: int) -> int
  req n >= 0
  ens result >= 0
{ n * (n + 1) / 2 }

pure fn fac(n: int) -> int
  req n >= 0
  ens result >= 1
{ if n < 2 { 1 }
  else { n * fac(n - 1) } }
//@ break
fn compute(m: int) -> SumFac req n > 1, ens result.n == m {
  let r = SumFac { n: 0, sum: 0, fac: 1 };

  for i in 1..m + 1
    inv r.n == i - 1
  { r.n = i; r.sum = r.sum + i; r.fac = r.fac * i; }

  r
}
```

> [!caption]
> A Mist program containing `struct SumFac` \lineref{1-5} with three fields constrained by an `invariant SumFac` \lineref{6-10}, ensuring that `self.sum` is the sum from `1` to `self.n` and `self.fac` is the product of the same range. These use the functions `pure fn sum` \lineref{11} and `pure fn fac` \lineref{16}, which compute the sum and factorial, respectively. Finally, `fn compute` \lineref{21} takes `m` and produces a `struct SumFac` with `result.n == m`.

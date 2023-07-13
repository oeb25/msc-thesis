---
tags: example
---

A function, `@pow2`, for computing $2^n$, and a lemma to show that the size of a complete binary-tree is given by $2^{h+1}-1$ where $h$ is the height of the tree.

```{.mist .numberLines offset="56"}
pure fn pow2(n: int) -> int
  req n >= 0
  dec n
{
  if n == 0 { 1 } else { 2 * pow2(n - 1) }
}
ghost fn lemma_complete(b: BTree)
  ens size(b) == pow2(height(b) + 1) - 1
  dec height(b)
  proof {
    if b.left == null {
      assert pow2(1) == 2 * pow2(0);
    } else {
      lemma_complete(b.left!);
      lemma_complete(b.right!);
    }
  }
```
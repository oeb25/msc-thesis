---
tags: example
---

Building on [[Example – BTree – Values]], `fn height` recursively computes the height of a `BTree`.

```{.mist .numberLines offset="19"}
pure fn height(b: &BTree) -> int
  ens result >= 0
{
  if b.left == null {
    0
  } else {
    let l = height(b.left!);
    let r = height(b.right!);
    1 + if l > r { l } else { r }
  }
}
```
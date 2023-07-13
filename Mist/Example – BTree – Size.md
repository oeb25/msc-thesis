---
tags: example
---

A pure and terminating function `@size`, recursively computing the total number of nodes and leaves in a `BTree`.

```{.mist .numberLines offset="33"}
pure fn size(b: &BTree) -> int
  ens result >= 1
  dec height(b)
{
  if b.left == null {
    1
  } else {
    let l = size(b.left!);
    let r = size(b.right!);
    1 + l + r
  }
}
```
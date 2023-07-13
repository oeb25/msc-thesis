---
tags: example
---

A declaration of a struct representing _binary-trees of integers_.

```{.mist .numberLines}
struct BTree {
  value: int,
  left: ?BTree,
  right: ?BTree,
}
invariant BTree {
  (self.left == null) == (self.right == null)
}
fn leaf(v: int) -> BTree {
  BTree { value: v, left: null, right: null }
}
```

---
tags: example
---

A continuation of [[Example – BTree – Struct]]

```{.mist .numberLines offset="11"}
pure fn values(b: &BTree) -> Set[int] {
  [b.value].to_set()
    .union(if b.left != null {
      values(b.left!).union(values(b.right!))
    } else {
      [].to_set()
    })
}
```
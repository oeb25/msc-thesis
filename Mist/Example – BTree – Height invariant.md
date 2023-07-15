---
tags: example
---

The code below is a continuation of [[Example – BTree – Height]].

```{.mist .numberLines offset="28"}
invariant BTree {
  if self.left != null {
    height(self.left!) == height(self.right!)
  }
}
```
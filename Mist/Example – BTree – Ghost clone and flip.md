---
tags: example
---

Two abstract ghost functions, `@clone` and `@flip`.

```{.mist .numberLines offset="76"}
ghost fn clone(b: &BTree) -> BTree
  ens values(b) == values(&result);

ghost fn flip(b: &BTree) -> BTree
  ens values(b) == values(&result),
      height(b) == -1 * height(&result);
```

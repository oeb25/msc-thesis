---
tags: example
---

A display of what becomes possible when `@flip` is involved.

```{.mist .numberLines offset="79"}
ghost fn unsoundness(b: &BTree)
  req height(b) != 0
{
  let p = flip(b);
  assert 1 == 2;
}
```

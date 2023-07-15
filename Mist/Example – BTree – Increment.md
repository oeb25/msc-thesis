---
tags: example
---

A function, `@increment`, which increases the value of each node a given `amount`.

```{.mist .numberLines offset="45"}
fn increment(b: &mut BTree, amount: int)
  ens forall x in old(values(b)) exists y in values(b)
        { x + amount == y }
  dec height(b)
{
  b.value = b.value + amount;
  if b.left != null {
    increment(b.left!, amount); increment(b.right!, amount);
    assert old(values(b) == [t.value].to_set()
                             .union(values(t.left!))
                             .union(values(t.right!)));
  }
  let tmp = b.left; b.left = b.right; b.right = tmp;
}
```

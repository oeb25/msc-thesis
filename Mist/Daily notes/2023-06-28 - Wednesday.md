---
tags: daily
---

```mist
pure fn sum(r: Range) -> int {
	let a = r.min * (r.min + 1) / 2;
	let b = r.max * (r.max + 1) / 2;

	let mut sum = 0;
	for i in r
	  inv sum == (i * (i + 1) / 2) - a
	{
	  sum = sum + i;
	}
	assert sum == b - a;

	b - a
}
```

```mist
pure fn sum(r: Range) -> int {
	let mut sum = 0;
	for i in r
	  inv sum == (i * (i + 1) / 2) - (r.min * (r.min + 1) / 2)
	{
	  sum = sum + i;
	}
    sum
}
```

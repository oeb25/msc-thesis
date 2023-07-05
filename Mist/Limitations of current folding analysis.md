---
tags: subsection
---

- Expressions such as `f(s, s.a)` are in violation of [[Definition – FIR well-defined access rules]].
- Expressions such as `r.sum = compute_sum(r)` are not allowed.
	- These could be allowed by rewriting to

		`tmp = compute_sum(r); r.sum = tmp`,

		where the analysis would then insert an `unfold r` in between the two statements.

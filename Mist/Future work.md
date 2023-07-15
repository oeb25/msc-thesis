---
tags: section
---

We have planted the seed for the Mist language and compiler, but there is quite way to go for Mist to become a competitive choice for serious verification. The following sections cover current limitations and potential future capabilities.

%%

> [!paragraph] Random ideas
> 
> - Infer preconditions for other conditions, i.e.
>     - `req xs[10]` should automatically be transformed into `req xs.len >= 10, xs[10]`.
> - Same principle for function invocation:
>     - With `fn f(x: int) -> bool req x == 5`
>     - `fn g(y: int) req f(y)` becomes `fn g(y: int) req y == 5, f(y);`

%%

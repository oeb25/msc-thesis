---
tags: section
---

The seed for the Mist language and compiler has been planted, but there is still a long way to go for Mist to become a viable choice for serious verification work. The following sections cover some of the current limitations, and potential future capabilities.

%%

> [!paragraph] Random ideas
> 
> - Infer preconditions for other conditions, i.e.
>     - `req xs[10]` should automatically be transformed into `req xs.len >= 10, xs[10]`.
> - Same principle for function invocation:
>     - With `fn f(x: int) -> bool req x == 5`
>     - `fn g(y: int) req f(y)` becomes `fn g(y: int) req y == 5, f(y);`

%%

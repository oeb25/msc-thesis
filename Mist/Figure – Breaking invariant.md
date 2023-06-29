---
tags: figure
---

> [!subfigure|width=0.9,align=b]
> ```{.mist .ignoreErrors}
> struct S { x: X, y: Y }     struct X { u: int, v: int }
> struct Y { a: A, b: int }   struct A { f: int, g: int, h: int }
>
> invariant X { self.u == -self.v }
>
> fn inc(s: &mut S) -> int {
>     assert s.x.v + s.x.u == 0;
>     
>     s.x.u += 1;
>     s.x.v -= 1;
>
>     s.y.a.g
> }
> ```

> [!caption]
> This program introduces three pieces of information: Definitions for structs `S`, `X`, `Y`, and `A` \lineref{1-2}, an invariant on `X` \lineref{4}, and a function `inc` \lineref{6-13}.

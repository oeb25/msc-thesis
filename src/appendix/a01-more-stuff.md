# More Stuff {#app:more-stuff}

We can even have code blocks!

\makeatletter
\def\verbatim@nolig@list{}%
\makeatother

```{.mist .ignoreErrors}
fn fib(n: u32) -> u32 {
    match n {
        n if n < 2 => 1,
        _ => fib(n - 1) + fib(n - 2),
    }
}
```
```rs
fn fib(n: u32) -> u32 {
    match n {
        n if n < 2 => 1,
        _ => fib(n - 1) + fib(n - 2),
    }
}
```

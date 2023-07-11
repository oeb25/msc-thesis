---
tags: figure
---

```{.mist .ignoreErrors .numberLines}
#[test]
fn trace_body() {
  check_trace(
    r#"
fn f() {
  let x = $012$0;
}
    "#,
    expect!(@r###"
    method f() {
      var x_1: Int := 12;
                      ^^
    }
    "###),
  );


//@ align
  check_trace(
    r#"
fn f() {
  let x = 10 + $012$0 + 14;
}
    "#,
    expect!(@r###"
    method f() {
      var _2: Int := (10 + 12);
                     ^^^^^^^^^
      var x_1: Int := (_2 + 14);
                      ^^^^^^^^^
    }
    "###),
  );
}
```

> [!caption]
> A `#[test]` function extracted from the Mist codebase, testing back tracing in generated Viper program.
---
tags: figure
---

```{.mist .ignoreErrors}
struct Fn { syntax: SyntaxNode }
impl Fn {
  fn fn_token(self)        -> Option<Token>     { self.token(T![fn]) }
  fn name(self)            -> Option<Name>      { self.child() }
  fn param_list(self)      -> Option<ParamList> { self.child() }
  fn arrow_token(self)     -> Option<Token>     { self.token(T![->]) }
  fn ret(self)             -> Option<Type>      { self.child() }
  fn conditions(self)      -> Vec<Condition>    { self.children() }
  fn decreases(self)       -> Option<Decreases> { self.child() }
  fn body(self)            -> Option<BlockExpr> { self.child() }
  fn semicolon_token(self) -> Option<Token>     { self.token(T![;]) }
}
```

> [!caption]
> A slightly modified definition for the `Fn` AST syntax node.

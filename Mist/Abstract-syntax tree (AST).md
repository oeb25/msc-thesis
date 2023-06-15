---
tags: subsubsection
---

The _abstract-syntax tree_ layer provides a fully type-safe way to traverse syntax. Contrary to how ASTs are often implemented, that is, using in-memory data types with concrete fields for children, the AST in Mist is only a projection over the CST. This allows high reuse of the underlying data and minimizes overall memory usage.

Each node in the AST contains a pointer into the CST and provides type-safe accessors for children and even parents. These accessors are generated directly from the grammar (see [[Appendix – The Mist Grammar]]), resulting in about 3500 lines of highly repetitive code. AST nodes also maintain the current span in the source file, allowing referencing back to the original source code for providing diagnostics and semantic highlighting.

```rust
enum Item {
  Fn(Fn),
  Struct(Struct),
  TypeInvariant(TypeInvariant),
}
```

```rust
struct Fn { syntax: SyntaxNode }
impl Fn {
  fn fn_token(&self) -> Option<SyntaxToken> { .. }
  fn name(&self) -> Option<Name> { .. }
  fn param_list(&self) -> Option<ParamList> { .. }
  fn thin_arrow_token(&self) -> Option<SyntaxToken> { .. }
  fn ret(&self) -> Option<Type> { .. }
  fn conditions(&self) -> AstChildren<Condition> { .. }
  fn decreases(&self) -> Option<Decreases> { .. }
  fn body(&self) -> Option<BlockExpr> { .. }
  fn semicolon_token(&self) -> Option<SyntaxToken> { .. }
}
```
---
tags: figure
---

```{.mist .ignoreErrors .numberLines}
#[salsa::input]
struct SourceFile { text: String }
#[salsa::tracked]
struct Parse { root: SyntaxTree, errors: Vec<ParseError> }

#[salsa::tracked]
fn parse(db: &dyn crate::Db, file: SourceFile) -> Parse {
  mist_syntax::parse(file.text(db))
}
#[salsa::tracked]
fn items(db: &dyn crate::Db, file: SourceFile) -> Vec<Item> {
  let root = parse(db, file).root;
  // ... computes hir and generates items ...
}
#[salsa::tracked]
fn mir(db: &dyn crate::Db, item: Item) -> mir::Body {
  // ... computes mir and does analysis ...
}
#[salsa::tracked]
fn viper_files(db: &dyn crate::Db, body: mir::Body) -> Vec<Viper> {
  // ... generates viper files(s) necessaray to verify body ...
}
#[salsa::tracked]
fn verify(db: &dyn crate::Db, vpr: Viper) -> VerificationResult {
  // ... calls viperserver ...
}
```

> [!caption]
> A slightly simplified version of the compiler source code.

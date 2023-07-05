---
tags: figure
---

```{.mist .ignoreErrors .numberLines}
fn drive(db: &dyn crate::Db, file: File) {
  for item in items(&db, file) {
    let body = mir(&db, item);
    for vpr in viper_files(&db, body) {
      let res = verify(&db, vpr);
      println!("{res:?}");
    }
  }
}
fn main() {
  let mut db = Database::default();
  let file = SourceFile::new(&db, r#"
    fn f() {
      assert 1 == 2;
    }
  "#);
  drive(&db, file);
  file.set_text(&mut db).to(r#"
    fn f() { assert 1 == 2; }
  "#);
  drive(&db, file);
}
```

> [!caption]
> A mock function driving the compiler, using functions defined in [[Figure – Salsa SourceFile example]].

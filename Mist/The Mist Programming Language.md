---
tags: chapter
---

**Chapter status:**

- Barely started.
- Just move on to next.

---

- Mist is a Rust derivative with Viper concepts, which is to say that it looks like Rust, but without many features including life-times and traits, and with additional annotations to specify logical properties and invariants.
- The grammar for the language is in [[Appendix – The Mist Grammar]].
- Supports limited type inference, requiring method signatures to be annotated, but with full inference inside of method bodies.
- It has an ownership system, which is checked using quantifiable permissions in Viper.
- Mist uses a _nominal type system_ [see @pierceTypesProgrammingLanguages2002, pp. 251-54].

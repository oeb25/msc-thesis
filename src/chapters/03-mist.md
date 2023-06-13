# The Mist Programming Language {#chap:mist}

- Mist is a Rust deriviative with Viper concepts, which is to say that it looks
like Rust, but without many features including life-times and traits, and with
additional annotations to specify logical properties and invariants.
- The grammar for the language is in +@app:mist-grammar.
- Supports limited type inference, requiring method signatures to be annotated,
but with full inference inside of method bodies.
- It has an ownership system, which is checked using quantifiable permissions
in Viper.
- Mist uses a _norminal type system_ [see @pierceTypesProgrammingLanguages2002,
pp. 251-54].

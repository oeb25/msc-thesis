---
tags: chapter
---

The Mist compiler^[https://github.com/oeb25/mist] is written in Rust and is heavily inspired by the architecture of both `rustc` [[@RustCompilerDevelopment2018]]  and `rust-analyzer` [[@RustAnalyzerArchitecture2023]]. Similar to both is that it uses an incremental query-based model at every compilation step, discussed in detail in [[Compilation structure]]. Conceptually, the compiler follows a standard compilation model [[@ahoCompilersPrinciplesTechniques1986#p. 5]], but with modifications to support interactive editing feedback, discussed in detail in [[Language server integration]]. We outline the architecture of the compilation structure in [[Compilation structure]], where the concrete objects used during compilation, and their purposes, are only referenced briefly and later described in more detail in [[Compilation stages]].

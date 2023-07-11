---
tags: section
---

Program verification is a highly interactive task, where the programmer and verifier are in constant dialogue, as specification and implementation continuously change in response to verification errors. Additionally, the editor should provide the programmer with valuable hints and annotations, aiding their task.

In Mist, we have an implementation of the _Language server protocol_ (LSP), [[@lsp]] which is a standard for editor and programming language tooling communication. By having an LSP implementation, we can provide editor features to any editor which supports LSP.

The Mist LSP provides _semantic highlighting_, inline _syntax and type errors_, _goto definition_, and _types on hover_. Additionally, this is one of the primary ways of verifying Mist programs. By spawning a verification process every time parts of programs changes, we can deliver responses as quickly as possible. Following the principle of modularity described in [[Modes and modularity]] and the compilation structure described in [[Compilation structure]], we split verification into smaller pieces, which are verified independently and cached upon completion. Having results cached means that the LSP can significantly speed up perceived verification time and, in some cases, provide instant results. LSP has previously been used for verification, [[@viperserver]] and extensions to LSP have been proposed to support verification-like procedures. [[@raskDecouplingCoreAnalysis2021]] [[@raskSpecificationLanguageServer2021]]

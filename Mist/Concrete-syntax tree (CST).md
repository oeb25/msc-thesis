---
tags: subsubsection
---

The concrete-syntax tree is a full-fidelity representation over source code, meaning no information about the original code is lost. Conceptually, the goal of a CST is to provide a semi-structured view into the source code, by storing a hierarchy of syntax-nodes and tokens, which maintains a complete mapping back to the original source code.

The job of the parser is to construct such a CST, by consuming tokens are adding nodes to the tree. Internally the CST maintains the _kind_ of syntax-nodes and tokens inserted by the parser, but does nothing to prevent the parser from emitting malformed language constructs. This is by design, as we want the CST to be able to contain partial or invalid syntax. Doing so, allows the compiler to optimistically continue its compilation process, even when the source program contains syntax errors and/or is actively being written.

Relating the CST to the grammar (see [[Appendix – The Mist Grammar]]) node kinds represent productions and terminals for syntax-nodes and tokens respectively. For all syntactically valid programs, the corresponding CST will have a structure conforming the to the hierarchy described in the grammar. But for invalid programs, children of nodes might have a mixture of allowed and disallowed syntax kinds.

This means that all queries performed on the CST must account of the partial nature of the tree, and thus the caller must consider both cases where the expected node exists, and where it does not.

![[Figure – CST Example]]

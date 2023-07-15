---
tags: subsubsection
---

The concrete-syntax tree is a full-fidelity representation over source code, meaning no information about the original code is lost. Conceptually, the goal of a CST is to provide a semi-structured view into the source code by storing a hierarchy of syntax nodes and tokens, which maintains a complete mapping back to the original source code.

The parser's job is to construct such a CST by consuming tokens and adding nodes to the tree. Internally the CST maintains the _kind_ of syntax nodes and tokens inserted by the parser but does nothing to prevent the parser from emitting malformed language constructs. This is by design, as we want the CST to be able to contain partial or invalid syntax. Doing so allows the compiler to optimistically continue its compilation process, even when the source program contains syntax errors and when being actively written.

Relating the CST to the grammar (see [[Appendix – The Mist Grammar]]), node kinds represent productions and terminals for syntax nodes and tokens, respectively. For all syntactically valid programs, the corresponding CST will have a structure conforming to the hierarchy described in the grammar. However, for invalid programs, children of nodes might have a mixture of allowed and disallowed syntax kinds.

Therefore all queries performed on the CST must account for the tree's partial nature, and thus the caller must consider both cases where the expected node exists and where it does not.

![[Figure – CST Example]]

[[Figure – CST Example]] shows how the CST will look for a simple function annotated above the graph. Notice how the function is actually _syntactically invalid_: it is missing a return type after `->` and a semi-colon after `5`. The CST, however, is still produced, but any query for the missing elements will return `None`.

Another fact worth pointing out is that nodes are _interned_. Interning means that if a subtree is identical to another, then only one copy will be used, and the two places it is used will contain a pointer to the same tree. In [[Figure – CST Example]], the `WS` token with a single space occurs eight times, but all occurrences will point to the same node in memory.

Interning, however, conflicts with another property we want, which is _source spans_ of nodes. If each node carried its span, it would inherently be unique and disqualified for interning. Each node contains its length to solve this dispute, and children will have a _relative offset_. Thus, if we know the absolute span of a node, then we can use the relative offsets to compute the absolute spans of its children ad-hoc. We know the absolute span of the root node, so by induction, we can determine source spans for all nodes when traversing the tree.
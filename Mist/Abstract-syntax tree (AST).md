---
tags: subsubsection
---

The _abstract-syntax tree_ layer provides a fully type-safe way to traverse syntax. Contrary to traditional AST implementations, using in-memory data types with concrete fields for children, the AST in Mist is only a _projection over the CST_. Projections allow high reuse of the underlying data, minimizing consumed memory.

Each AST node contains a pointer into the CST and provides type-safe accessors for children and even parents. These accessors are generated directly from the grammar (see [[Appendix – The Mist Grammar]]), resulting in about 3500 lines of highly repetitive code. AST nodes also maintain their span in the source file to provide source code locations for diagnostics and semantic highlighting.

![[Figure – AST Fn Sample]]

[[Figure – AST Fn Sample]] shows how an AST syntax node might be defined in the auto-generated file. We see a function for each child, which queries the stored `SyntaxNode` for the requested kind.

> [!remark]
> Notice how some of the bodies appear identical in [[Figure – AST Fn Sample]], but return different types. This is because `self.child` has a _generic return type_, and thus can find the child of the specific kind among its children. For tokens, where we cannot use the return type to determine the returned kind, `self.token` takes an argument of the desired kind and returns the first token of that kind.

AST nodes are short-lived objects created when traversing a tree and get thrown out immediately as the node leaves scope. However, we often want to store select AST nodes in persisted structures, such as HIR definitions. Storing AST nodes directly is not an option, as they contain non-atomic reference-counted pointers into the CST and thus cannot be shared across threads. The solution to storing AST nodes is to instead store a _span_ into the CST and recompute the node as needed, given the root of the CST.

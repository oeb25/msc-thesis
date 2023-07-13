---
tags: section
---

Mist has _structs_, which are custom data types that contain a collection of named fields for grouping data together. Structs have explicit names, which allows them to be referred to in types, and to be distinguished from other similar-looking structures, thus qualifying as _nominal types_ [[@pierceTypesProgrammingLanguages2002#pp. 251-254]].

In addition to having fields, we can give structs _invariants_, which are properties any instance of such a struct _must satisfy_.

![[Example – BTree – Struct]]

To illustrate structs, we consider [[Example – BTree – Struct]], where `BTree` has three fields, one of which is an `int` $\lineref{2}$, and the other two are binary-trees $\lineref{3, 4}$. The two children are _optional_ as indicated by the leading `?`, meaning that their value may or may not be present, and if not, will be `null`.

The `invariant BTree` $\lineref{9}$ imposes a _full binary-tree_ [[@paule.blackFullBinaryTree]] constraint on the structure, effectively saying that any `BTree` with a left child must have a right child, and vice versa. In the invariant, `self` refers to an instance of `BTree`. The function, `fn leaf`, shows how a `BTree` might be instantiated $\lineref{7}$, and following the previous invariant, the newly constructed `BTree` must have two children or none, of which the latter is true.

![[Example – BTree – Values]]

In [[Example – BTree – Values]], we see the recursive function `@values` $\lineref{12}$, which computes the set of values in an _immutable reference_ to a `BTree`. As indicated by leading `&`, immutable references prevent any mutation from happening to the passed object and require that we return them entirely as given by the end of the function.

When dealing with sets in the body of `@values` _post-fix functions_ are used, such as `.to_set()` and `.union()`, to convert from a list and combine two sets, respectively.

Accessing an optional field requires checking that it is not null before usage. The `if` $\lineref{14}$ only allows the subsequent line to execute if `t.left` is present. Consequently, this also means that `t.right` will be, by the invariant in [[Example – BTree – Struct]]. Using the fields, however, requires adding a _not-null_ post-fix operator `!` to the end, which in contrast to similar operators in TypeScript (`!`)^[https://www.typescriptlang.org/docs/handbook/release-notes/typescript-2-0.html#non-null-assertion-operator] and Kotlin (`!!`)^[https://kotlinlang.org/docs/null-safety.html#the-operator], will be _verified statically_ rather than at runtime.

In Mist, `if` is not a statement but rather an expression, allowing it to be used as such, seen in the body of `@values` $\lineref{14}$. A similar thing is seen in `@height` of [[Example – BTree – Height]] $\lineref{20}$, where the returned value is computed by the `if` $\lineref{21}$, and similarly by the inner `if` $\lineref{28}$ choosing the height of the taller child.

![[Example – BTree – Height]]

Multiple invariants are allowed for structs in Mist, and thus by adding [[Example – BTree – Height invariant]], we require that siblings in any `BTree` must have the same height. This invariant effectively makes all `BTree` _complete binary-trees_.

![[Example – BTree – Height invariant]]

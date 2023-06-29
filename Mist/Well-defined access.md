---
tags: subsection
---

It may not always be the case that an instruction can be executed, such as if it refers to a place that does not exist or is not well-typed. Additionally, there might be cases where execution is possible, but doing so would result in a panic or crash, such as division by zero.

Folding analysis, however, only considers well-typed programs with fully resolved places. This is valid since the previous stages of the compiler (specifically during HIR construction and type-checking, see [[Types in Mist]] and [[High-level IR (HIR)]]) prevents malformed programs from reaching this point.

Thus, we need a separate notion of a _well-defined execution_ for FIR instructions to specify which executions are valid in relation to specific folding trees.

![[Definition – FIR well-defined access rules]]

The rules for the `fold` and `unfold` FIR instructions exactly mirror those of performing $\fold(\rho, \T)$ and $\unfold(\rho, \T)$ defined in [[Definition – Folding]]. The two others state that places being read has to be folded in $\T$, while any place written to has to be accessible in $\T$ but not necessarily folded.

> [!remark]
> Not requiring written places to be folded might seem strange, but it comes from overwriting it, thus making it inaccessible. This drops any field it might have had and makes its invariants irrelevant. It is, however, important that the place is _accessible_, or in other words, that its parent is unfolded, as performing the assignment needs access to the fields of the parent.

Notice that nothing in [[Definition – FIR well-defined access rules]] folds or unfolds. The access rules only describe _when an instruction is well-typed_ not what it _produces_, which is the described by semantics.

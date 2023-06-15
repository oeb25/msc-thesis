---
tags: subsection
---

It may not always be the case that an instruction can be executed, such as if it refers to a place that does not exist or is not well-typed. Additionally, there might be cases where execution is possible, but doing so would result in a panic or crash, such as division by zero.

Folding analysis, however, only considers well-typed programs with fully resolved places. It is allowed to do so since the previous stages of the compiler (specifically during HIR construction and type-checking, see [[Types in Mist]] and [[High-level IR (HIR)]]) prevents malformed programs from reaching this point.

Thus, we need a separate notion of a _well-defined execution_ for FIR instructions to specify which executions are valid in relation to specific folding trees.

![[Definition – FIR well-defined access rules]]

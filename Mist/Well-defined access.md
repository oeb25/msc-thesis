---
tags: subsection
---

It may not always be the case that an instruction can be executed, such as if it refers to a place that does not exist or is not well-typed. Additionally, there might be cases where execution is possible, but doing so would result in a panic or crash, such as division by zero.

Folding analysis, however, only considers well-typed programs with fully resolved places. This is valid since the previous stages of the compiler (specifically during HIR construction and type-checking, see [[Types in Mist]] and [[High-level IR (HIR)]]) prevents malformed programs from reaching this point. Furthermore, folding analysis expects some additional properties to be hold, as detailed in [[Definition – FIR well-defined access rules]].

Thus, we need a separate notion of a _well-defined execution_ for FIR instructions to specify which executions are valid in relation to specific folding trees.

![[Definition – FIR well-defined access rules]]

The rules for the `fold` and `unfold` FIR instructions mirror those of performing $\fold(\rho, \T)$ and $\unfold(\rho, \T)$ defined in [[Definition – Folding]]. The two others state that places being read has to be folded in $\T$, while any place written to has to be accessible in $\T$ but not necessarily folded.

> [!remark]
> The assignment instruction contains some details which are not immediately apparent, meriting exploration.
>
> Firstly, not requiring written places to be folded might seem strange, but it comes from overwriting it, thus making it inaccessible. This drops any field it might have had and makes their invariants irrelevant. It is, however, important that the place is _accessible_, or in other words, that its parent is unfolded, as performing the assignment needs access to the fields of the parent.
>
> Secondly, since $\pread(a) \subseteq \leaves(\T)$ and $\leaves(\T)$ is a compatible set by [[Proposition – Leaves are compatible]], then so is $\pread(a)$. This being a compatible set has certain limitations of which expressions are allowed, further explored in [[Limitations of current folding analysis]]. The same holds for $\Rho$ in $\iuse\;\Rho$. The reason we can assume that the sets are compatible, however, is because during compilation, we can check this property and prevent malformed programs from ever reaching FIR analysis.
>
> Lastly, is an interaction between the two requirements. For $\rho \in \T$, it means that $\rho$ cannot be a descendant of anything $\pread(a)$. This prevents spurious statements such as `x.f = x`, where the ownership of `x` becomes ill-defined. The limitations of this interaction is discussed further in [[Limitations of current folding analysis]].

If an instruction is well-defined for multiple folding trees, then a neat property is that the instruction will also be well-defined for the $\meet$ of those trees.

![[Lemma – Meet of well-defined is well-defined]]

![[Proof – Meet of well-defined is well-defined]]

Notice that nothing in [[Definition – FIR well-defined access rules]] folds or unfolds. The access rules only describe _when an instruction is well-typed_ not what it _produces_, which is the described by semantics.



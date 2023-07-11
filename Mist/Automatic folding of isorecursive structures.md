---
tags: chapter
---

In Mist, types such as `struct`s and `enum`s are named, allowing them to be referenced inside other types, in function arguments, and local variables (see [[Types in Mist]]). In addition to being a collection of fields, they can also carry logical properties with `invariant`s. From a programmer's perspective, these fields and properties can be accessed at any point in the program, and the invariants can usually be assumed to hold without the need for any additional annotation.^[Note: Due to this, we say that the Mist source language has _transparent_ types, but it's not important.]

This property, however, introduces an implicit guarantee about where and when the invariants of a type hold. This is tricky when we, for example, modify the internals of a struct in a sequence of operations, and part way through the mutation, the invariants only partially hold.

![[Figure – Breaking invariant]]

> [!example]
> Consider the program in [[Figure – Breaking invariant]]. The `struct X` has an `invariant` \lineref{4} establishing a relationship between `.u` and `.v`.
>
> With this, the programmer can assume that when given a reference to type `S` \lineref{6}, its field `.x` of type `X` satisfies the invariant of `X`. Thus the assertion \lineref{7} must hold.
>
> The invariant, however, is temporarily broken when `s.x.u` is incremented \lineref{9} only to be restored again when `s.x.v` is decremented \lineref{10}.

In Mist, _temporary invalidation_ of invariants is allowed, and we let the compiler figure out when invariants must hold and when they may temporarily be broken.

The goal is thus for the compiler to infer what we call _folding points_, which are locations in a function where we either can assume the properties of an invariant (an _unfold_) or where we must assert that an invariant holds (a _fold_).

Each folding is associated with a place (as a reminder, a place is a local and sequence of fields, see [[Definition – Place]]). We say that when we unfold a place, it becomes unfolded and so becomes its type; conversely, the same is true for fold. Additionally, only folded places can be unfolded, and only unfolded places can be folded.

Conceptually, this means that all places and types have two states, folded and unfolded, and such types are called _isorecursive types_.

> [!remark]
> For the most part, languages has _equirecursive types_ as opposed to _isorecursive types_. With equirecursive types, there is no difference between having a named type and having access to its fields (they are equivalent), but with isorecursive types, these two states are distinct. Going between the two states, we refer to as _fold_ and _unfold_, but common terminology is also _roll_ and _unroll_.

In addition to controlling invariants, folding also limits field access such that a place must be unfolded for its fields to be accessible, and this is exactly what the compiler uses to determine folding places.

> [!example]
> Consider again the program in [[Figure – Breaking invariant]]. In the first assertion (repeated below), the nested field `.x.v` needs to be accessible on `s`.
> ```{.mist .ignoreErrors}
> assert s.x.v + s.x.u == 0;
> ```
> This means that not only do `s` need to be unfolded, but also `s.x`. Incidentally, `.x.u` is also accessed, which requires the same unfolded places.
>
> Moreover, the argument `s: &mut S` requires that upon returning, `s` and anything it might contain will be folded. This ensures that invariants of `s`, and any nested invariants hold when the function exits.

In this chapter we will introduce a formal definition of _folding trees_ in [[Folding tree structure]] as the foundation for precisely describing folding requirements, then go on to describe the semantics of the language with respect to foldings and an analysis for computing sufficient foldings in [[Folding analysis]], and finish with a proof that we can augment any program to be sound in terms of foldings in [[Folding augmented programs]].

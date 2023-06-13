---
tags: chapter
---

In Mist, types such as `struct`s and `enum`s are named, allowing them to be referenced inside other types, in function arguments, and local variables (see [[Types in Mist]]). In addition to being a collection of fields, they can also carry logical properties with `invariant`s. From a programmer's perspective, these fields and properties can be accessed at any point in the program, and the invariants can for the most part be assumed to hold, without the need for any additional annotation.^[Note: Due to this, we say that the Mist source language has _transparent_ types, but it's not important.]

This property, however, introduces an implicit guarantee about where and when the invariants of a type holds. This is tricky when we, for example, modify the internals of a struct in a sequence of operations, and part way through the mutation the invariants only partially hold.

![[Figure – Breaking invariant]]

> [!example]
> Consider the program in [[Figure – Breaking invariant]]. The `struct X` has an `invariant` \lineref{4} establishing a relationship between `.u` and `.v`.
>
> With this, the programmer can assume that when given a reference to type `S` \lineref{6} then it's field `.x` of type `X` satisfies the invariant of `X`. Thus the assertion \lineref{7} must hold.
>
> The invariant, however, is temporarily broken when `s.x.u` is incremented \lineref{9} only to be restored again when `s.x.v` is decremented \lineref{10}.

In Mist, we want _temporary invalidation_ of invariants to be allowed, and we want the compiler to figure out^[Authors note: I don't use "infer" here on purpose, since we will use it in a more formal sense later] when invariants must hold and when it's okay for them not to.

The goal is thus for the compiler to infer what we call _folding points_, which are locations in a function were we either can assume the properties of an invariant (an _unfold_), or where we must assert that an invariant holds (a _fold_).

Each folding is associated with a place (as a reminder, a place is a slot and sequence of fields, see [[Definition – Place]]). We say that when we unfold a place, that place becomes unfolded and so is its type, and conversely the same is true for fold. Additionally, only folded places can be unfolded and only unfolded places can be folded.

Conceptually, this means that all places and types have two states, folded and unfolded, and such types are called _isorecursive types_.

> [!remark]
> For the most part, languages has _equirecursive types_ as opposed to _isorecursive types_. With equirecursive types there's no difference between having a named type, and having access to its fields (they are equivalent), but with isorecursive types these two states are distinct. Going between the two states we refer to as _fold_ and _unfold_, but common terminology is also _roll_ and _unroll_.

In addition to controlling invariants, folding also limits field access such that a place must be unfolded for its fields to be accessible, and this exactly what the compiler uses to determine folding places.

> [!example]
> Consider again the program in [[Figure – Breaking invariant]]. In the first assertion (repeated below) the nested field `.x.v` needs to be accessible on `s`.
> ```{.mist .ignoreErrors}
> assert s.x.v + s.x.u == 0;
> ```
> This means that not only does `s` need to be unfolded, but also `s.x`. Incidentally, `.x.u` is also accessed, which requires the same unfolded places.
> 
> Moreover, the argument `s: &mut S` requires that upon returning, `s` and anything it might contain, will be folded. This makes sure that invariants of `s`, and any nested invariants, hold when the function exits.

To reason about these foldings more formally, we need a data structure to lay a foundation precisely describing folding requirements.

%%---

The lower layers of the compiler, therefore, work with _isorecursive_ types.

With isorecursive types, there is a distinction between having a named type and having an unfolded type. To read or update a field of a named type, it must first be _unfolded_. In contrast, when referencing a named type, it must be _folded_. Whenever a type is folded, its invariant is held internally, and when unfolded that invariant can be subsequently assumed. For unfolded types, their invariant may or may not hold, but when the type is folded, its invariant is asserted.

An example of folding in action can be seen in [[Figure – Breaking invariant]]. One interesting detail to highlight is the fact that the invariant on `X` establishes a relationship between `.u` and `.v` \lineref{4}, that after performing the increment of `s.x.v` \lineref{7} is broken until the increment of `s.x.u` \lineref{8} has occurred. This is fine since the invariant is first required to hold at the folding point at the end of the scope \lineref{11}.

To transform a program from its equirecursive form into its isorecursive form, we must compute the necessary foldings and unfolding. This is done through a series of analyses, which are covered in this chapter, but as a foundation for these analyses, we introduce a data structure and notation to reason about foldings.%%

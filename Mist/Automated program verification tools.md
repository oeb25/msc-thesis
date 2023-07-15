---
tags: section
---

Program verification is the process of checking that a program behaves as expected. We do this by annotating code with what must be true, known as assumptions and assertions, at specific points in the program and then machine-checking that these conditions hold. We express conditions as logical predicates, which can refer to variables and depend on properties shown in other parts of the program.

Assertions can be added in many places and come in many forms, either explicit or implicit. Among explicit assertions are preconditions and postconditions, propositions that must hold at the start and end of methods, and loop invariants, which must hold at every iteration of a loop. Implicit assertions are those _inferred_ by the compiler or verifier, such as structural invariants, which are properties that hold for some _type_ or _data structure_, or assertions about _resource availability_.

Viper [[@mullerViperVerificationInfrastructure2016]] is described as a _verification infrastructure for permission-based reasoning_, and is the umbrella project for the Silver [[@silvercontributersViperprojectSilverDefinition]] intermediate verification language, and Carbon [[@carboncontributersCarbon2023]] and Silicon [[@schwerhoffAdvancingAutomatedPermissionBased2016]] backends. Viper allows for the verification of imperative code using the kinds of assertions referred to above. In many ways, it allows for the natural expression of algorithms while providing tools for defining mathematical concepts to verify a piece of code.

Verification tasks can quickly become large, and in those cases, it is often necessary to limit the search space to guide the automatic solver in the right direction. One such place where Viper does this is by use of _predicates_. Predicates are named packaging of logical properties that specifications can refer to.

![[Figure – Program verification predicate]]

[[Figure – Program verification predicate]] shows `@vpr invariant Sorted` containing the logical definition of a sorted list. The functions below both take a sorted sequence of three elements, where the first is positive and add them together. We then assert that the sum is no less than three, which comes from the fact that all three numbers must be positive since the sequence is sorted. The first `@vpr fn fails` this check since the logical properties of being sorted are packaged away in the `@vpr invariant`. However, the second `@vpr fn works` by first _unfolding_ the predicate and thus explicitly introducing the internals of the `@vpr invariant Sorted` to the prover.

This property of hiding internals of logical predicates is known as _isorecursive types_, and we explore it in detail in [[Automatic folding of isorecursive structures]].

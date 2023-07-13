---
tags: section
title: "Case study 2: Ascending sum"
---

The second program we will examine, is one computing the sum of a sorted list of integers stored in a struct. The Mist implementation is presented in [[Figure – Ascending sum – Mist]].

![[Figure – Ascending sum – Mist]]

### Program outline

The `struct Sorted` $\lineref{1-4}$ contains two fields: `xs: [int]` is a sorted list, and `ghost min: int` is the minimum element of `xs`. Just below the struct are two invariants, which asserts the mentioned properties of the fields. The first `invariant` $\lineref{6-10}$ uses `forall-in` syntax to range over all `i` and `j` satisfying `0 <= i <= j < xs.len`, and states that all ascending indices will have ascending values in `xs`, and thus be sorted. The second `invariant` $\lineref{11-13}$ says that if `xs` is not empty, then `min` will be the value of the first entry in `xs`. Note that this `xs` _is empty_, then no constraint is enforced on `min` by these invariants.

The function `fn sum` $\lineref{15}$ takes a _shared reference_ to `@Sorted` and ensures $\lineref{16}$ that the produced value will be larger than the smallest value in the list times the number of elements. Additionally, the annotation `dec _` $\lineref{17}$, says that `@sum` must terminate, where `_` means that the termination criteria is inferred, which for `@sum` means that all statements in its body must be terminating.

In the body of `@sum`, we declare the mutable variable `sum` to be initially zero. Then we iterate the indices of `xs` using a `for-in` loop, with the invariant that `sum` is larger than the current index times the smallest value in `xs`. In the loop-body, we add the value at the current index to `sum`.

### Hidden assertions

> [!paragraph] Shared-reference
> 
> The parameter `s: &Sorted` is a shared-reference to an object, and thus must give back `s` by the end of the function, exactly as it was at the beginning of the function. The first is that it and its fields must be the same as initially. Second, it must give back the _same amount of permissions_, since Mist uses quantified permissions.
>
> We enforce this last criteria by having callers of `@sum` pass the amount of permission they are lending of `s` along side the reference `&Sorted`. By letting the caller of `@sum` choose how much permission they are giving away, we allow the permission to be dynamically determined.

> [!paragraph] Ghost code
> 
> The field `min` is declared `ghost`, meaning that every time we use it, any code it touches will also become ghost, and thus we have to make sure that it does not leak into non-ghost code.

> [!paragraph] Termination
> 
> In the case of `@sum`, three statement must be checked:
> 
> - The assignment `let mut sum = 0;` terminates trivially.
> - The compiler checks that the loop ranges over a _finite bounded interval_ and thus will terminate.
> - The update `sum = sum + s.xs[index];` also terminates trivially.

> [!paragraph] Loop invariants
> 
> In the original program, the loop had one explicit invariant, but from `s: &Sorted` we have to add a few more invariants:
> 
> - The loop must retain the exact amount of permission we have to `s`.
> - The invariants for the `@Sorted` struct must hold at each iteration.
> - The value of `s` and its fields must not have changed since the start of the function.

> [!paragraph] Unfolding
> 
> Each time a field of `s` is accessed, it must be appropriately unfolded. Conversely, some parts of the function requires `s` to be folded, namely at the start and end of loop, and by the end of the function, and thus we must fold appropriately. These folding points are all determined by the analysis described in [[Automatic folding of isorecursive structures]].

```{=tex}
\newpage
```

![[Figure – Ascending sum – Viper]]

### Comparative Viper program

To get a feel for the annotations the Mist compiler adds to verify the program, we present an comparative Viper program.

Mist generates one or more Viper programs which it sends to `viperserver` for verification. The generated program can become quite unreadable, since they contain mangled names and are noted formatted for readability. The program [[Figure – Ascending sum – Viper]], is _not an auto-generated_ program, but rather a handwritten implementation of the same program as [[Figure – Ascending sum – Mist]].

> [!paragraph] Predicate
> 
> Viper does not have structs^[The ADT Viper plugin [[@maissenAddingAlgebraicData2022]] gets close, but does not have invariants.] but has fields and predicates which can emulate the same principle. In Mist we could split field declarations and the two invariants into three pieces, but in Viper one predicate encapsulates all invariants and field access, while field declarations are not linked to specific structs and can be combined ad-hoc.

> [!paragraph] Ghost code
> 
> Viper does not have any notion of ghost code, and we must thus track this manually without verification.

> [!paragraph] Quantifiers
> 
> Viper needs explicit declaration of quantified variables and separate condition to bound them, while Mist has syntax sugar for quantifiers allowing expressing using ranges with inference for variable types.

> [!paragraph] Shared references
>
> Viper uses `@vpr Ref` to track objects, and associates predicates separately in the function specification. When declaring a predicate holds for a reference, i.e. `@vpr acc(Sorted(s), p)`, we also need to give an explicit quantified permission. Since we want the permission to by dynamically chosen by the caller, the permission `p` is passed by the caller, and we can then use this to also ensure that this is the amount of permission given back in the end.
> 
> Additionally, in the postcondition, we must check that none of the fields have changed values since the start of the function.

> [!paragraph] Equality
> 
> Equality (`==`) of `@vpr Ref`'s in Viper is _referential equality_ where two objects are equal iff they _are the same object_. This is in contrast to _structural equality_ where two objects are equal iff their fields are structurally equal. When comparing `s` to its original we want structural equality, and thus must compare each field. Using Viper macros, `@vpr define SortedEq` $\lineref{11}$ expresses structural equality for `Sorted`, making the code more readable.

> [!paragraph] While-loop
>
> A `while` loops over the indices of `s.xs`, where the invariants must maintain that `index` stays within the expected range. The same conditions checked in the postconditions must also hold in the loop-invariants.

> [!paragraph] Unfolding
>  
> Seen throughout the Viper program are explicit unfoldings, of which there are two kinds. Firstly, `@vpr unfolding-in` are expressions which only unfolds a predicate in the contained subexpression. Secondly, `unfold` and `fold` statements $\lineref{27, 30}$ are procedural foldings, which mean they alter the state, and to maintain access, each unfolding must be paired with a folding with the same quantity.

```{=tex}
\newpage
```

![[Figure – Ascending sum – Dafny]]

### Comparative Dafny program

Another verification language is Dafny [[@leinoDafnyAutomaticProgram2010]], discussed further in [[Dafny comparison]], which carries many syntactic similarities to Viper. [[Figure – Ascending sum – Dafny]] shows an implementation of the [[Figure – Ascending sum – Mist]] program again.

> [!paragraph] Classes and predicates
>
> In the Dafny program, we use `@vpr struct Sorted` to define the structure with fields. The declaration `@vpr ghost let min: int` highlights that _Dafny does have ghost code analysis_. Structural invariants, however, are described in a separate `@vpr ghost invariant`, quite similar to Viper. This separation means that anytime we expect an instance `s` of `Sorted` to be sorted, we have to require `IsSorted(s)`.

> [!paragraph] Access
>
> One thing that sets Dafny apart from Mist and Viper, is that Dafny does not have quantified permissions. Dafny instead uses `@vpr reads` and `@vpr modifies` clauses to describe how an object may be accessed and changed. Since there is no modifies clause on `@vpr fn sum`, we know that the passed `s` will remain unchanged by the end of the function, thus no explicit postcondition is necessary to check this. The `@vpr modifies s` clause $\lineref{16}$ is commented out to show where such a clause would go.

> [!paragraph] Function body
>
> The Dafny implementation looks almost exactly like the Mist implementation. Internally, Dafny automatically does ghost code analysis and termination checking.

%%$$
\forall \; 0 \leq i \leq j < |xs| : xs[i] \leq xs[j]
$$%%
---
tags: chapter
---

_Mist_ is a new programming language that combines different programming and software engineering ideas in a unified and coherent package. For those with knowledge of Rust, the syntax of Mist should be very familiar, and those proficient in languages like Dafny and Viper will find many verification constructs native to such languages to be recognizable in Mist.

![[Example – Add Mist example]]

[[Example – Add Mist example]] give a good introduction to many of the constructs in Mist. A function declaration has the same syntactic structure as in Rust, with added pre and postconditions indicated by the `req` and `ens` keywords; shorted versions of the `requires` and `ensures` keywords familiar to Viper users, who will also recognize `result` used in the postcondition, referring to the returned value of the function.

`req` and `ens` contains comma-separated boolean expressions, which must hold at the beginning and end, respectively, of the function they are annotating.

Quantifiers, such as `forall` and `exists`, allow specifying some boolean condition for a set of values. These values can be bounded by the `in`-range syntax like seen in [[Example – Add Mist example]] $\lineref{4, 9}$, syntactically similar to `for-in` loops $\lineref{7}$, which carries invariants indicated by the shorted `inv` $\lineref{8}$.

In Mist, parameters variables are _immutable by default_, meaning that reassignments and updates of variables are allowed only after adding a `mut` in front of the variable name, like done for `let mut z = [];` $\lineref{6}$. Notice how no type annotation is required in declarations since Mist has _local type inference_, and can, from context, figure out what type `z` needs to be. Mists notion of local type inference, as opposed to _global type inference_ seen in many ML style languages, requires explicit type declaration at the function boundaries, namely for parameters and return types. However, almost all types can be inferred within the body of a function. The declaration of `let mut z = [];` is an interesting case in this regard, as upon declaration, the empty list `[]` does not indicate what type the content of the list will be. Only by inspecting later expressions in the function does it become clear that it is `[int]`.

Return values in Mist are not named and will take the final expression value in the function body, as seen with the lone `z` $\lineref{13}$. However, Mist also allows explicit returns using the `return` keyword at arbitrary locations in a function.

%%
- Mist is a Rust derivative with Viper concepts, which is to say that it looks like Rust, but without many features including life-times and traits, and with additional annotations to specify logical properties and invariants.
- The grammar for the language is in [[Appendix – The Mist Grammar]].
- Supports limited type inference, requiring method signatures to be annotated, but with full inference inside of method bodies.
- It has an ownership system, which is checked using quantifiable permissions in Viper.
- Mist uses a _nominal type system_, [[@pierceTypesProgrammingLanguages2002#pp. 251-254]].
- Mist allows for shadowing.
- Mist has implicit return.
%%

---
tags: subsection
---

After the HIR stage, the code gets transformed into another representation, namely the _mid-level IR_ (MIR). The MIR representation is a control-flow graph (CFG) consisting of _basic blocks_ and edges between these blocks. Internally in basic blocks are sequence of instructions, which cannot branch, and a final terminator, which may or may not branch to another basic block.

[[Figure – Loop CFG]] shows how the MIR CFG will look for a select portion of the basic blocks of the function `@f` from [[Example – Desugaring for loop]].

![[Figure – Loop CFG]]


MIR consists of a much smaller and simpler set of constructs compared to HIR. Expressions in MIR, for example, cannot be recursive, and we must thus spread nested HIR operations into sequential MIR instructions in a post-order fashion. Additionally, expressions are limited to variable access and reference and unary and binary operations.

Expressions only occur in one instruction, namely _assignments_. All other HIR locations where expressions might be used, has to be first computed into a new _local_, and then be referenced later.

![[Definition – Locals]]

> [!example]
> Continuing [[Example – Desugaring for loop]], the function `@f` contains two named locals, `#1_sum` and `#2_i`, and approximately 8 temporary locals. In [[Figure – Loop CFG]], locals are prefixed with a `#`, where variables have a suffix starting with `_`.

Mist contains 4 instructions: `@Assign` consists of updating the value of some place with the result of an expression, like `#8 := (< #7 #4)`. `@NewAdt` are constructions for `struct`s an similar and stores it. `@Folding` is either a `fold` or an `unfold`, which is a concept discussed in great detail in [[Automatic folding of isorecursive structures]]. `@PlaceMention` simply refers to some some place, which is useful for data flow analysis.

Instructions are executed sequentially in a fixed order, unlike terminators, which can branch to multiple blocks. Some terminators such as `!goto` will always go to the same block, but others such as `!switch` can look at values and and branch to one of multiple blocks based on the outcome of the tested value. Another terminator, `!assert` checks a value, and jumps to another block. Assertions in Mist are only used by verifiers, and thus cannot influence control flow, like an assertion might do in a language without static assertions.

> [!example]
> In the process of lowering, the `dec`reases annotation $\lineref{18}$ gets wrapped around the body of the function, producing two new statements:
> ```{.mist .numberLines offset="19"}
> let variant = 10 - i;
> sum = sum - 1;
> i = i + 1;
> assert variant > 10 - i, 10 - i >= 0;
> ```
> In the MIR [[Figure – Loop CFG]], `variant` becomes `#4` and is assigned to in `:B2`. The expression is converted to prefix-notation `(- $10 #1_i)` and the `10` literal prefixed with a `$`, both done to reduce ambiguity.
> 
> The final assertions, checks that `variant` has decreased, and that the new value is still within the well-founded order. The way these are written as nested expressions, however, requires hoisting the each subexpression up. Firstly, the `10 - i` is assigned to a temporary `#7`, and then `#8 := (< #7 #4)` computes the boolean value for decreasing which is then asserted in the terminator `!assert #8 -> :B4`. The same is done in `:B4` to assert `#7` is non-negative.

When dealing with `struct`s, we need a way to refer to its fields, and for nested `struct`s we need to be able to refer to arbitrarily long sequence of fields. We call such accesses _places_.

![[Definition – Place]]

> [!example]
> Consider the `struct`s in the following snippet of a simplified model of humans an animals:
> ```{.mist .ignoreErrors}
> struct Human { child: Human, dog: Animal }
> struct Animal { age: int, color: Color }
> pure struct Color { r: int, g: int, b: int }
> ```
> If we have a local `Human` called `h`, then `h`, `h.dog`, `h.dog.color.r`, `h.child.dog`, and `h.child.child.dog.age` will all be potential places. In this case, the set of places, will be infinite, since `Human` is a recursive type.

Places will play a critical role in the following chapter, so it useful to define some operations on them. The first we define is the notion of a _prefix_ of a place.

![[Definition – Place Prefix]]

Prefixes are particularly useful when reasoning about all places used in the process of accessing a place. In this regard, we define an ordering of places to mean that any place in the prefix of another will be smaller than that place.

![[Definition – Place Partial-order]]

The final definitions, speaks about the related places of one place, namely those which are fields of the parent, or perhaps _siblings_.

![[Definition – Place siblings]]

After the MIR has been generated, it is still subject to mutations. The main objective for MIR is precisely to enable passes that potentially augment the CFG with additional instructions, change its structure, or perform optimisations. Additionally, CFG's are a well studies object with rich literature, [[@allenControlFlowAnalysis1970]] [[@nielsonPrinciplesProgramAnalysis1999]] used by many existing compilers, [[@midtgaardControlflowAnalysisFunctional2012]] where most inspiration for Mists architecture is taken from rustc. [[@matsakisIntroducingMIRRust2016]] [[@RustCompilerDevelopment2018]]

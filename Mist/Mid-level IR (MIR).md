---
tags: subsection
---

After the HIR stage, the code transforms into another representation, namely the _mid-level IR_ (MIR). The MIR representation is a control-flow graph (CFG) consisting of _basic blocks_ and edges between these blocks. Internally in basic blocks are a sequence of instructions, which cannot branch, and a final terminator, which may or may not branch to another basic block.

[[Figure – Loop CFG]] shows how the MIR CFG will look for a select portion of the basic blocks of the function `@f` from [[Example – Desugaring for loop]].

MIR consists of a much smaller and simpler set of constructs than HIR. Expressions in MIR, for example, cannot be recursive, and we must thus spread nested HIR operations into sequential MIR instructions in a post-order fashion. Additionally, expressions are limited to variable access and reference and unary and binary operations.

Expressions only occur in one instruction, namely _assignments_. All other HIR locations where expressions might be used must be computed into a new _local_ first and then referenced later.

![[Definition – Locals]]

> [!example]
> Continuing [[Example – Desugaring for loop]], the function `@f` contains two named locals, `#1_sum` and `#2_i`, and approximately 8 temporary locals. In [[Figure – Loop CFG]], locals are prefixed with a `#`, whereas variables have a suffix starting with `_`.

Mist contains four instructions: `@Assign` consists of updating the value of someplace with the result of an expression, like `#8 := (< #7 #4)`. `@NewAdt` are constructions for `struct`s and similar and stores it. `@Folding` is either a `fold` or an `unfold`, a concept discussed in great detail in [[Automatic folding of isorecursive structures]]. `@PlaceMention` refers to some place only used for analyses, such as data flow.

Instructions are executed sequentially in a fixed order, unlike terminators, which can branch to multiple blocks. Some terminators, such as `!goto`, will always go to the same block, but others, such as `!switch`, can look at values and the branch to one of the multiple blocks based on the outcome of the tested value. Another terminator, `!assert,` checks a value and jumps to another block. Assertions in Mist are only used by verifiers and thus cannot influence the control flow like an assertion might do in a language without static assertions.

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
> The final assertion checks that the `variant` has decreased and the new value is still within the well-founded order. However, the way these are written as nested expressions requires hoisting each subexpression up. Firstly, the `10 - i` is assigned to a temporary `#7`, and then `#8 := (< #7 #4)` computes the boolean value for decreasing which is then asserted in the terminator `!assert #8 -> :B4`. The same is done in `:B4` to assert that `#7` is non-negative.

When dealing with `struct`s, we need a way to refer to its fields, and for nested `struct`s we need to be able to refer to arbitrarily long sequences of fields. We call such accesses _places_.

![[Definition – Place]]

> [!example]
> Consider the `struct`s in the following snippet of a simplified model of humans an animals:
> ```{.mist .ignoreErrors}
> struct Human { child: Human, dog: Animal }
> struct Animal { age: int, color: Color }
> pure struct Color { r: int, g: int, b: int }
> ```
> If we have a local `Human` called `h`, then `h`, `h.dog`, `h.dog.color.r`, `h.child.dog`, and `h.child.child.dog.age` will all be potential places. In this case, the set of places will be infinite since `Human` is a recursive type.

Places will play a critical role in the following chapter, so defining some operations on them is valuable. The first we define is the notion of a _prefix_ of a place.

![[Definition – Place Prefix]]

Prefixes are particularly useful when reasoning about all places used in the process of accessing a place. In this regard, we define an ordering of places to mean that any place in the prefix of another will be smaller than that place.

![[Definition – Place partial-order]]

The final definition speaks about the related places of one place, namely those which are fields of the parent or perhaps _siblings_.

![[Definition – Place siblings]]

After the MIR has been generated, it is still subject to mutations. MIR's main objective is to enable passes that augment the CFG with additional instructions, change its structure, or perform optimizations. Additionally, CFGs are a good studies object with rich literature, [[@allenControlFlowAnalysis1970]] [[@nielsonPrinciplesProgramAnalysis1999]] used by many existing compilers, [[@midtgaardControlflowAnalysisFunctional2012]] where the most inspiration for Mists architecture is taken from rustc. [[@matsakisIntroducingMIRRust2016]] [[@RustCompilerDevelopment2018]]

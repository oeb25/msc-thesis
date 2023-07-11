---
tags: subsection
---

While snapshot testing is excellent for subsystems with loosely defined semantics, they do not alleviate the need for asserting that a system is acting as expected. For some parts of the compiler, well-defined models describe how the code must perform and what certain operations must produce. Program verification is perfect for such scenarios but can sometimes be unfeasible to apply or integrate into existing systems or more extensive subsystems.

_Property-based testing_ is a tool, quite like program verification, in that we specify the properties of functions and give an implementation for those functions and validate them against the specification. Instead of verifying them statically, we exercise them with automatically generated test cases, the other core element of property-based testing.

By specifying what the possible input for functions might look like, testing tools use strategies to generate new inputs, iteratively exploring the function domain. When the generators find a failing test case, they begin a process called _shrinking_, instead of reporting the failing input immediately. Since the tools automatically generate the test cases, they may be unnecessarily large. To make the cases easier to debug, they undergo mutations to make them smaller while ensuring that the test keeps failing as it shrinks.

The area of the compiler where we saw the most benefit from using property-based testing was during the implementation of the folding tree structure, described in detail in [[Folding tree structure]]. As we will get into in that section, folding trees satisfies many algebraic properties. By providing a way to generate arbitrary folding trees and encode their properties in property-based tests, we found _many_ bugs in the implementation. After fixing those, we were reasonably sure that the code modeled the structure correctly.

[[Figure – Proptest example]] shows how we, by the use of the Rust testing library Proptest [[@Proptest]], test that the join operator $\join$ is commutative, i.e. $\T_1 \join \T_2 = \T_2 \join \T_1$. The first part of the example contains a `prop_compose!` definition `@arb_folding_tree` $\lineref{1-12}$ of how to construct arbitrary folding trees from a collection of arbitrary places. The second part is the property test `@folding_tree_join_commute` $\lineref{13-24}$, which generates two arbitrary folding trees using `@arb_folding_tree` $\lineref{16, 17}$, and checks that $\join$ commutes $\lineref{22}$.

![[Figure – Proptest example]]

%%- Amazon uses property-based testing for lightweight verification of the implementation of a model [[@Bornholt2021#Section~4]] using property-based testing [[@Proptest]].%%

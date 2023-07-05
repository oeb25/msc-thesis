---
tags: subsection
---

By the end of [[Computing solutions]], we had computed folding states for all program points and even augmented the programs with foldings. The final folding states in the analysis will be those foldings naturally emerging from arbitrary executions of the program. The inserted folds and unfolds will ensure that when discrepancies between the existing state of one instruction do not match the required state to execute another instruction, it will be made to arrive at precisely the expected folding state.

![[Theorem – Folding analysis results in well-defined programs]]

> [!proof]
> With [[Lemma – FIR Abstract Semantics compute well-defined access]], we showed that any folding tree given by transforming over an instruction will make executions of that instruction well-defined. Similarly, by [[Proposition – Monotone the computes greatest solution]], we have computed the greatest solution before these instructions. The greatest solution is derived by performing a sequence of $\meet$, and by [[Lemma – Meet of well-defined is well-defined]], we know that the resulting folding states in $\A$ must make execution well-defined.

In conclusion, by performing folding analysis on the FIR projection of a program and augmenting that program with folds and unfolds between adjacent program points, _all instruction executions will be well-defined in terms of folding access_.

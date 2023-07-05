---
tags: subsection
---

Having defined the semantics for individual instructions, we want to reason about the interaction of sequences of instructions in the program. As FIR programs can be branching, the executions through the CFG might not be linear, and even be cyclic. The goal of the folding analysis is to determine what foldings are required at each step of any execution.

For linear segments of the program, we can look at the requirements of one instruction, and then transform them _going backwards_ through the preceding instructions, altering the requirements as they change for each instruction. If the control flow branches, we look at the requirements for each branch, and propagate that backwards through the CFG.

Doing the backwards propagation computes the least restrictive folding state at each program point immediately before any instruction. In many ways this is similar to liveness analysis [[@ahoCompilersPrinciplesTechniques1986#p. 608]] which determines which variables need to be live at each program point.

Computing the requirements involves transforming one folding tree into one which satisfies the requirements of some instruction, while performing minimal changes to that tree. We call this operation the _abstract-semantics_ for folding-analysis.

![[Definition – FIR Abstract Semantics]]

The main goal of this function, is to compute folding trees satisfying the requirements of executing any instruction.

![[Lemma – FIR Abstract Semantics compute well-defined access]]

![[Proof – FIR Abstract Semantics compute well-defined access]]

![[Lemma – FIR Abstract Semantics is a monotone function]]

![[Proof – FIR Abstract Semantics is a monotone function]]

With $\bsems$ defined, we can begin to reason about complete programs, by applying it to the full FIR.

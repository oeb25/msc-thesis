---
tags: subsection
---

To reason about how a program changes as instructions are executed, we must define the _semantics_ of the instructions. Semantics describe how a program goes from on program state to another. Program states in the context of FIR is the folding states of places, but also _memories_, which are snapshots of values places take at distinct points in the execution.

![[Definition – FIR Memory]]

> [!remark]
> As folding analysis is not concerned with concrete values, the details of $\Value$'s definition will be left undefined for brevity, but to summarise, these will be integers, booleans, lists of values, potentially recursive structs, and so on. For the same reasons, the details of $\eval$ is also left undefined.

Memories make up the values of executions, but they do not store any information about which places are folded or not. Thus when analysing foldings, we associate each memory with a folding tree, together forming a program state.

![[Definition – FIR Program State]]

With program states defined, we need to look at how they change as instructions are performed, defined by the semantics of those instructions.

![[Definition – FIR Semantics]]

To execute as an instruction, certain things must be true about the program state specifically for that instruction. Firstly, all places read must be part of the domain of memory. Secondly, the instruction must be well-defined in terms of folding access as per [[Definition – FIR well-defined access rules]]. Iff both these requirements hold, then we can compute the next program state.

![[Definition – FIR Semantics inference rules]]

![[Example – SubFib Semantics]]

The semantics are the foundation for reasoning about FIR programs, as they both describe what is necessary for executing an instruction and what the state of the program will be after that execution. The goal of folding analysis, is to ensure that any execution of the program after analysis meets the requirements at each step of the execution.

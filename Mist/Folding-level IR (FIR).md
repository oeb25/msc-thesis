---
tags: subsection
---

In [[Compilation stages]], we introduced the multi-stage structure of the Mist compiler, and specifically the MIR representation (see [[Mid-level IR (MIR)]]). In terms of computing foldings and unfoldings, this is the stage we are considering. However, instead of considering the complete set of instructions and terminators, we consider a smaller variant that focuses much more concretely on _how and what_ places are used and omit the details for performing actual computation or verification.

![[Figure – FIR Grammar]]

![[Definition – FIR]]

Although FIR is defined as a separate set of constructs from MIR, it is, in fact, entirely derivable from MIR, leading it to be a projection. The details from MIR that folding analysis is concerned with are read and written places in instructions and terminators.

FIR consists of five abstracts instructions, the first three instructions being $:=$, `reference`, and `mention`. %%, as well as `read` to extract the places read in an expression.%%

The remaining two instructions, `fold` and `unfold`, serve a similar purpose as those defined in [[Definition – Folding]] but has _no impact on the execution of the program_ and are used solely during folding analysis.

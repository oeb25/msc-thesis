---
tags: subsection
---

In [[Compilation stages]] we introduced the multi-stage structure of the Mist compiler, and specifically the MIR representation which is a CFG. In terms of computing foldings and unfoldings, this is the stage we are concerned with, but instead of considering the full set of instructions and terminators, we consider a smaller variant that focuses much more concretely on _how and what_ places are used and omit the details for performing actual computation or verification. The grammar for this smaller language is shown in [[Figure – FIR Grammar]].

![[Figure – FIR Grammar]]

---
tags: definition
---

We define the Folding-level IR (FIR) as a projection of MIR that encodes places written to and places read from into a sequence of instructions.

FIR comprises blocks of FIR instructions, connected by unconditional (potentially branching and cyclic) jumps terminating the blocks.

The grammar for FIR is defined in [[Figure – FIR Grammar]].
